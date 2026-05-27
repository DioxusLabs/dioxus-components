//! Defines the [`RadioGroup`] component and its sub-components.

use std::collections::HashMap;

use crate::{
    focus::{use_focus_controlled_item_disabled, use_focus_provider, FocusState},
    use_controlled, use_effect_with_cleanup,
};
use dioxus::prelude::*;

#[derive(Clone, Copy)]
struct RadioGroupCtx {
    // State
    disabled: ReadSignal<bool>,
    value: Memo<String>,
    set_value: Callback<String>,

    // Keyboard nav data
    // A map of tabindex -> value in the enabled radio items
    values: Signal<HashMap<usize, String>>,
    focus: FocusState,

    horizontal: ReadSignal<bool>,
    roving_loop: ReadSignal<bool>,
}

impl RadioGroupCtx {
    /// Set the currently focused radio item.
    ///
    /// This should be used by `focus`/`focusout` event only to start tracking focus.
    fn set_focus(&mut self, id: Option<usize>) {
        self.focus.set_focus(id);
    }

    /// Set the value of the radio group.
    fn set_value(&mut self, value: String) {
        let current_value = self.value.peek();
        if *current_value == value {
            return; // No change, do nothing
        }
        self.set_value.call(value);
    }

    fn focus_next(&mut self) {
        self.focus.focus_next();
        self.select_focused_value();
    }

    fn focus_prev(&mut self) {
        self.focus.focus_prev();
        self.select_focused_value();
    }

    fn select_focused_value(&mut self) {
        if let Some(current_focus) = self.focus.current_focus() {
            let value = { self.values.read().get(&current_focus).cloned() };
            if let Some(value) = value {
                self.set_value(value.clone());
            }
        }
    }

    fn focus_start(&mut self) {
        self.focus.focus_first();
    }

    fn focus_end(&mut self) {
        self.focus.focus_last();
    }
}

/// The props for the [`RadioGroup`] component.
#[derive(Props, Clone, PartialEq)]
pub struct RadioGroupProps {
    /// The controlled value of the selected radio item.
    pub value: ReadSignal<Option<String>>,

    /// The default selected value when uncontrolled.
    #[props(default)]
    pub default_value: String,

    /// Callback fired when the selected value changes.
    #[props(default)]
    pub on_value_change: Callback<String>,

    /// Whether the radio group is disabled.
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// Whether the radio group is required in a form.
    #[props(default)]
    pub required: ReadSignal<bool>,

    /// The name attribute for form submission.
    #[props(default)]
    pub name: ReadSignal<String>,

    /// Whether the radio group is horizontal.
    #[props(default)]
    pub horizontal: ReadSignal<bool>,

    /// Whether focus should loop around when reaching the end.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub roving_loop: ReadSignal<bool>,

    /// Additional attributes to apply to the radio group element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the radio group component.
    pub children: Element,
}

/// # RadioGroup
///
/// The `RadioGroup` component is a container for a group of [`RadioItem`] components that allows users to select a single option from a list of choices.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::radio_group::{RadioGroup, RadioItem};
///
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         RadioGroup {
///             RadioItem {
///                 value: "option1".to_string(),
///                 index: 0usize,
///                 "Blue"
///             }
///             RadioItem {
///                 value: "option2".to_string(),
///                 index: 1usize,
///                 "Red"
///             }
///             RadioItem {
///                 value: "option3".to_string(),
///                 index: 2usize,
///                 disabled: true,
///                 "Green"
///             }
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`RadioGroup`] component defines the following data attributes you can use to control styling:
/// - `data-orientation`: Indicates the orientation of the radio group. Values are `horizontal` or `vertical`.
/// - `data-disabled`: Indicates if the radio group is disabled. Values are `true` or `false`.
#[component]
pub fn RadioGroup(props: RadioGroupProps) -> Element {
    let (value, set_value) =
        use_controlled(props.value, props.default_value, props.on_value_change);

    let focus = use_focus_provider(props.roving_loop);
    let mut ctx = use_context_provider(|| RadioGroupCtx {
        value,
        set_value,
        disabled: props.disabled,

        values: Signal::new(Default::default()),
        focus,
        horizontal: props.horizontal,
        roving_loop: props.roving_loop,
    });

    rsx! {
        div {
            role: "radiogroup",
            "data-orientation": if (props.horizontal)() { "horizontal" } else { "vertical" },
            "data-disabled": (props.disabled)(),
            aria_required: props.required,

            onfocusout: move |_| ctx.set_focus(None),
            ..props.attributes,

            {props.children}
        }
    }
}

/// The props for the [`RadioItem`] component
#[derive(Props, Clone, PartialEq)]
pub struct RadioItemProps {
    /// The value of the radio item. This will be passed to [`RadioGroupProps::on_value_change`] when selected.
    pub value: ReadSignal<String>,
    /// The index of the radio item within the [`RadioGroup`]. This is used to order the items for keyboard navigation.
    pub index: ReadSignal<usize>,

    /// Whether the radio item is disabled.
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// Optional ID for the radio item element.
    pub id: Option<String>,
    /// Optional class for the radio item element.
    pub class: Option<String>,

    /// Additional attributes to apply to the radio item element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the radio item component.
    pub children: Element,
}

/// # RadioItem
///
/// The `RadioItem` component represents a single radio button within a [`RadioGroup`]. Only one radio item can be selected at a time within a group.
///
/// This must be used inside a [`RadioGroup`] component.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::radio_group::{RadioGroup, RadioItem};
///
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         RadioGroup {
///             RadioItem {
///                 value: "option1".to_string(),
///                 index: 0usize,
///                 "Blue"
///             }
///             RadioItem {
///                 value: "option2".to_string(),
///                 index: 1usize,
///                 "Red"
///             }
///             RadioItem {
///                 value: "option3".to_string(),
///                 index: 2usize,
///                 disabled: true,
///                 "Green"
///             }
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`RadioItem`] component defines the following data attributes you can use to control styling:
/// - `data-state`: Indicates the state of the radio item. Values are `checked` or `unchecked`.
/// - `data-disabled`: Indicates if the radio item is disabled. Values are `true` or `false`.
#[component]
pub fn RadioItem(props: RadioItemProps) -> Element {
    let mut ctx: RadioGroupCtx = use_context();
    let disabled = move || (ctx.disabled)() || (props.disabled)();

    use_effect_with_cleanup(move || {
        let index = (props.index)();
        if disabled() {
            ctx.values.write().remove(&index);
        } else {
            ctx.values.write().insert(index, (props.value)());
        }
        move || {
            ctx.values.write().remove(&index);
        }
    });

    let value = (props.value)().clone();
    let checked = use_memo(move || (ctx.value)() == value);

    // Tab index for roving index
    let tab_index = use_memo(move || {
        if !(ctx.roving_loop)() {
            return "0";
        }

        if checked() {
            return "0";
        }
        let current_focus = ctx.focus.current_focus();
        if let Some(current_focus) = current_focus {
            if current_focus == (props.index)() {
                return "0";
            }
        } else if (ctx.value)().is_empty() {
            return "0";
        }

        "-1"
    });

    let onmounted = use_focus_controlled_item_disabled(props.index, disabled);

    rsx! {
        button {
            role: "radio",
            id: props.id,
            class: props.class,
            tabindex: tab_index,
            type: "button",

            aria_checked: checked,
            "data-state": if checked() { "checked" } else { "unchecked" },
            "data-disabled": disabled(),
            disabled: disabled(),

            onclick: move |_| {
                let value = (props.value)().clone();
                ctx.set_value(value);
            },

            onmounted,
            onfocus: move |_| ctx.set_focus(Some((props.index)())),

            onkeydown: move |event: Event<KeyboardData>| {
                let key = event.key();
                let horizontal = (ctx.horizontal)();
                let mut prevent_default = true;
                match key {
                    Key::ArrowUp if !horizontal => ctx.focus_prev(),
                    Key::ArrowDown if !horizontal => ctx.focus_next(),
                    Key::ArrowLeft if horizontal => ctx.focus_prev(),
                    Key::ArrowRight if horizontal => ctx.focus_next(),
                    Key::Home => ctx.focus_start(),
                    Key::End => ctx.focus_end(),
                    _ => prevent_default = false,
                };
                if prevent_default {
                    event.prevent_default();
                }
            },
            ..props.attributes,

            {props.children}
        }
    }
}
