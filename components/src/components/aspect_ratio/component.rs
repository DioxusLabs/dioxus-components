use dioxus::prelude::*;
use dioxus_primitives::aspect_ratio::AspectRatioProps;

#[css_module("/src/components/aspect_ratio/style.css")]
pub struct Styles;

#[component]
pub fn AspectRatio(props: AspectRatioProps) -> Element {
    dioxus_primitives::aspect_ratio::AspectRatio(props)
}
