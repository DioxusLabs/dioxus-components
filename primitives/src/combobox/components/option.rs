//! Combobox option components.

use dioxus::prelude::*;

use super::super::{context::ComboboxContext, hook::ComboboxDropdownEventSource};
use crate::{
    listbox::{ListboxContext, ListboxItemIndicator},
    selectable::{
        pointer_select_cancel, pointer_select_commit, pointer_select_start, use_selectable_option,
        SelectableOptionConfig,
    },
    selection::option_text_value,
    use_effect_with_cleanup,
};

/// Props for [`ComboboxOption`].
#[derive(Props, Clone, PartialEq)]
pub struct ComboboxOptionProps<T: Clone + PartialEq + 'static> {
    /// The value carried by this option.
    pub value: ReadSignal<T>,

    /// Display/searchable text. Required for non-string types.
    #[props(default)]
    pub text_value: ReadSignal<Option<String>>,

    /// Whether the option is disabled.
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// Optional id for the option element.
    #[props(default)]
    pub id: ReadSignal<Option<String>>,

    /// Registration order used for keyboard navigation.
    pub index: ReadSignal<usize>,

    /// Optional aria-label.
    #[props(default)]
    pub aria_label: Option<String>,

    /// Optional aria-roledescription.
    #[props(default)]
    pub aria_roledescription: Option<String>,

    /// Additional attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children rendered inside the option.
    pub children: Element,
}

/// A selectable option inside a [`ComboboxList`](super::list::ComboboxList).
#[component]
pub fn ComboboxOption<T: PartialEq + Clone + 'static>(props: ComboboxOptionProps<T>) -> Element {
    let index = props.index;

    let mut ctx: ComboboxContext = use_context();
    let text_value = use_memo(move || {
        option_text_value(&*props.value.read(), (props.text_value)(), "ComboboxOption")
    });
    let visible = move || ctx.is_visible_text(index(), text_value.cloned());
    let option = use_selectable_option(
        ctx.selectable,
        SelectableOptionConfig {
            id: props.id,
            index,
            value: props.value,
            text_value: props.text_value,
            option_disabled: props.disabled,
            component_name: "ComboboxOption",
        },
    );
    use_effect_with_cleanup({
        let store = ctx.store;
        let id = option.id;
        let disabled = option.disabled;
        let selected = option.selected;
        move || {
            let id_value = id.cloned();
            store.register_option(id_value.clone(), index(), disabled(), visible(), selected());
            move || store.unregister_option(&id_value)
        }
    });

    let render = use_context::<ListboxContext>().render;

    rsx! {
        if render() && visible() {
            div {
                role: "option",
                id: option.id,

                aria_selected: (option.selected)(),
                aria_disabled: (option.disabled)(),
                aria_label: props.aria_label.clone(),
                aria_roledescription: props.aria_roledescription.clone(),

                "data-highlighted": (option.focused)(),
                "data-disabled": (option.disabled)(),
                "data-selected": (option.selected)(),

                onmouseenter: move |_| {
                    if !(option.disabled)() {
                        ctx.selectable.focus_state.set_focus(Some((option.index)()));
                        ctx.store.select_option((option.index)());
                    }
                },
                onpointerdown: move |event| {
                    pointer_select_start(&event, (option.disabled)(), option.down_pos);
                },
                onpointerup: move |event| {
                    if pointer_select_commit(&event, (option.disabled)(), option.down_pos) {
                        ctx.submit_index((option.index)(), ComboboxDropdownEventSource::Mouse);
                    }
                },
                onpointercancel: move |_| {
                    pointer_select_cancel(option.down_pos);
                },

                ..props.attributes,
                {props.children}
            }
        }
    }
}

/// Props for [`ComboboxItemIndicator`].
#[derive(Props, Clone, PartialEq)]
pub struct ComboboxItemIndicatorProps {
    /// Children rendered only when the parent option is selected.
    pub children: Element,
}

/// Renders its children when the parent option is selected.
#[component]
pub fn ComboboxItemIndicator(props: ComboboxItemIndicatorProps) -> Element {
    rsx! {
        ListboxItemIndicator {
            {props.children}
        }
    }
}
