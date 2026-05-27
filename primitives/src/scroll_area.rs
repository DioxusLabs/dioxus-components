//! Defines the [`ScrollArea`] component for creating scrollable areas with customizable scrollbars.

use dioxus::prelude::*;

/// The props for the [`ScrollArea`] component.
#[derive(Props, Clone, PartialEq)]
pub struct ScrollAreaProps {
    /// The scroll direction.
    #[props(default)]
    pub direction: ReadSignal<ScrollDirection>,

    /// Whether the scrollbars should be always visible.
    #[props(default)]
    pub always_show_scrollbars: ReadSignal<bool>,

    /// The scroll type.
    #[props(default)]
    pub scroll_type: ReadSignal<ScrollType>,

    /// Additional attributes to apply to the scroll area element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The children of the scroll area component.
    pub children: Element,
}

/// The direction in which scrolling is allowed.
#[derive(Clone, Copy, PartialEq, Default)]
pub enum ScrollDirection {
    /// Allow vertical scrolling only.
    Vertical,
    /// Allow horizontal scrolling only.
    Horizontal,
    /// Allow scrolling in both directions.
    #[default]
    Both,
}

/// The type of scrolling behavior.
#[derive(Clone, Copy, PartialEq, Default)]
pub enum ScrollType {
    /// Browser default scrolling.
    #[default]
    Auto,
    /// Always show scrollbars.
    Always,
    /// Hide scrollbars but enable scrolling.
    Hidden,
}

/// # ScrollArea
///
/// The `ScrollArea` component creates a scrollable area. If you don't
/// have any focusable content within the scroll area, you should make the
/// scroll area focusable by adding a `tabindex` attribute.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::scroll_area::{ScrollArea, ScrollDirection};
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         ScrollArea {
///             width: "10em",
///             height: "10em",
///             direction: ScrollDirection::Vertical,
///             tabindex: "0",
///             div { class: "dx-scroll-content",
///                 for i in 1..=20 {
///                     p {
///                         "Scrollable content item {i}"
///                     }
///                 }
///             }
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`ScrollArea`] component defines the following data attributes you can use to control styling:
/// - `data-scroll-direction`: Indicates the scroll direction. Values are `vertical`, `horizontal`, or `both`.
#[component]
pub fn ScrollArea(props: ScrollAreaProps) -> Element {
    let direction = props.direction;
    let scroll_type = props.scroll_type;
    let always_show = props.always_show_scrollbars;

    let (overflow_x, overflow_y, scrollbar_width) = match scroll_type() {
        ScrollType::Auto => match direction() {
            ScrollDirection::Vertical => (Some("hidden"), Some("auto"), None),
            ScrollDirection::Horizontal => (Some("auto"), Some("hidden"), None),
            ScrollDirection::Both => (Some("auto"), Some("auto"), None),
        },
        ScrollType::Always => match direction() {
            ScrollDirection::Vertical => (Some("hidden"), Some("scroll"), None),
            ScrollDirection::Horizontal => (Some("scroll"), Some("hidden"), None),
            ScrollDirection::Both => (Some("scroll"), Some("scroll"), None),
        },
        ScrollType::Hidden => match direction() {
            ScrollDirection::Vertical => (Some("hidden"), Some("scroll"), Some("none")),
            ScrollDirection::Horizontal => (Some("scroll"), Some("hidden"), Some("none")),
            ScrollDirection::Both => (Some("scroll"), Some("scroll"), Some("none")),
        },
    };

    let visibility_class = use_memo(move || {
        if always_show() {
            "dx-scroll-area-always-show"
        } else {
            "dx-scroll-area-auto-hide"
        }
    });

    rsx! {
        div {
            class: "{visibility_class}",
            overflow_x,
            overflow_y,
            "scrollbar-width": scrollbar_width,
            "data-scroll-direction": match direction() {
                ScrollDirection::Vertical => "vertical",
                ScrollDirection::Horizontal => "horizontal",
                ScrollDirection::Both => "both",
            },
            ..props.attributes,

            {props.children}
        }
    }
}
