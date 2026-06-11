use super::super::component::*;
use dioxus::prelude::*;
use dioxus_primitives::scroll_spy::{ScrollSpyOptions, ScrollSpyScrollHost};

#[component]
pub fn Demo() -> Element {
    let scroll_spy_options = ScrollSpyOptions {
        selector: "article :is(h2, h3, h4)".to_string(),
        scroll_host: ScrollSpyScrollHost::Selector("[data-toc-demo-scroll-region]".to_string()),
        offset: 88.0,
        ..Default::default()
    };

    rsx! {
        div {
            "data-toc-demo-scroll-region": "true",
            display: "grid",
            grid_template_columns: "minmax(0, 1fr) 16rem",
            gap: "2rem",
            align_items: "start",
            max_height: "40rem",
            overflow_y: "auto",
            padding: "2rem",
            border: "1px solid var(--primary-color-6)",
            border_radius: "1rem",
            background: "var(--primary-color-2)",
            color: "var(--secondary-color-1)",

            article {
                max_width: "44rem",
                display: "flex",
                flex_direction: "column",
                gap: "2.5rem",
                padding_bottom: "12rem",

                section {
                    display: "flex",
                    flex_direction: "column",
                    gap: "1rem",
                    h2 { id: "overview", "Overview" }
                    p { "The table of contents tracks headings in this document and updates the active link while the scroll container moves through long-form content." }
                    p { "This preview keeps the navigation pinned in view so you can verify that the highlighted entry changes as each heading crosses the configured offset." }
                }

                section {
                    display: "flex",
                    flex_direction: "column",
                    gap: "1rem",
                    h2 { id: "installation", "Installation" }
                    p { "Render the table of contents beside the article and provide initial heading data so server rendering and the hydrated client show the same navigation structure." }
                    p { "The preview intentionally includes enough copy to force scrolling, making it easier to validate active heading transitions instead of relying on a static layout." }

                    div {
                        display: "flex",
                        flex_direction: "column",
                        gap: "1rem",
                        padding_left: "1.5rem",
                        border_left: "1px solid var(--primary-color-6)",
                        h3 { id: "configuration", "Configuration" }
                        p { "Use the selector to scope which headings participate and choose a scroll host when the document uses an internal panel instead of the browser window." }
                        p { "Indentation in the rendered list reflects heading depth, so a mix of h2, h3, and h4 entries is useful when checking hierarchy." }

                        div {
                            display: "flex",
                            flex_direction: "column",
                            gap: "1rem",
                            padding_left: "1.25rem",
                            border_left: "1px solid var(--primary-color-6)",
                            h4 { id: "offsets", "Offsets" }
                            p { "Offset tuning decides when a heading becomes active. In this demo the active item flips before the heading reaches the top edge, which keeps the label change readable during slower scrolling." }
                        }
                    }
                }

                section {
                    display: "flex",
                    flex_direction: "column",
                    gap: "1rem",
                    h2 { id: "api", "API" }
                    p { "The primitive exposes initial data, selector configuration, a scroll host, and a reinitialize callback for dynamic documents that add or remove headings after first render." }
                    p { "The active state is still index-based in the core primitive. This preview only improves the surrounding layout and visual feedback." }

                    div {
                        display: "flex",
                        flex_direction: "column",
                        gap: "1rem",
                        padding_left: "1.5rem",
                        border_left: "1px solid var(--primary-color-6)",
                        h3 { id: "reinitialization", "Reinitialization" }
                        p { "Call reinitialize after heading content changes so the hook can rescan the document and keep the table of contents aligned with the rendered article." }
                    }
                }

                section {
                    display: "flex",
                    flex_direction: "column",
                    gap: "1rem",
                    h2 { id: "styling", "Styling" }
                    p { "Inactive items should stay readable but subdued. The active item needs stronger contrast, a clearer accent, and preserved indentation so the current heading stands out immediately." }
                    p { "Keeping the navigation sticky inside the scroll region lets the preview demonstrate both hierarchy and scroll-spy feedback in one compact example." }

                    div {
                        display: "flex",
                        flex_direction: "column",
                        gap: "1rem",
                        padding_left: "1.5rem",
                        border_left: "1px solid var(--primary-color-6)",
                        h3 { id: "accessibility", "Accessibility" }
                        p { "Consistent heading order and stable ids help keyboard users and assistive technology users move between the article and its generated navigation." }
                    }
                }

                section {
                    display: "flex",
                    flex_direction: "column",
                    gap: "1rem",
                    h2 { id: "usage-notes", "Usage Notes" }
                    p { "Scroll through this panel to watch the highlighted entry move from section to section. The demo now includes enough vertical space for the active item to change several times before the end of the document." }
                    p { "If you swap in your own content, keep a similar amount of spacing and section depth so the preview continues to exercise the component meaningfully." }
                }
            }

            aside {
                position: "sticky",
                top: "1.5rem",
                TableOfContents {
                    scroll_spy_options,
                }
            }
        }
    }
}
