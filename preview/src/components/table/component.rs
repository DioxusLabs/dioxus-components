use dioxus::prelude::*;

#[component]
pub fn DemoWithStyles() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        super::variants::main::Demo {}
    }
}