//! Defines the [`Tabs`] component and its sub-components.

use crate::{
    collection::{collection_item, use_collection_provider, use_item, CollectionState},
    use_controlled, use_id_or, use_unique_id,
};
use dioxus::prelude::*;

#[derive(Clone, Copy)]
struct TabsContext {
    // State
    value: ReadSignal<String>,
    set_value: Callback<String>,
    disabled: ReadSignal<bool>,

    // Focus state
    focus: CollectionState,

    // Orientation
    horizontal: ReadSignal<bool>,

    // ARIA attributes
    tab_content_ids: Signal<Vec<String>>,
}

/// The props for the [`Tabs`] component.
#[derive(Props, Clone, PartialEq)]
pub struct TabsProps {
    /// The controlled value of the active tab.
    pub value: ReadSignal<Option<String>>,

    /// The default active tab value when uncontrolled.
    #[props(default)]
    pub default_value: String,

    /// Callback fired when the active tab changes.
    #[props(default)]
    pub on_value_change: Callback<String>,

    /// Whether the tabs are disabled.
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// Whether the tabs are horizontal.
    #[props(default)]
    pub horizontal: ReadSignal<bool>,

    /// Whether focus should loop around when reaching the end.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub roving_loop: ReadSignal<bool>,

    /// Additional attributes to apply to the tabs element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the tabs component.
    pub children: Element,
}

/// # Tabs
///
/// The `Tabs` component creates a tabbed interface that allows users to switch between different panels
/// of content. The [`TabTrigger`] component is used to switch between the different [`TabContent`]s.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::tabs::{TabContent, TabTrigger, Tabs, TabList};
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         Tabs {
///             default_value: "tab1".to_string(),
///             horizontal: true,
///             TabList {
///                 TabTrigger {
///                     value: "tab1".to_string(),
///                     index: 0usize,
///                     "Tab 1"
///                 }
///                 TabTrigger {
///                     value: "tab2".to_string(),
///                     index: 1usize,
///                     "Tab 2"
///                 }
///             }
///             TabContent {
///                 index: 0usize,
///                 value: "tab1".to_string(),
///                 "Tab 1 Content"
///             }
///             TabContent {
///                 index: 1usize,
///                 value: "tab2".to_string(),
///                 "Tab 2 Content"
///             }
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`Tabs`] component defines the following data attributes you can use to control styling:
/// - `data-orientation`: Indicates the orientation of the tabs. Values are `horizontal` or `vertical`.
/// - `data-disabled`: Indicates if the tabs are disabled. Values are `true` or `false`.
#[component]
pub fn Tabs(props: TabsProps) -> Element {
    let (value, set_value) =
        use_controlled(props.value, props.default_value, props.on_value_change);

    let focus = use_collection_provider(props.roving_loop);
    let mut ctx = use_context_provider(|| TabsContext {
        value: value.into(),
        set_value,
        disabled: props.disabled,

        focus,

        horizontal: props.horizontal,
        tab_content_ids: Signal::new(Vec::new()),
    });

    rsx! {
        div {
            "data-orientation": if (props.horizontal)() { "horizontal" } else { "vertical" },
            "data-disabled": (props.disabled)(),

            onfocusout: move |_| ctx.focus.clear_focus(),
            ..props.attributes,

            {props.children}
        }
    }
}

/// The props for the [`TabList`] component.
#[derive(Props, Clone, PartialEq)]
pub struct TabListProps {
    /// Additional attributes to apply to the tab list element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the tab list component.
    pub children: Element,
}

/// # TabList
///
/// The `TabList` component contains a list of [`TabTrigger`] components that allow users to switch between different tabs.
///
/// This must be used inside a [`Tabs`] component.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::tabs::{TabContent, TabTrigger, Tabs, TabList};
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         Tabs {
///             default_value: "tab1".to_string(),
///             horizontal: true,
///             TabList {
///                 TabTrigger {
///                     value: "tab1".to_string(),
///                     index: 0usize,
///                     "Tab 1"
///                 }
///                 TabTrigger {
///                     value: "tab2".to_string(),
///                     index: 1usize,
///                     "Tab 2"
///                 }
///             }
///             TabContent {
///                 index: 0usize,
///                 value: "tab1".to_string(),
///                 "Tab 1 Content"
///             }
///             TabContent {
///                 index: 1usize,
///                 value: "tab2".to_string(),
///                 "Tab 2 Content"
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn TabList(props: TabListProps) -> Element {
    rsx! {
        div {
            role: "tablist",
            ..props.attributes,

            {props.children}
        }
    }
}

/// The props for the [`TabTrigger`] component
#[derive(Props, Clone, PartialEq)]
pub struct TabTriggerProps {
    /// The value of the tab trigger, which is used to identify the corresponding tab content. This
    /// must match the `value` prop of the corresponding [`TabContent`].
    pub value: String,
    /// The index of the tab trigger. This is used to define the focus order for keyboard navigation.
    pub index: ReadSignal<usize>,

    /// Whether the tab trigger is disabled.
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// The ID of the tab trigger element.
    pub id: Option<String>,
    /// The class of the tab trigger element.
    pub class: Option<String>,

    /// Additional attributes to apply to the tab trigger element.
    #[props(extends = GlobalAttributes)]
    #[props(extends = button)]
    pub attributes: Vec<Attribute>,

    /// The children of the tab trigger component.
    pub children: Element,
}

/// # TabTrigger
///
/// The `TabTrigger` component is a button that switches to the [`TabContent`] with the same `value` when clicked.
///
/// This must be used inside a [`TabList`] component.
///
/// ## Example
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::tabs::{TabContent, TabTrigger, Tabs, TabList};
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         Tabs {
///             default_value: "tab1".to_string(),
///             horizontal: true,
///             TabList {
///                 TabTrigger {
///                     value: "tab1".to_string(),
///                     index: 0usize,
///                     "Tab 1"
///                 }
///                 TabTrigger {
///                     value: "tab2".to_string(),
///                     index: 1usize,
///                     "Tab 2"
///                 }
///             }
///             TabContent {
///                 index: 0usize,
///                 value: "tab1".to_string(),
///                 "Tab 1 Content"
///             }
///             TabContent {
///                 index: 1usize,
///                 value: "tab2".to_string(),
///                 "Tab 2 Content"
///             }
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`TabTrigger`] component defines the following data attributes you can use to control styling:
/// - `data-state`: Indicates the state of the tab trigger. Values are `active` or `inactive`.
/// - `data-disabled`: Indicates if the tab trigger is disabled. Values are `true` or `false`.
#[component]
pub fn TabTrigger(props: TabTriggerProps) -> Element {
    let mut ctx: TabsContext = use_context();

    let value = props.value.clone();
    let selected = use_memo(move || (ctx.value)() == value);

    let disabled = move || (ctx.disabled)() || (props.disabled)();
    let item = use_item(
        collection_item(ctx.focus, props.index)
            .disabled(disabled)
            .selected(move || selected.cloned()),
    );
    let onmounted = item.onmounted();
    let tab_index = item.tabindex;

    rsx! {
        button {
            role: "tab",
            id: props.id,
            class: props.class,
            tabindex: tab_index,
            type: "button",

            aria_selected: selected,
            aria_controls: (ctx.tab_content_ids)().get((props.index)()).cloned(),
            "data-state": if selected() { "active" } else { "inactive" },
            "data-disabled": disabled(),
            disabled: disabled(),

            onmounted,
            onclick: move |_| {
                let value = props.value.clone();
                if !selected() {
                    ctx.set_value.call(value);
                }
            },

            onfocus: move |_| ctx.focus.set_focus(Some((props.index)())),

            onkeydown: move |event: Event<KeyboardData>| {
                let key = event.key();
                let horizontal = (ctx.horizontal)();
                let mut prevent_default = true;
                match key {
                    Key::ArrowUp if !horizontal => ctx.focus.focus_prev(),
                    Key::ArrowDown if !horizontal => ctx.focus.focus_next(),
                    Key::ArrowLeft if horizontal => ctx.focus.focus_prev(),
                    Key::ArrowRight if horizontal => ctx.focus.focus_next(),
                    Key::Home => ctx.focus.focus_first(),
                    Key::End => ctx.focus.focus_last(),
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

/// The props for the [`TabContent`] component
#[derive(Props, Clone, PartialEq)]
pub struct TabContentProps {
    /// The value of the tab content, which must match the `value` prop of the corresponding [`TabTrigger`].
    pub value: String,

    /// The ID of the tab content element.
    pub id: ReadSignal<Option<String>>,
    /// The class of the tab content element.
    pub class: Option<String>,

    /// The index of the tab content. This is used to define the focus order for keyboard navigation.
    pub index: ReadSignal<usize>,

    /// Additional attributes to apply to the tab content element.
    #[props(extends = GlobalAttributes)]
    #[props(extends = div)]
    pub attributes: Vec<Attribute>,

    /// The children of the tab content element.
    pub children: Element,
}

/// # TabContent
///
/// The content of a tab panel. This component will only be rendered when its corresponding [`TabTrigger`] is active.
///
/// This should be used inside a [`Tabs`] component.
///
/// ## Example
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::tabs::{TabContent, TabTrigger, Tabs, TabList};
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         Tabs {
///             default_value: "tab1".to_string(),
///             horizontal: true,
///             TabList {
///                 TabTrigger {
///                     value: "tab1".to_string(),
///                     index: 0usize,
///                     "Tab 1"
///                 }
///                 TabTrigger {
///                     value: "tab2".to_string(),
///                     index: 1usize,
///                     "Tab 2"
///                 }
///             }
///             TabContent {
///                 index: 0usize,
///                 value: "tab1".to_string(),
///                 "Tab 1 Content"
///             }
///             TabContent {
///                 index: 1usize,
///                 value: "tab2".to_string(),
///                 "Tab 2 Content"
///             }
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`TabTrigger`] component defines the following data attributes you can use to control styling:
/// - `data-state`: Indicates the state of the tab trigger. Values are `active` or `inactive`.
#[component]
pub fn TabContent(props: TabContentProps) -> Element {
    let mut ctx: TabsContext = use_context();
    let selected = use_memo(move || (ctx.value)() == props.value);
    let uuid = use_unique_id();
    let id = use_id_or(uuid, props.id);

    use_effect(move || {
        let mut tab_ids = ctx.tab_content_ids.write();
        let index = (props.index)();
        while tab_ids.len() <= index {
            tab_ids.push(String::new());
        }
        tab_ids[index] = id();
    });

    rsx! {
        div {
            role: "tabpanel",
            id,
            class: props.class,

            tabindex: "0",
            "data-state": if selected() { "active" } else { "inactive" },
            hidden: if !selected() { "true" },
            ..props.attributes,

            if selected() {
                {props.children}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{TabContent, TabList, TabTrigger, Tabs};
    use dioxus::{
        html::{Key, Modifiers},
        prelude::*,
    };
    use dioxus_test::{
        by_role,
        matchers::{contains_substring, has_focus, inner_html},
        render, Result, Role,
    };

    #[tokio::test]
    async fn arrow_right_moves_focus_to_next_tab() -> Result<()> {
        #[component]
        fn TestComponent() -> Element {
            rsx! {
                Tabs {
                    horizontal: true,
                    TabList {
                        TabTrigger {
                            value: "first-tab".to_string(),
                            index: 0usize,
                            "First Tab"
                        }
                        TabTrigger {
                            value: "second-tab".to_string(),
                            index: 1usize,
                            "Second Tab"
                        }
                    }
                    TabContent {
                        index: 0usize,
                        value: "first-tab".to_string(),
                        "First tab content"
                    }
                    TabContent {
                        index: 1usize,
                        value: "second-tab".to_string(),
                        "Second tab content"
                    }
                }
            }
        }
        let tester = render(TestComponent);
        tester
            .query(by_role(Role::Tab).having_name("First Tab"))
            .focus()
            .await?;

        tester.key_down(Key::ArrowRight, Modifiers::empty())?;

        tester
            .query(by_role(Role::Tab).having_name("Second Tab"))
            .expect(has_focus())
            .await
    }

    #[tokio::test]
    async fn clicking_a_tab_opens_it() -> Result<()> {
        #[component]
        fn TestComponent() -> Element {
            rsx! {
                Tabs {
                    horizontal: true,
                    TabList {
                        TabTrigger {
                            value: "first-tab".to_string(),
                            index: 0usize,
                            "First Tab"
                        }
                        TabTrigger {
                            value: "second-tab".to_string(),
                            index: 1usize,
                            "Second Tab"
                        }
                    }
                    TabContent {
                        index: 0usize,
                        value: "first-tab".to_string(),
                        "First tab content"
                    }
                    TabContent {
                        index: 1usize,
                        value: "second-tab".to_string(),
                        "Second tab content"
                    }
                }
            }
        }
        let tester = render(TestComponent);

        tester
            .query(by_role(Role::Tab).having_name("Second Tab"))
            .click()
            .await?;

        tester
            .query(by_role(Role::TabPanel))
            .expect(inner_html(contains_substring("Second tab content")))
            .await
    }
}
