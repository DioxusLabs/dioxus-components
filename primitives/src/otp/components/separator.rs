use dioxus::prelude::*;

/// The props for the [`OneTimePasswordSeparator`] component.
#[derive(Props, Clone, PartialEq)]
pub struct OneTimePasswordSeparatorProps {
    /// Additional attributes applied to the separator element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Decorative content rendered inside the separator (for example, an icon or dash).
    pub children: Element,
}

/// # OneTimePasswordSeparator
///
/// A purely decorative separator placed between [`super::OneTimePasswordGroup`]s. The
/// element is hidden from assistive technology because the underlying single
/// `<input>` already exposes the full value to screen readers — the visual division
/// between groups is purely cosmetic.
#[component]
pub fn OneTimePasswordSeparator(props: OneTimePasswordSeparatorProps) -> Element {
    rsx! {
        div {
            aria_hidden: "true",
            ..props.attributes,
            {props.children}
        }
    }
}
