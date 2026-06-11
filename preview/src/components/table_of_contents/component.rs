use dioxus::prelude::*;
use dioxus_primitives::table_of_contents::{self, TableOfContentsProps};

#[css_module("/src/components/table_of_contents/style.css")]
struct Styles;

#[component]
pub fn TableOfContents(props: TableOfContentsProps) -> Element {
    rsx! {
        table_of_contents::TableOfContents {
            class: Styles::dx_table_of_contents,
            scroll_spy_options: props.scroll_spy_options,
            initial_data: props.initial_data,
            min_depth_to_offset: props.min_depth_to_offset,
            depth_offset: props.depth_offset,
            attributes: props.attributes,
        }
    }
}
