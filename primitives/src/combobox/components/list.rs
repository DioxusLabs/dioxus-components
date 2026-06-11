//! Combobox options components.

use dioxus::prelude::*;

use super::super::context::ComboboxContext;
use crate::listbox::{use_listbox_container_with_open, use_listbox_id};

/// Props for [`ComboboxOptions`].
#[derive(Props, Clone, PartialEq)]
pub struct ComboboxOptionsProps {
    /// Optional id for the list element.
    #[props(default)]
    pub id: ReadSignal<Option<String>>,

    /// Additional attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children, typically [`ComboboxOption`](super::option::ComboboxOption)s
    /// and an optional [`ComboboxEmpty`](super::empty::ComboboxEmpty).
    pub children: Element,
}

/// Listbox that contains the visible options.
#[component]
pub fn ComboboxOptions(props: ComboboxOptionsProps) -> Element {
    let ctx = use_context::<ComboboxContext>();
    let open = use_memo(move || ctx.store.dropdown_opened());
    let id = use_listbox_id(props.id, ctx.selectable.list_id);
    let listbox = use_listbox_container_with_open(id, ctx.selectable, open);
    let render = listbox.render;

    rsx! {
        if render() {
            div {
                id: listbox.id,
                role: "listbox",
                "data-state": if open() { "open" } else { "closed" },
                onpointerdown: move |event| {
                    event.prevent_default();
                },
                ..props.attributes,
                {props.children}
            }
        } else {
            {props.children}
        }
    }
}

/// Compatibility props alias for [`ComboboxOptions`].
pub type ComboboxListProps = ComboboxOptionsProps;

/// Compatibility alias for [`ComboboxOptions`].
#[component]
pub fn ComboboxList(props: ComboboxListProps) -> Element {
    rsx! {
        ComboboxOptions {
            id: props.id,
            attributes: props.attributes,
            {props.children}
        }
    }
}
