use super::super::component::AspectRatio;
use dioxus::prelude::*;

#[css_module("/src/components/aspect_ratio/style.css")]
struct Styles;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div {
            class: Styles::dx_aspect_ratio_container,
            width: "20rem",
            max_width: "30vw",
            AspectRatio { ratio: 4.0 / 3.0,
                div {
                    background: "linear-gradient(to bottom right, var(--primary-color-5), var(--primary-color-3))",
                    width: "100%",
                    height: "100%",
                }
            }
        }
    }
}
