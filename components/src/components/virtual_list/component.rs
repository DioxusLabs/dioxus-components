use dioxus::prelude::*;
pub use dioxus_primitives::virtual_list::VirtualListProps;
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

/// Styled wrapper around the primitive `VirtualList`.
#[component]
pub fn VirtualList(props: VirtualListProps) -> Element {
    let base = attributes!(div {
        class: "dx-virtual-list-container"
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        dioxus_primitives::virtual_list::VirtualList {
            count: props.count,
            buffer: props.buffer,
            estimate_size: props.estimate_size,
            render_item: props.render_item,
            attributes: merged,
        }
    }
}
