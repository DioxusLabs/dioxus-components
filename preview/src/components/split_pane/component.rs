use dioxus::prelude::*;
use dioxus_primitives::split_pane::{self, PaneProps, SplitPaneDividerProps, SplitPaneProps};
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};
use std::sync::atomic::{AtomicUsize, Ordering};

pub use dioxus_primitives::split_pane::{
    use_split_pane_persistence, SplitPaneDirection, SplitPaneResizeEvent, SplitPaneSize,
    SplitPaneStorage,
};

#[css_module("/src/components/split_pane/style.css")]
struct Styles;

const DEFAULT_DIVIDER_SIZE: f64 = 6.0;
static SPLIT_PANE_ID: AtomicUsize = AtomicUsize::new(0);

/// Styled wrapper around the primitive split pane root.
#[component]
pub fn SplitPane(props: SplitPaneProps) -> Element {
    let measurement_id = use_hook(|| SPLIT_PANE_ID.fetch_add(1, Ordering::Relaxed));
    let mut measured_divider_size = use_signal(|| DEFAULT_DIVIDER_SIZE);
    let base = attributes!(div {
        class: Styles::dx_split_pane,
        "data-split-pane-id": "{measurement_id}"
    });
    let merged = merge_attributes(vec![base, props.attributes]);
    let divider_class = (props.divider_class)()
        .or_else(|| Some(Styles::dx_split_pane_divider.to_string()));
    let divider_size = use_memo(move || {
        let size = (props.divider_size)();
        if size > 0.0 {
            size
        } else {
            measured_divider_size().max(0.0)
        }
    });
    let divider_style = use_memo(move || {
        let style = (props.divider_style)();
        if (props.divider_size)() > 0.0 {
            style
        } else {
            let inferred_style = "flex:0 0 var(--split-pane-divider-size);";
            Some(match style {
                Some(style) => format!("{inferred_style}{style}"),
                None => inferred_style.to_string(),
            })
        }
    });

    use_effect(move || {
        spawn(async move {
            let script = format!(
                r#"
                const id = "{measurement_id}";
                const key = `__dxSplitPaneDividerSize_${{id}}`;

                if (window[key]?.cleanup) {{
                    window[key].cleanup();
                }}

                const root = document.querySelector(`[data-split-pane-id="${{id}}"]`);
                if (!root) {{
                    dioxus.send(null);
                    return;
                }}

                let frame = 0;
                window[key] = {{
                    divider: null,
                    cleanup: null
                }};

                function parsePx(value) {{
                    const size = Number.parseFloat(value);
                    return Number.isFinite(size) ? size : 0;
                }}

                function currentDivider() {{
                    return root.querySelector('[role="separator"]');
                }}

                function observeDivider() {{
                    const divider = currentDivider();
                    if (divider && window[key]?.divider !== divider) {{
                        if (window[key]?.divider) {{
                            resizeObserver.unobserve(window[key].divider);
                        }}
                        resizeObserver.observe(divider);
                        window[key].divider = divider;
                    }}
                    return divider;
                }}

                function dividerSize() {{
                    const divider = observeDivider();
                    if (!divider) {{
                        return 0;
                    }}

                    const axis = root.dataset.orientation === "vertical" ? "height" : "width";
                    const rect = divider.getBoundingClientRect();
                    const measured = axis === "width" ? rect.width : rect.height;
                    if (measured > 0) {{
                        return measured;
                    }}

                    const dividerStyles = getComputedStyle(divider);
                    const flexBasis = parsePx(dividerStyles.flexBasis);
                    if (flexBasis > 0) {{
                        return flexBasis;
                    }}

                    return parsePx(getComputedStyle(root).getPropertyValue("--split-pane-divider-size"));
                }}

                function publish() {{
                    cancelAnimationFrame(frame);
                    frame = requestAnimationFrame(() => {{
                        const size = dividerSize();
                        if (size > 0) {{
                            dioxus.send(size);
                        }}
                    }});
                }}

                const resizeObserver = new ResizeObserver(publish);
                resizeObserver.observe(root);

                const mutationObserver = new MutationObserver(publish);
                mutationObserver.observe(root, {{
                    attributes: true,
                    childList: true,
                    subtree: true,
                    attributeFilter: ["class", "style", "data-orientation"]
                }});
                mutationObserver.observe(document.documentElement, {{
                    attributes: true,
                    attributeFilter: ["class", "style", "data-theme"]
                }});
                if (document.body) {{
                    mutationObserver.observe(document.body, {{
                        attributes: true,
                        attributeFilter: ["class", "style", "data-theme"]
                    }});
                }}

                window[key].cleanup = function() {{
                        cancelAnimationFrame(frame);
                        resizeObserver.disconnect();
                        mutationObserver.disconnect();
                        delete window[key];
                }};

                publish();
                "#
            );
            let mut eval = document::eval(&script);

            while let Ok(Some(size)) = eval.recv::<Option<f64>>().await {
                measured_divider_size.set(size);
            }
        });
    });

    use_drop(move || {
        _ = document::eval(&format!(
            r#"
            window.__dxSplitPaneDividerSize_{measurement_id}?.cleanup?.();
            "#
        ));
    });

    rsx! {
        split_pane::SplitPane {
            direction: props.direction,
            resizable: props.resizable,
            snap_points: props.snap_points,
            snap_tolerance: props.snap_tolerance,
            step: props.step,
            divider_size,
            divider_class,
            divider_style,
            on_resize_start: props.on_resize_start,
            on_resize: props.on_resize,
            on_resize_end: props.on_resize_end,
            attributes: merged,
            {props.children}
        }
    }
}

/// Styled split pane child pane.
#[component]
pub fn Pane(props: PaneProps) -> Element {
    let base = attributes!(div {
        class: Styles::dx_split_pane_pane
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        split_pane::Pane {
            size: props.size,
            default_size: props.default_size,
            min_size: props.min_size,
            max_size: props.max_size,
            attributes: merged,
            {props.children}
        }
    }
}

/// Styled focusable divider for resizing adjacent panes.
#[component]
pub fn SplitPaneDivider(props: SplitPaneDividerProps) -> Element {
    rsx! {
        split_pane::SplitPaneDivider {
            index: props.index,
            class: props.class,
            style: props.style,
            divider: props.divider,
            attributes: props.attributes,
            span { class: Styles::dx_split_pane_divider_handle }
            {props.children}
        }
    }
}
