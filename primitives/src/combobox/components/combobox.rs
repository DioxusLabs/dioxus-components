//! Root combobox component.

use dioxus::prelude::*;

use super::super::context::{default_combobox_filter, ComboboxContext};
use super::super::hook::{use_combobox, UseComboboxOptions};
use crate::{
    selectable::{
        use_selectable_root_with_state, use_single_selectable_value, RcPartialEqValue,
        SelectionMode,
    },
    use_controlled, Controlled,
};

/// Props for [`Combobox`].
#[derive(Props, Clone, PartialEq)]
pub struct ComboboxProps<T: Clone + PartialEq + 'static = String> {
    /// The controlled value. If supplied, the combobox is controlled
    /// and the signal's `None` value means no option is selected.
    #[props(default)]
    pub value: Option<ReadSignal<Option<T>>>,

    /// The default uncontrolled value.
    #[props(default)]
    pub default_value: Option<T>,

    /// Callback fired when the value changes.
    #[props(default)]
    pub on_value_change: Callback<Option<T>>,

    /// Whether the combobox is disabled.
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

    /// Additional attributes for the root element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children.
    pub children: Element,
}

pub(super) fn use_combobox_root(
    values: Memo<Vec<RcPartialEqValue>>,
    set_value: Callback<RcPartialEqValue>,
    config: ComboboxRootConfig,
) -> Memo<bool> {
    let ComboboxRootConfig {
        selection_mode,
        disabled,
        roving_loop,
        open,
        query,
        filter,
    } = config;
    let store = use_combobox(UseComboboxOptions {
        opened: open.value,
        default_opened: open.default,
        on_opened_change: open.on_change,
        loop_navigation: roving_loop,
        ..Default::default()
    });
    let store_open = use_memo(move || store.dropdown_opened());
    let selectable = use_selectable_root_with_state(
        values,
        set_value,
        selection_mode,
        disabled,
        roving_loop,
        store_open,
        Callback::new(move |_| {}),
    );
    let (query, set_query) = use_controlled(query.value, query.default.cloned(), query.on_change);
    use_context_provider(|| ComboboxContext {
        selectable,
        store,
        query,
        set_query,
        filter,
    });

    use_memo(move || store.dropdown_opened())
}

pub(super) struct ComboboxRootConfig {
    pub(super) selection_mode: SelectionMode,
    pub(super) disabled: ReadSignal<bool>,
    pub(super) roving_loop: ReadSignal<bool>,
    pub(super) open: Controlled<bool>,
    pub(super) query: Controlled<String>,
    pub(super) filter: Callback<(String, String), bool>,
}

/// A single-select autocomplete input with a filterable popup list.
#[component]
pub fn Combobox<T: Clone + PartialEq + 'static>(props: ComboboxProps<T>) -> Element {
    let (selected, set_value) = use_single_selectable_value(
        props.value,
        props.default_value,
        props.on_value_change,
        "combobox",
    );

    let open = use_combobox_root(
        selected,
        set_value,
        ComboboxRootConfig {
            selection_mode: SelectionMode::Single,
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
            {props.children}
        }
    }
}
