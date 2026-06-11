//! Higher-level combobox-based input components.

use core::panic;

use dioxus::prelude::*;

use super::combobox::use_combobox_root;
use super::{
    Combobox, ComboboxDropdownTarget, ComboboxInput, ComboboxOptions, ComboboxSearch,
    ComboboxTarget,
};
use crate::{
    combobox::context::default_combobox_filter,
    selectable::{RcPartialEqValue, SelectionMode},
    use_controlled, Controlled,
};

/// Props for [`Autocomplete`].
#[derive(Props, Clone, PartialEq)]
pub struct AutocompleteProps {
    /// The controlled input value.
    #[props(default)]
    pub value: Option<ReadSignal<Option<String>>>,

    /// The initial uncontrolled input value.
    #[props(default)]
    pub default_value: Option<String>,

    /// Callback fired when the input value changes.
    #[props(default)]
    pub on_value_change: Callback<Option<String>>,

    /// Whether the autocomplete is disabled.
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// The controlled open state of the popup.
    #[props(default)]
    pub open: ReadSignal<Option<bool>>,

    /// The initial open state when uncontrolled.
    #[props(default)]
    pub default_open: ReadSignal<bool>,

    /// Callback fired when the popup open state changes.
    #[props(default)]
    pub on_open_change: Callback<bool>,

    /// The controlled text query used to filter options.
    #[props(default)]
    pub query: ReadSignal<Option<String>>,

    /// The initial text query when uncontrolled.
    #[props(default)]
    pub default_query: ReadSignal<String>,

    /// Callback fired when the text query changes.
    #[props(default)]
    pub on_query_change: Callback<String>,

    /// Whether arrow-key navigation should wrap.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub roving_loop: ReadSignal<bool>,

    /// Custom filter callback. Receives `(query, option_text_value)`.
    #[props(default = Callback::new(|(q, t): (String, String)| default_combobox_filter(&q, &t)))]
    pub filter: Callback<(String, String), bool>,

    /// Placeholder shown when the input is empty.
    #[props(default)]
    pub placeholder: ReadSignal<String>,

    /// Additional attributes for the root element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Option children.
    pub children: Element,
}

/// A string autocomplete built on top of [`Combobox`].
#[component]
pub fn Autocomplete(props: AutocompleteProps) -> Element {
    rsx! {
        Combobox::<String> {
            value: props.value,
            default_value: props.default_value,
            on_value_change: props.on_value_change,
            disabled: props.disabled,
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            query: props.query,
            default_query: props.default_query,
            on_query_change: props.on_query_change,
            roving_loop: props.roving_loop,
            filter: props.filter,
            attributes: props.attributes,
            ComboboxInput {
                placeholder: props.placeholder,
            }
            ComboboxOptions {
                {props.children}
            }
        }
    }
}

/// Props for [`MultiSelect`].
#[derive(Props, Clone, PartialEq)]
pub struct MultiSelectProps<T: Clone + PartialEq + 'static = String> {
    /// The controlled list of selected values.
    #[props(default)]
    pub values: ReadSignal<Option<Vec<T>>>,

    /// The default list of selected values.
    #[props(default)]
    pub default_values: Vec<T>,

    /// Callback when the list of selected values changes.
    #[props(default)]
    pub on_values_change: Callback<Vec<T>>,

    /// Whether the multi-select is disabled.
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// The controlled open state of the popup.
    #[props(default)]
    pub open: ReadSignal<Option<bool>>,

    /// The initial open state when uncontrolled.
    #[props(default)]
    pub default_open: ReadSignal<bool>,

    /// Callback fired when the popup open state changes.
    #[props(default)]
    pub on_open_change: Callback<bool>,

    /// The controlled search query.
    #[props(default)]
    pub query: ReadSignal<Option<String>>,

    /// The initial search query when uncontrolled.
    #[props(default)]
    pub default_query: ReadSignal<String>,

    /// Callback fired when the search query changes.
    #[props(default)]
    pub on_query_change: Callback<String>,

    /// Whether arrow-key navigation should wrap.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub roving_loop: ReadSignal<bool>,

    /// Custom filter callback. Receives `(query, option_text_value)`.
    #[props(default = Callback::new(|(q, t): (String, String)| default_combobox_filter(&q, &t)))]
    pub filter: Callback<(String, String), bool>,

    /// Search placeholder.
    #[props(default)]
    pub placeholder: ReadSignal<String>,

    /// Maximum number of selected values.
    #[props(default)]
    pub max_values: Option<usize>,

    /// Renders a selected value as a pill inside the target.
    #[props(default)]
    pub render_value: Option<Callback<T, Element>>,

    /// Additional attributes for the root element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Option children.
    pub children: Element,
}

/// A searchable multi-select built on the combobox store.
#[component]
pub fn MultiSelect<T: Clone + PartialEq + 'static>(props: MultiSelectProps<T>) -> Element {
    let (values_state, set_values) =
        use_controlled(props.values, props.default_values, props.on_values_change);
    let values = use_memo(move || {
        values_state()
            .into_iter()
            .map(RcPartialEqValue::new)
            .collect()
    });
    let set_value = use_callback(move |incoming: RcPartialEqValue| {
        let value = incoming
            .as_ref::<T>()
            .unwrap_or_else(|| panic!("MultiSelect and option value types must match"))
            .clone();
        let mut current = values_state();
        if let Some(index) = current.iter().position(|item| item == &value) {
            current.remove(index);
        } else {
            if props
                .max_values
                .is_some_and(|max_values| current.len() >= max_values)
            {
                return;
            }
            current.push(value);
        }
        set_values.call(current);
    });

    let open = use_combobox_root(
        values,
        set_value,
        super::combobox::ComboboxRootConfig {
            selection_mode: SelectionMode::Multiple,
            disabled: props.disabled,
            roving_loop: props.roving_loop,
            open: Controlled {
                value: props.open,
                default: props.default_open,
                on_change: props.on_open_change,
            },
            query: Controlled {
                value: props.query,
                default: props.default_query,
                on_change: props.on_query_change,
            },
            filter: props.filter,
        },
    );

    rsx! {
        div {
            "data-state": if open() { "open" } else { "closed" },
            "data-disabled": (props.disabled)(),
            ..props.attributes,
            ComboboxTarget {
                "data-pills-input": true,
                if let Some(render_value) = props.render_value {
                    for (index, value) in values_state().into_iter().enumerate() {
                        Pill {
                            key: "{index}",
                            on_remove: Some(Callback::new(move |_| {
                                let mut next = values_state();
                                if index < next.len() {
                                    next.remove(index);
                                    set_values.call(next);
                                }
                            })),
                            {render_value.call(value)}
                        }
                    }
                }
                ComboboxSearch {
                    placeholder: props.placeholder,
                    show_selected_text: false,
                }
            }
            ComboboxDropdownTarget {
                ComboboxOptions {
                    {props.children}
                }
            }
        }
    }
}

/// Props for [`PillsInput`].
#[derive(Props, Clone, PartialEq)]
pub struct PillsInputProps {
    /// Whether the input is disabled.
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// Additional attributes for the root element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Pill and input children.
    pub children: Element,
}

/// A layout primitive for pill-based inputs.
#[component]
pub fn PillsInput(props: PillsInputProps) -> Element {
    rsx! {
        div {
            role: "group",
            "data-pills-input": true,
            "data-disabled": (props.disabled)(),
            ..props.attributes,
            {props.children}
        }
    }
}

/// Props for [`Pill`].
#[derive(Props, Clone, PartialEq)]
pub struct PillProps {
    /// Callback fired when the remove button is pressed.
    #[props(default)]
    pub on_remove: Option<Callback<()>>,

    /// Additional attributes for the pill.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Pill label.
    pub children: Element,
}

/// A removable pill item.
#[component]
pub fn Pill(props: PillProps) -> Element {
    rsx! {
        span {
            "data-pill": true,
            ..props.attributes,
            {props.children}
            if let Some(on_remove) = props.on_remove {
                button {
                    type: "button",
                    aria_label: "Remove",
                    onclick: move |_| on_remove.call(()),
                    "×"
                }
            }
        }
    }
}

/// Props for [`TagsInput`].
#[derive(Props, Clone, PartialEq)]
pub struct TagsInputProps {
    /// The controlled tag list.
    #[props(default)]
    pub values: ReadSignal<Option<Vec<String>>>,

    /// The default tag list.
    #[props(default)]
    pub default_values: Vec<String>,

    /// Callback fired when tags change.
    #[props(default)]
    pub on_values_change: Callback<Vec<String>>,

    /// Placeholder for the text input.
    #[props(default)]
    pub placeholder: ReadSignal<String>,

    /// Whether duplicate tags are allowed.
    #[props(default)]
    pub allow_duplicates: ReadSignal<bool>,

    /// Whether the tags input is disabled.
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// Additional attributes for the root element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// A basic tags input that owns tag parsing and pill removal.
#[component]
pub fn TagsInput(props: TagsInputProps) -> Element {
    let (values, set_values) =
        use_controlled(props.values, props.default_values, props.on_values_change);
    let mut input = use_signal(String::new);

    let add_tag = use_callback(move |tag: String| {
        let tag = tag.trim().to_string();
        if tag.is_empty() {
            return;
        }
        let mut next = values();
        if !(props.allow_duplicates)() && next.iter().any(|item| item == &tag) {
            return;
        }
        next.push(tag);
        set_values.call(next);
    });

    rsx! {
        PillsInput {
            disabled: props.disabled,
            attributes: props.attributes,
            for (index, tag) in values().into_iter().enumerate() {
                Pill {
                    key: "{tag}-{index}",
                    on_remove: Some(Callback::new(move |_| {
                        let mut next = values();
                        if index < next.len() {
                            next.remove(index);
                            set_values.call(next);
                        }
                    })),
                    "{tag}"
                }
            }
            input {
                type: "text",
                "data-pills-input-field": true,
                disabled: (props.disabled)(),
                value: input(),
                placeholder: props.placeholder,
                oninput: move |event| {
                    input.set(event.value());
                },
                onkeydown: move |event| {
                    let key = event.key();
                    let should_add = matches!(key, Key::Enter)
                        || matches!(key, Key::Character(ref value) if value == ",");
                    if should_add {
                        add_tag.call(input());
                        input.set(String::new());
                        event.prevent_default();
                        event.stop_propagation();
                        return;
                    }

                    match key {
                        Key::Backspace if input().is_empty() => {
                            let mut next = values();
                            if next.pop().is_some() {
                                set_values.call(next);
                            }
                        }
                        _ => {}
                    }
                },
            }
        }
    }
}
