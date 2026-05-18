//! Combobox input component.

use dioxus::prelude::*;

use super::super::context::ComboboxContext;
use crate::{use_id_or, use_unique_id};

/// Props for [`ComboboxInput`].
#[derive(Props, Clone, PartialEq)]
pub struct ComboboxInputProps {
    /// Placeholder shown when the input is empty.
    #[props(default)]
    pub placeholder: ReadSignal<String>,

    /// Optional id for the input element.
    #[props(default)]
    pub id: ReadSignal<Option<String>>,

    /// Additional attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// The text input that opens and filters the popup list.
#[component]
pub fn ComboboxInput(props: ComboboxInputProps) -> Element {
    let mut ctx = use_context::<ComboboxContext>();

    let id = use_unique_id();
    let id = use_id_or(id, props.id);

    let open = ctx.selectable.open;
    let query = ctx.query;
    let set_query = ctx.set_query;

    let active_descendant = use_memo(move || {
        if !open() {
            return None;
        }
        ctx.focused_option_id()
    });

    let display_value = use_memo(move || {
        if open() {
            query.cloned()
        } else {
            ctx.selectable.selected_text().unwrap_or_default()
        }
    });

    let onkeydown = move |event: KeyboardEvent| match event.key() {
        Key::ArrowDown => {
            if !open() {
                ctx.open_with_empty_query_and_focus_first();
            } else {
                ctx.focus_next_visible();
            }
            event.prevent_default();
            event.stop_propagation();
        }
        Key::ArrowUp => {
            if !open() {
                ctx.open_with_empty_query_and_focus_last();
            } else {
                ctx.focus_prev_visible();
            }
            event.prevent_default();
            event.stop_propagation();
        }
        Key::Home if open() => {
            ctx.focus_first_visible();
            event.prevent_default();
            event.stop_propagation();
        }
        Key::End if open() => {
            ctx.focus_last_visible();
            event.prevent_default();
            event.stop_propagation();
        }
        Key::Enter if open() => {
            ctx.select_focused();
            event.prevent_default();
            event.stop_propagation();
        }
        Key::Escape if open() => {
            ctx.set_open(false);
            event.prevent_default();
            event.stop_propagation();
        }
        _ => {}
    };

    rsx! {
        input {
            id,
            r#type: "text",
            value: display_value(),
            placeholder: props.placeholder,
            autocomplete: "off",
            spellcheck: "false",
            disabled: (ctx.selectable.disabled)(),

            role: "combobox",
            aria_autocomplete: "list",
            aria_haspopup: "listbox",
            aria_expanded: open(),
            aria_controls: ctx.selectable.list_id,
            aria_activedescendant: active_descendant(),

            "data-state": if open() { "open" } else { "closed" },

            onclick: move |_| {
                if !open() {
                    set_query.call(String::new());
                    ctx.set_open(true);
                }
            },
            oninput: move |event| {
                let was_open = open();
                let value = event.value();
                let next_query = if was_open {
                    value
                } else {
                    ctx.selectable
                        .selected_text()
                        .and_then(|selected| {
                            value
                                .strip_prefix(&selected)
                                .map(ToString::to_string)
                        })
                        .unwrap_or(value)
                };
                set_query.call(next_query);
                if was_open {
                    ctx.selectable.focus_state.set_focus(None);
                } else {
                    ctx.set_open(true);
                }
            },
            onkeydown,
            onblur: move |_| {
                if open() {
                    ctx.set_open(false);
                }
            },

            ..props.attributes,
        }
    }
}
