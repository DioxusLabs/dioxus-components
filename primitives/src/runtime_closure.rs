use std::rc::Rc;

use dioxus::core::{Runtime, RuntimeGuard};
use wasm_bindgen::closure::Closure;
use web_sys::Event;

#[derive(Clone)]
pub(crate) struct RuntimeContext {
    runtime: Option<Rc<Runtime>>,
}

impl RuntimeContext {
    pub(crate) fn current() -> Self {
        Self {
            runtime: Runtime::try_current(),
        }
    }

    pub(crate) fn run<O>(&self, callback: impl FnOnce() -> O) -> O {
        match &self.runtime {
            Some(runtime) => {
                let _guard = RuntimeGuard::new(runtime.clone());
                callback()
            }
            None => callback(),
        }
    }
}

pub(crate) fn closure_with_runtime(mut callback: impl FnMut() + 'static) -> Closure<dyn FnMut()> {
    let runtime = RuntimeContext::current();
    Closure::wrap(Box::new(move || runtime.run(|| callback())) as Box<dyn FnMut()>)
}

pub(crate) fn event_closure_with_runtime(
    mut callback: impl FnMut(&Event) + 'static,
) -> Closure<dyn FnMut(&Event)> {
    let runtime = RuntimeContext::current();
    Closure::wrap(
        Box::new(move |event: &Event| runtime.run(|| callback(event))) as Box<dyn FnMut(&Event)>,
    )
}
