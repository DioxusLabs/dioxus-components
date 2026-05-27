use super::super::component::*;
use crate::components::label::Label;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut description = use_signal(|| "sample text".to_string());
    rsx! {
        div {
            display: "flex",
            flex_direction: "column",
            gap: "1.5rem",

            p { id: "textarea-message", "Description here: {description}" }

            div {
                display: "flex",
                flex_direction: "column",
                gap: ".5rem",
                justify_content: "center",
                Label { html_for: "outline", "Outline" }
                Textarea {
                    id: "outline",
                    variant: TextareaVariant::Outline,
                    placeholder: "Enter your description",
                    value: description,
                    oninput: move |e: FormEvent| description.set(e.value()),
                }
            }
        }
    }
}
