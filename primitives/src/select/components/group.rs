//! SelectGroup and SelectGroupLabel component implementations.

use crate::{listbox::ListboxContext, use_effect, use_id_or, use_unique_id};
use dioxus::prelude::*;

use super::super::context::{SelectContext, SelectGroupContext};

/// The props for the [`SelectGroup`] component
#[derive(Props, Clone, PartialEq)]
pub struct SelectGroupProps {
    /// Whether the group is disabled
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// Optional ID for the group
    #[props(default)]
    pub id: ReadSignal<Option<String>>,

    /// Additional attributes for the group
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children to render inside the group
    pub children: Element,
}

/// # SelectGroup
///
/// The `SelectGroup` component is used to group related options within a [`SelectList`](super::list::SelectList). It provides a way to organize options into logical sections.
///
/// This must be used inside a [`SelectList`](super::list::SelectList) component.
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
#[component]
pub fn SelectGroup(props: SelectGroupProps) -> Element {
    let ctx = use_context::<SelectContext>();
    let disabled = ctx.selectable.disabled.cloned() || props.disabled.cloned();

    let labeled_by = use_signal(|| None);

    use_context_provider(|| SelectGroupContext { labeled_by });
    let render = use_context::<ListboxContext>().render;

    rsx! {
        if render() {
            div {
                role: "group",

                // ARIA attributes
                aria_disabled: disabled,
                aria_labelledby: labeled_by,

                ..props.attributes,
                {props.children}
            }
        } else {
            // If we are not rendering, still render the children components
            {props.children}
        }
    }
}

/// The props for the [`SelectGroupLabel`] component
#[derive(Props, Clone, PartialEq)]
pub struct SelectGroupLabelProps {
    /// Optional ID for the label
    pub id: ReadSignal<Option<String>>,

    /// Additional attributes for the label
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children to render inside the label
    pub children: Element,
}

/// # SelectGroupLabel
///
/// The `SelectGroupLabel` component is used to render a label for a group of options within a [`SelectList`](super::list::SelectList).
///
/// This must be used inside a [`SelectGroup`](SelectGroup) component.
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
#[component]
pub fn SelectGroupLabel(props: SelectGroupLabelProps) -> Element {
    let mut ctx: SelectGroupContext = use_context();

    let id = use_unique_id();
    let id = use_id_or(id, props.id);

    use_effect(move || {
        ctx.labeled_by.set(Some(id()));
    });

    let render = use_context::<ListboxContext>().render;

    rsx! {
        if render () {
            div {
                // Set the ID for the label
                id,
                ..props.attributes,
                {props.children}
            }
        }
    }
}
