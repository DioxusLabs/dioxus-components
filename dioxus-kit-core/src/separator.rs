//! Defines the [`Separator`] component for creating visual or semantic separators.

use dioxus::prelude::*;

/// The props for the [`Separator`] component.
#[derive(Props, Clone, PartialEq)]
pub struct SeparatorProps {
    /// Horizontal if true, vertical if false.
    #[props(default = true)]
    pub horizontal: bool,

    /// If the separator is decorative and should not be classified
    /// as a separator to the ARIA standard.
    #[props(default = false)]
    pub decorative: bool,

    /// Additional attributes to apply to the separator element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the separator component.
    pub children: Element,
}

/// # Separator
///
/// The `Separator` component creates a visual or semantic divider between sections of content. If the divider
/// is purely decorative, it can be marked as such to avoid being classified as a separator by assistive technologies.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_kit_core::separator::Separator;
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         "One thing"
///         Separator {
///             style: "margin: 15px 0; width: 50%;",
///             horizontal: true,
///             decorative: true,
///         }
///         "Another thing"
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`Separator`] component defines the following data attributes you can use to control styling:
/// - `data-orientation`: Indicates the orientation of the separator. Values are `horizontal` or `vertical`.
#[component]
pub fn Separator(props: SeparatorProps) -> Element {
    let orientation = match props.horizontal {
        true => "horizontal",
        false => "vertical",
    };

    rsx! {
        div {
            role: if !props.decorative { "separator" } else { "none" },
            aria_orientation: if !props.decorative { orientation },
            "data-orientation": orientation,
            ..props.attributes,
            {props.children}
        }
    }
}
