use std::future::Future;
use std::rc::Rc;
use std::time::Duration;
#[cfg(target_arch = "wasm32")]
use std::{
    cell::{Cell, RefCell},
    pin::Pin,
    sync::atomic::{AtomicU32, Ordering},
    task::{Context, Poll, Waker},
};

use dioxus::core::{Runtime, RuntimeGuard, ScopeId, Task};
use dioxus::prelude::{spawn, use_hook};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::closure::Closure;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(inline_js = r#"
const timeouts = globalThis.__dxPrimitivesTimeouts ??= new Map();

export function dx_set_timeout(key, callback, ms) {
    dx_clear_timeout(key);

    const handle = setTimeout(() => {
        timeouts.delete(key);
        callback();
    }, ms);

    timeouts.set(key, handle);
}

export function dx_clear_timeout(key) {
    const handle = timeouts.get(key);
    if (handle !== undefined) {
        clearTimeout(handle);
        timeouts.delete(key);
    }
}
"#)]
extern "C" {
    fn dx_set_timeout(key: u32, callback: &js_sys::Function, milliseconds: f64);
    fn dx_clear_timeout(key: u32);
}

#[cfg(target_arch = "wasm32")]
static NEXT_TIMEOUT_ID: AtomicU32 = AtomicU32::new(1);

/// Pause the current task using the browser/WebView timer runtime.
#[cfg(not(target_arch = "wasm32"))]
pub async fn sleep(duration: Duration) {
    tokio::time::sleep(duration).await;
}

/// Pause the current task using the browser/WebView timer runtime.
#[cfg(target_arch = "wasm32")]
pub async fn sleep(duration: Duration) {
    Sleep::new(duration).await;
}

/// A Dioxus runtime and scope captured from a component.
#[derive(Clone)]
pub struct DioxusTaskSpawner {
    runtime: Rc<Runtime>,
    scope: ScopeId,
}

impl DioxusTaskSpawner {
    /// Run a closure in the captured Dioxus scope.
    pub fn run<O>(&self, callback: impl FnOnce() -> O) -> O {
        self.runtime.in_scope(self.scope, callback)
    }

    /// Spawn a task in the captured Dioxus scope.
    pub fn spawn(&self, task: impl Future<Output = ()> + 'static) -> Task {
        self.run(|| spawn(task))
    }
}

/// Capture the current Dioxus runtime and scope for later task spawning.
pub fn use_task_spawner() -> DioxusTaskSpawner {
    use_hook(|| {
        let runtime = Runtime::current();
        let scope = runtime.current_scope_id();

        DioxusTaskSpawner { runtime, scope }
    })
}

/// A cancellable timeout task.
pub struct Timeout {
    #[cfg(not(target_arch = "wasm32"))]
    runtime: Rc<Runtime>,
    #[cfg(not(target_arch = "wasm32"))]
    task: Option<Task>,
    #[cfg(target_arch = "wasm32")]
    key: u32,
    #[cfg(target_arch = "wasm32")]
    _callback: Closure<dyn FnMut()>,
}

impl Timeout {
    /// Run `callback` after `milliseconds`.
    pub fn new(
        spawner: DioxusTaskSpawner,
        milliseconds: u32,
        callback: impl FnOnce() + 'static,
    ) -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let runtime = spawner.runtime.clone();
            let task = spawner.spawn(async move {
                sleep(Duration::from_millis(milliseconds as u64)).await;
                callback();
            });

            Self {
                runtime,
                task: Some(task),
            }
        }

        #[cfg(target_arch = "wasm32")]
        {
            let key = next_timeout_key();
            let mut callback = Some(callback);
            let callback_spawner = spawner.clone();
            let closure = Closure::<dyn FnMut()>::new(move || {
                if let Some(callback) = callback.take() {
                    callback_spawner.run(callback);
                }
            });

            dx_set_timeout(key, closure.as_ref().unchecked_ref(), milliseconds as f64);

            Self {
                key,
                _callback: closure,
            }
        }
    }
}

impl Drop for Timeout {
    fn drop(&mut self) {
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(task) = self.task.take() {
            let _runtime = RuntimeGuard::new(self.runtime.clone());
            task.cancel();
        }

        #[cfg(target_arch = "wasm32")]
        dx_clear_timeout(self.key);
    }
}

#[cfg(target_arch = "wasm32")]
struct Sleep {
    key: u32,
    state: Rc<SleepState>,
    _callback: Closure<dyn FnMut()>,
}

#[cfg(target_arch = "wasm32")]
struct SleepState {
    complete: Cell<bool>,
    waker: RefCell<Option<Waker>>,
}

#[cfg(target_arch = "wasm32")]
impl Sleep {
    fn new(duration: Duration) -> Self {
        let key = next_timeout_key();
        let state = Rc::new(SleepState {
            complete: Cell::new(false),
            waker: RefCell::new(None),
        });
        let callback_state = state.clone();
        let callback = Closure::<dyn FnMut()>::new(move || {
            callback_state.complete.set(true);
            if let Some(waker) = callback_state.waker.borrow_mut().take() {
                waker.wake();
            }
        });

        dx_set_timeout(
            key,
            callback.as_ref().unchecked_ref(),
            duration.as_millis() as f64,
        );

        Self {
            key,
            state,
            _callback: callback,
        }
    }
}

#[cfg(target_arch = "wasm32")]
impl Future for Sleep {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.state.complete.get() {
            Poll::Ready(())
        } else {
            *self.state.waker.borrow_mut() = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

#[cfg(target_arch = "wasm32")]
impl Drop for Sleep {
    fn drop(&mut self) {
        dx_clear_timeout(self.key);
    }
}

#[cfg(target_arch = "wasm32")]
fn next_timeout_key() -> u32 {
    NEXT_TIMEOUT_ID.fetch_add(1, Ordering::Relaxed)
}
