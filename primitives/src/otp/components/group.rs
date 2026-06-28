use dioxus::prelude::*;

/// The props for the [`OneTimePasswordGroup`] component.
#[derive(Props, Clone, PartialEq)]
pub struct OneTimePasswordGroupProps {
    /// Additional attributes applied to the group element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The slots inside the group.
    pub children: Element,
}

/// # OneTimePasswordGroup
///
/// A visual grouping of [`super::OneTimePasswordSlot`]s. Used to render contiguous slots
/// separated by [`super::OneTimePasswordSeparator`]s.
#[component]
pub fn OneTimePasswordGroup(props: OneTimePasswordGroupProps) -> Element {
    rsx! {
        div {
            role: "presentation",
            ..props.attributes,
            {props.children}
        }
    }
}
