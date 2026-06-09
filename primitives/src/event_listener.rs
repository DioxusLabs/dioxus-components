use std::borrow::Cow;

use crate::runtime_closure::event_closure_with_runtime;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{AddEventListenerOptions, Event, EventTarget};

/// Specifies whether the event listener runs during capture or bubble.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventListenerPhase {
    /// Bubble phase.
    #[default]
    Bubble,
    /// Capture phase.
    Capture,
}

impl EventListenerPhase {
    fn is_capture(self) -> bool {
        matches!(self, Self::Capture)
    }
}

/// Options for a DOM event listener.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EventListenerOptions {
    /// The phase that the event listener should run in.
    pub phase: EventListenerPhase,
    /// Whether the listener is passive.
    pub passive: bool,
}

impl EventListenerOptions {
    /// Runs the event listener in the capture phase.
    pub fn run_in_capture_phase() -> Self {
        Self {
            phase: EventListenerPhase::Capture,
            ..Self::default()
        }
    }

    /// Allows the listener callback to call `prevent_default`.
    pub fn enable_prevent_default() -> Self {
        Self {
            passive: false,
            ..Self::default()
        }
    }

    fn as_js(self) -> AddEventListenerOptions {
        let options = AddEventListenerOptions::new();
        options.set_capture(self.phase.is_capture());
        options.set_passive(self.passive);
        options
    }
}

impl Default for EventListenerOptions {
    fn default() -> Self {
        Self {
            phase: EventListenerPhase::Bubble,
            passive: true,
        }
    }
}

/// RAII DOM event listener backed by `wasm-bindgen-x`/`web-sys-x`.
#[must_use = "event listener will be removed when dropped"]
pub struct EventListener {
    target: EventTarget,
    event_type: Cow<'static, str>,
    callback: Option<Closure<dyn FnMut(&Event)>>,
    phase: EventListenerPhase,
}

impl EventListener {
    /// Registers an event listener on an event target.
    pub fn new<S, F>(target: &EventTarget, event_type: S, callback: F) -> Self
    where
        S: Into<Cow<'static, str>>,
        F: FnMut(&Event) + 'static,
    {
        Self::new_with_options(
            target,
            event_type,
            EventListenerOptions::default(),
            callback,
        )
    }

    /// Registers an event listener with explicit options.
    pub fn new_with_options<S, F>(
        target: &EventTarget,
        event_type: S,
        options: EventListenerOptions,
        callback: F,
    ) -> Self
    where
        S: Into<Cow<'static, str>>,
        F: FnMut(&Event) + 'static,
    {
        let event_type = event_type.into();
        let callback = event_closure_with_runtime(callback);
        let js_options = options.as_js();

        target
            .add_event_listener_with_callback_and_add_event_listener_options(
                &event_type,
                callback.as_ref().unchecked_ref(),
                &js_options,
            )
            .expect("failed to add DOM event listener");

        Self {
            target: target.clone(),
            event_type,
            callback: Some(callback),
            phase: options.phase,
        }
    }
}

impl Drop for EventListener {
    fn drop(&mut self) {
        if let Some(callback) = &self.callback {
            let _ = self.target.remove_event_listener_with_callback_and_bool(
                &self.event_type,
                callback.as_ref().unchecked_ref(),
                self.phase.is_capture(),
            );
        }
    }
}
