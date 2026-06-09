use dioxus::prelude::*;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsCast;

#[wasm_bindgen(inline_js = r#"
export function install_focus_listener(callback) {
    globalThis.document?.addEventListener("focusin", callback, true);
}

export function focus_soon(id) {
    setTimeout(() => {
        globalThis.document?.getElementById(id)?.focus();
    }, 100);
}
"#)]
extern "C" {
    fn install_focus_listener(callback: &js_sys::Function);
    fn focus_soon(id: &str);
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let _listener: Rc<Closure<dyn FnMut(&web_sys::Event)>> = use_hook(|| {
        let callback =
            Closure::<dyn FnMut(&web_sys::Event)>::new(move |_event: &web_sys::Event| {
                eprintln!("rust focusin listener callback running");

                // This panics because this raw wasm-bindgen-x callback is not
                // running inside a Dioxus scope, even though the callback was
                // created by a hook in a Dioxus component.
                spawn(async {
                    eprintln!("spawned task");
                });
            });

        install_focus_listener(callback.as_ref().unchecked_ref());
        focus_soon("target");

        Rc::new(callback)
    });

    rsx! {
        main {
            style: "font-family: system-ui; padding: 24px;",
            h1 { "wry-bindgen event reentry MRE" }
            p {
                "This Dioxus desktop app calls dioxus::spawn from a raw wasm-bindgen-x focusin callback."
            }
            p {
                "It should either spawn the task or return an error. On the affected stack it panics because there is no current Dioxus scope."
            }
            button {
                id: "target",
                "Focus target"
            }
        }
    }
}
