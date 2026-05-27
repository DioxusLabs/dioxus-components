//! Combobox empty state component.

use dioxus::prelude::*;

use super::super::context::ComboboxContext;
use crate::listbox::ListboxContext;

/// Props for [`ComboboxEmpty`].
#[derive(Props, Clone, PartialEq)]
pub struct ComboboxEmptyProps {
    /// Additional attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children rendered when no options match.
    pub children: Element,
}

/// Renders when no option matches the current query.
#[component]
pub fn ComboboxEmpty(props: ComboboxEmptyProps) -> Element {
    let ctx = use_context::<ComboboxContext>();
    let render = use_context::<ListboxContext>().render;

    let any_visible = use_memo(move || ctx.has_visible_options());

    if !render() || any_visible() {
        return rsx! {};
    }

    rsx! {
        div {
            role: "presentation",
            ..props.attributes,
            {props.children}
        }
    }
}
