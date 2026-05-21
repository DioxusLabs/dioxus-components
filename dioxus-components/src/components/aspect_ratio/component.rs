use dioxus::prelude::*;
use dioxus_primitives::aspect_ratio::AspectRatioProps;

#[component]
pub fn AspectRatio(props: AspectRatioProps) -> Element {
    dioxus_primitives::aspect_ratio::AspectRatio(props)
}
