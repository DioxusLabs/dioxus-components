use crate::components::button::component::Button;

use super::super::component::*;
use dioxus::prelude::*;
use dioxus_kit_core::toast::{use_toast, ToastOptions};
use std::time::Duration;

#[component]
pub fn Demo() -> Element {
    rsx! {
        ToastProvider { ToastButton {} }
    }
}

#[component]
fn ToastButton() -> Element {
    let toast_api = use_toast();

    rsx! {
        Button {
            r#type: "button",
            "data-style": "outline",
            onclick: move |_| {
                toast_api
                    .info(
                        "Custom Toast".to_string(),
                        ToastOptions::new()
                            .description("Some info you need")
                            .duration(Duration::from_secs(60))
                            .permanent(false),
                    );
            },
            "Info (60s)"
        }
    }
}
