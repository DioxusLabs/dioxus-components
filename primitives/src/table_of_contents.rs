//! Defines the [`TableOfContents`] component.

use dioxus::prelude::*;

use crate::scroll_spy::{use_scroll_spy, ScrollSpyData, ScrollSpyOptions};

/// Props for the [`TableOfContents`] component.
#[derive(Props, Clone, PartialEq)]
pub struct TableOfContentsProps {
    /// Options passed to [`use_scroll_spy`].
    #[props(default)]
    pub scroll_spy_options: ScrollSpyOptions,

    /// Data rendered before browser-side heading discovery completes.
    #[props(default)]
    pub initial_data: Vec<ScrollSpyData>,

    /// Minimum heading depth before indentation starts.
    #[props(default = 1)]
    pub min_depth_to_offset: u8,

    /// CSS length multiplied by each heading depth level.
    #[props(default = "20px".to_string())]
    pub depth_offset: String,

    /// Additional attributes to apply to the table-of-contents root element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// # TableOfContents
///
/// The `TableOfContents` component renders anchors for headings discovered by
/// [`use_scroll_spy`]. The active anchor receives `data-active="true"`.
#[component]
pub fn TableOfContents(props: TableOfContentsProps) -> Element {
    let mut options = props.scroll_spy_options.clone();
    if options.initial_data.is_empty() {
        options.initial_data = props.initial_data.clone();
    }

    let spy = use_scroll_spy(options);
    let min_depth = props.min_depth_to_offset;
    let depth_offset = props.depth_offset.clone();

    rsx! {
        nav {
            "data-table-of-contents": "true",
            ..props.attributes,
            for (index, item) in (spy.data)().into_iter().enumerate() {
                TableOfContentsControl {
                    key: "{item.id}",
                    item,
                    active: (spy.active)() == Some(index),
                    min_depth,
                    depth_offset: depth_offset.clone(),
                }
            }
        }
    }
}

#[component]
fn TableOfContentsControl(
    item: ScrollSpyData,
    active: bool,
    min_depth: u8,
    depth_offset: String,
) -> Element {
    let depth = item.depth.saturating_sub(min_depth);
    let style = format!("--depth: {depth}; --depth-offset: {depth_offset};");

    rsx! {
        a {
            href: "#{item.id}",
            style,
            "data-active": if active { "true" } else { "false" },
            "data-depth": "{item.depth}",
            "{item.value}"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table_of_contents_renders_initial_data_on_server() {
        let rendered = dioxus_ssr::render_element(rsx! {
            TableOfContents {
                initial_data: vec![
                    ScrollSpyData {
                        id: "intro".to_string(),
                        value: "Intro".to_string(),
                        depth: 1,
                    },
                    ScrollSpyData {
                        id: "details".to_string(),
                        value: "Details".to_string(),
                        depth: 2,
                    },
                ],
            }
        });

        assert!(rendered.contains("href=\"#intro\""));
        assert!(rendered.contains("Intro"));
        assert!(rendered.contains("href=\"#details\""));
        assert!(rendered.contains("Details"));
    }
}
