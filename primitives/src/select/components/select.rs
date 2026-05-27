//! Main Select and SelectMulti component implementations.

use core::panic;
use std::time::Duration;

use crate::{
    selectable::{
        use_selectable_root, use_single_selectable_value, RcPartialEqValue, SelectionMode,
    },
    use_controlled, use_effect, Controlled,
};
use dioxus::prelude::*;
use dioxus_core::Task;

use super::super::context::SelectContext;

/// Props for the [`Select`] (single-select) component
#[derive(Props, Clone, PartialEq)]
pub struct SelectProps<T: Clone + PartialEq + 'static = String> {
    /// The controlled value of the select. If supplied, the select is controlled
    /// and the signal's `None` value means no option is selected.
    #[props(default)]
    pub value: Option<ReadSignal<Option<T>>>,

    /// The initial value of the select when uncontrolled. `None` means no initial
    /// selection — the placeholder is shown until the user picks an option.
    #[props(default)]
    pub default_value: Option<T>,

    /// Callback fired when the selected value changes.
    #[props(default)]
    pub on_value_change: Callback<Option<T>>,

    /// Whether the select is disabled
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// The controlled open state of the select popup.
    #[props(default)]
    pub open: ReadSignal<Option<bool>>,

    /// The initial open state when uncontrolled.
    #[props(default)]
    pub default_open: ReadSignal<bool>,

    /// Callback fired when the popup open state changes.
    #[props(default)]
    pub on_open_change: Callback<bool>,

    /// Name of the select for form submission
    #[props(default)]
    pub name: ReadSignal<String>,

    /// Whether focus should loop around when reaching the end.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub roving_loop: ReadSignal<bool>,

    /// Timeout in milliseconds before clearing typeahead buffer
    #[props(default = ReadSignal::new(Signal::new(Duration::from_millis(1000))))]
    pub typeahead_timeout: ReadSignal<Duration>,

    /// Additional attributes for the select element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the Select component
    pub children: Element,
}

/// Props for the [`SelectMulti`] (multi-select) component
#[derive(Props, Clone, PartialEq)]
pub struct SelectMultiProps<T: Clone + PartialEq + 'static = String> {
    /// The controlled list of selected values.
    #[props(default)]
    pub values: ReadSignal<Option<Vec<T>>>,

    /// The default list of selected values.
    #[props(default)]
    pub default_values: Vec<T>,

    /// Callback when the list of selected values changes.
    #[props(default)]
    pub on_values_change: Callback<Vec<T>>,

    /// Whether the select is disabled
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// The controlled open state of the select popup.
    #[props(default)]
    pub open: ReadSignal<Option<bool>>,

    /// The initial open state when uncontrolled.
    #[props(default)]
    pub default_open: ReadSignal<bool>,

    /// Callback fired when the popup open state changes.
    #[props(default)]
    pub on_open_change: Callback<bool>,

    /// Name of the select for form submission
    #[props(default)]
    pub name: ReadSignal<String>,

    /// Whether focus should loop around when reaching the end.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub roving_loop: ReadSignal<bool>,

    /// Timeout in milliseconds before clearing typeahead buffer
    #[props(default = ReadSignal::new(Signal::new(Duration::from_millis(1000))))]
    pub typeahead_timeout: ReadSignal<Duration>,

    /// Additional attributes for the select element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the SelectMulti component
    pub children: Element,
}

/// Sets up the shared signals, focus, and context that both [`Select`] and
/// [`SelectMulti`] need. Returns the `open` signal for the root `<div>`.
fn use_select_root(
    values: Memo<Vec<RcPartialEqValue>>,
    set_value: Callback<RcPartialEqValue>,
    selection_mode: SelectionMode,
    disabled: ReadSignal<bool>,
    roving_loop: ReadSignal<bool>,
    open: Controlled<bool>,
    typeahead_timeout: ReadSignal<Duration>,
) -> Memo<bool> {
    let selectable = use_selectable_root(
        values,
        set_value,
        selection_mode,
        disabled,
        roving_loop,
        open,
    );
    let mut typeahead_buffer = use_signal(String::new);
    let adaptive_keyboard = use_signal(super::super::text_search::AdaptiveKeyboard::new);
    let mut typeahead_clear_task: Signal<Option<Task>> = use_signal(|| None);
    let open = selectable.open;

    // Clear the typeahead buffer when the select is closed
    use_effect(move || {
        if !open() {
            if let Some(task) = typeahead_clear_task.write().take() {
                task.cancel();
            }
            typeahead_buffer.take();
        }
    });
    use_context_provider(|| SelectContext {
        selectable,
        adaptive_keyboard,
        typeahead_buffer,
        typeahead_clear_task,
        typeahead_timeout,
    });

    open
}

/// # Select
///
/// The `Select` component is a searchable single-select dropdown that allows users to choose
/// one option from a list with keyboard navigation and typeahead search functionality. For
/// selecting multiple values, see [`SelectMulti`].
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::select::{
///     Select, SelectGroup, SelectGroupLabel, SelectItemIndicator, SelectList, SelectOption,
///     SelectTrigger, SelectValue,
/// };
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         Select::<String> {
///             SelectTrigger {
///                 aria_label: "Select Trigger",
///                 width: "12rem",
///                 SelectValue { placeholder: "Select a fruit..." }
///             }
///             SelectList {
///                 aria_label: "Select Demo",
///                 SelectGroup {
///                     SelectGroupLabel { "Fruits" }
///                     SelectOption::<String> {
///                         index: 0usize,
///                         value: "apple",
///                         "Apple"
///                         SelectItemIndicator { "✔️" }
///                     }
///                     SelectOption::<String> {
///                         index: 1usize,
///                         value: "banana",
///                         "Banana"
///                         SelectItemIndicator { "✔️" }
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
/// The [`Select`] component defines the following data attributes you can use to control styling:
/// - `data-state`: Indicates the current state of the select. Values are `open` or `closed`.
#[component]
pub fn Select<T: Clone + PartialEq + 'static>(props: SelectProps<T>) -> Element {
    let (values, set_value) = use_single_selectable_value(
        props.value,
        props.default_value,
        props.on_value_change,
        "select",
    );

    let open = use_select_root(
        values,
        set_value,
        SelectionMode::Single,
        props.disabled,
        props.roving_loop,
        Controlled {
            value: props.open,
            default: props.default_open,
            on_change: props.on_open_change,
        },
        props.typeahead_timeout,
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

/// # SelectMulti
///
/// The `SelectMulti` component is a searchable multi-select dropdown. Selecting an option
/// toggles it in or out of the selection and the dropdown stays open across selections; it
/// closes via Escape, the trigger, or tabbing out of the listbox. For single-selection use
/// [`Select`] instead.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::select::{
///     SelectMulti, SelectGroup, SelectGroupLabel, SelectItemIndicator, SelectList, SelectOption,
///     SelectTrigger, SelectValue,
/// };
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         SelectMulti::<String> {
///             default_values: vec!["pepperoni".into()],
///             SelectTrigger {
///                 aria_label: "Select Trigger",
///                 width: "16rem",
///                 SelectValue { placeholder: "Pick toppings..." }
///             }
///             SelectList {
///                 aria_label: "Topping Picker",
///                 SelectGroup {
///                     SelectGroupLabel { "Toppings" }
///                     SelectOption::<String> {
///                         index: 0usize,
///                         value: "pepperoni",
///                         "Pepperoni"
///                         SelectItemIndicator { "✔️" }
///                     }
///                     SelectOption::<String> {
///                         index: 1usize,
///                         value: "mushroom",
///                         "Mushroom"
///                         SelectItemIndicator { "✔️" }
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
/// The [`SelectMulti`] component defines the following data attributes you can use to control styling:
/// - `data-state`: Indicates the current state of the select. Values are `open` or `closed`.
#[component]
pub fn SelectMulti<T: Clone + PartialEq + 'static>(props: SelectMultiProps<T>) -> Element {
    let (multi_values, set_multi_internal) =
        use_controlled(props.values, props.default_values, props.on_values_change);

    let values = use_memo(move || {
        multi_values()
            .into_iter()
            .map(RcPartialEqValue::new)
            .collect()
    });
    let set_value = use_callback(move |value: RcPartialEqValue| {
        let value_t = value
            .as_ref::<T>()
            .unwrap_or_else(|| panic!("The values of select and all options must match types"))
            .clone();
        let mut current = multi_values();
        if let Some(pos) = current.iter().position(|v| v == &value_t) {
            current.remove(pos);
        } else {
            current.push(value_t);
        }
        set_multi_internal.call(current);
    });

    let open = use_select_root(
        values,
        set_value,
        SelectionMode::Multiple,
        props.disabled,
        props.roving_loop,
        Controlled {
            value: props.open,
            default: props.default_open,
            on_change: props.on_open_change,
        },
        props.typeahead_timeout,
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
