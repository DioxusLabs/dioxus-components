//! Typed bindings for the focus trap JavaScript library.

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
extern "C" {
    /// A focus trap instance that keeps focus within a container element.
    pub type FocusTrap;

    /// Creates a new focus trap for the given container element.
    /// This calls `window.createFocusTrap(container)` defined in focus-trap.js.
    #[wasm_bindgen(js_namespace = window, js_name = createFocusTrap)]
    pub fn create_focus_trap(container: &web_sys::HtmlElement) -> FocusTrap;

    /// Removes the focus trap, restoring normal focus behavior.
    #[wasm_bindgen(method)]
    pub fn remove(this: &FocusTrap);
}

/// Sets up or tears down a focus trap for the given element.
///
/// When `open` is true, creates a new focus trap. When `open` is false,
/// removes any existing focus trap.
pub fn setup_focus_trap(id: &str, open: bool, trap: &mut Option<FocusTrap>) {
    let Some(window) = web_sys::window() else {
        return;
    };
    let Some(document) = window.document() else {
        return;
    };
    let Some(element) = document.get_element_by_id(id) else {
        return;
    };
    let Ok(html_el) = element.dyn_into::<web_sys::HtmlElement>() else {
        return;
    };

    if open {
        *trap = Some(create_focus_trap(&html_el));
    } else if let Some(t) = trap.take() {
        t.remove();
    }
}
