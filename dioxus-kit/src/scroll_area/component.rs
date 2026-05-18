use dioxus::prelude::*;
use dioxus_kit_core::scroll_area::{self, ScrollAreaProps};

#[component]
pub fn ScrollArea(props: ScrollAreaProps) -> Element {
    scroll_area::ScrollArea(props)
}
