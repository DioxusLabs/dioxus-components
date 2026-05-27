use super::super::component::*;
use dioxus::prelude::*;

#[component]
fn ToggleToolbarButton(
    index: usize,
    is_on: bool,
    on_click: Callback<()>,
    children: Element,
) -> Element {
    rsx! {
        ToolbarButton {
            index,
            on_click,
            "data-state": if is_on { "on" } else { "off" },
            background: if is_on { "var(--light, var(--primary-color-5)) var(--dark, var(--primary-color-6))" } else { "" },
            color: if is_on { "var(--secondary-color-1)" } else { "" },
            {children}
        }
    }
}

#[component]
pub fn Demo() -> Element {
    let mut is_bold = use_signal(|| false);
    let mut is_italic = use_signal(|| false);
    let mut is_underline = use_signal(|| false);
    let mut text_align = use_signal(|| "left".to_string());

    rsx! {
        div { class: "dx-toolbar-demo",
            Toolbar { aria_label: "Text formatting",
                ToolbarGroup {
                    ToggleToolbarButton {
                        index: 0usize,
                        is_on: is_bold(),
                        on_click: move |_| is_bold.toggle(),
                        "Bold"
                    }
                    ToggleToolbarButton {
                        index: 1usize,
                        is_on: is_italic(),
                        on_click: move |_| is_italic.toggle(),
                        "Italic"
                    }
                    ToggleToolbarButton {
                        index: 2usize,
                        is_on: is_underline(),
                        on_click: move |_| is_underline.toggle(),
                        "Underline"
                    }
                }
                ToolbarSeparator {}
                ToolbarGroup {
                    ToggleToolbarButton {
                        index: 3usize,
                        is_on: text_align() == "left",
                        on_click: move |_| text_align.set("left".to_string()),
                        "Align Left"
                    }
                    ToggleToolbarButton {
                        index: 4usize,
                        is_on: text_align() == "center",
                        on_click: move |_| text_align.set("center".to_string()),
                        "Align Center"
                    }
                    ToggleToolbarButton {
                        index: 5usize,
                        is_on: text_align() == "right",
                        on_click: move |_| text_align.set("right".to_string()),
                        "Align Right"
                    }
                }
            }
            p {
                max_width: "30rem",
                text_align: "{text_align}",
                font_weight: if is_bold() { "bold" } else { "normal" },
                font_style: if is_italic() { "italic" } else { "normal" },
                text_decoration: if is_underline() { "underline" } else { "none" },
                "This is a sample text that will be formatted according to the toolbar buttons you click. Try clicking the buttons above to see how the text formatting changes."
            }
        }
    }
}
