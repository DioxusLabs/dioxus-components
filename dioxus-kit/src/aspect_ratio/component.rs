use dioxus::prelude::*;
use dioxus_kit_core::aspect_ratio::AspectRatioProps;

#[component]
pub fn AspectRatio(props: AspectRatioProps) -> Element {
    dioxus_kit_core::aspect_ratio::AspectRatio(props)
}
