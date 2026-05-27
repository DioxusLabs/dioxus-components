//! Defines the [`Accordion`] component and its sub-components.

use crate::dioxus_elements::Key;
use crate::focus::{use_focus_control_disabled, use_focus_entry_disabled, FocusState};
use crate::{use_animated_open, use_id_or, use_unique_id};
use dioxus::prelude::*;

// TODO: controlled version
// TODO: rewrite this to use collapsible

/// Internal accordion context.
#[derive(Clone, Copy)]
struct AccordionContext {
    /// Used to track the next runtime-generated id.
    next_id: Signal<usize>,

    /// The runtime generated ids of the open items.
    open_items: Signal<Vec<usize>>,

    /// Whether multiple items can be open at once.
    allow_multiple_open: ReadSignal<bool>,

    /// Whether the entire accordion is disabled.
    disabled: ReadSignal<bool>,

    /// Whether all accordion items can be collapsed.
    collapsible: ReadSignal<bool>,

    /// Whether the accordion is horizontal.
    horizontal: ReadSignal<bool>,

    /// Roving focus state, keyed by the per-item runtime id.
    focus: FocusState,
}

impl AccordionContext {
    pub fn new(
        allow_multiple_open: ReadSignal<bool>,
        disabled: ReadSignal<bool>,
        collapsible: ReadSignal<bool>,
        horizontal: ReadSignal<bool>,
    ) -> Self {
        Self {
            next_id: Signal::new(0),
            open_items: Signal::new(Vec::new()),
            allow_multiple_open,
            disabled,
            collapsible,
            horizontal,
            focus: FocusState::new(ReadSignal::new(Signal::new(true))),
        }
    }

    pub fn register_item(&mut self) -> usize {
        let mut next_id = self.next_id.write();
        let id = *next_id;
        *next_id += 1;
        id
    }

    pub fn set_open(&mut self, id: usize) {
        if !*self.allow_multiple_open.peek() {
            self.open_items.clear();
        }
        self.open_items.push(id);
    }

    pub fn set_closed(&mut self, id: usize) {
        let mut open_items = self.open_items.write();

        // If the accordion is not collapsible, we can't close this one.
        if !*self.collapsible.peek() && open_items.len() == 1 {
            return;
        }

        *open_items = open_items
            .iter()
            .cloned()
            .filter(|item| *item != id)
            .collect();
    }

    pub fn is_open(&self, id: usize) -> bool {
        self.open_items.read().contains(&id)
    }

    pub fn is_disabled(&self) -> bool {
        (self.disabled)()
    }

    pub fn is_horizontal(&self) -> bool {
        (self.horizontal)()
    }
}

/// The props for the [`Accordion`] component.
#[derive(Props, Clone, PartialEq)]
pub struct AccordionProps {
    /// The id of the accordion root element.
    pub id: Option<String>,

    /// Whether multiple accordion items are allowed to be open at once.
    ///
    /// Defaults to false.
    #[props(default)]
    pub allow_multiple_open: ReadSignal<bool>,

    /// Set whether the accordion is disabled.
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// Whether the accordion can be fully collapsed.
    ///
    /// Setting this to true will allow all accordion items to close. Defaults to true.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub collapsible: ReadSignal<bool>,

    /// Whether the accordion is horizontal.
    ///
    /// Settings this to true will use left/right keybinds for navigation instead of up/down. Defaults to false.
    #[props(default)]
    pub horizontal: ReadSignal<bool>,

    /// Attributes to extend the root element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the accordion, which should contain [`AccordionItem`] components.
    pub children: Element,
}

/// # Accordion
///
/// The accordion component displays a list of collapsible items, allowing users to expand or collapse sections of content.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::accordion::{
///     Accordion, AccordionContent, AccordionItem, AccordionTrigger,
/// };
///
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         Accordion {
///             allow_multiple_open: false,
///             horizontal: false,
///             for i in 0..4 {
///                 AccordionItem {
///                     index: i,
///                     on_change: move |open| {
///                         tracing::info!("{open};");
///                     },
///                     on_trigger_click: move || {
///                         tracing::info!("trigger");
///                     },
///                     AccordionTrigger {
///                         "the quick brown fox"
///                     }
///                     AccordionContent {
///                         div { padding_bottom: "1rem",
///                             p {
///                                 padding: "0",
///                                 "Jumped over the lazy dog."
///                             }
///                         }
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
/// The [`Accordion`] component defines the following data attributes you can use to control styling:
/// - `data-disabled`: Indicates if the accordion is disabled. values are `true` or `false`.
#[component]
pub fn Accordion(props: AccordionProps) -> Element {
    let mut ctx = use_context_provider(|| {
        AccordionContext::new(
            props.allow_multiple_open,
            props.disabled,
            props.collapsible,
            props.horizontal,
        )
    });

    rsx! {
        div {
            id: props.id,
            "data-disabled": (props.disabled)(),

            onfocusout: move |_| {
                ctx.focus.blur();
            },

            ..props.attributes,

            {props.children}
        }
    }
}

/// The props for the [`AccordionItem`] component.
#[derive(Props, Clone, PartialEq)]
pub struct AccordionItemProps {
    /// Whether the accordion item is disabled.
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// Whether this accordion item should be opened by default.
    #[props(default)]
    pub default_open: bool,

    /// Callback for when the accordion's open/closed state changes.
    ///
    /// The new value is provided.
    #[props(default)]
    pub on_change: Callback<bool, ()>,

    /// Callback for when the trigger is clicked.
    #[props(default)]
    pub on_trigger_click: Callback,

    /// The index of the accordion item within the [`Accordion`].
    ///
    /// This is required to implement keyboard navigation and focus management.
    pub index: usize,

    /// Additional attributes to extend the item element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the accordion item.
    pub children: Element,
}

/// # Accordion Item
///
/// The accordion item component represents a single item within an accordion, which can be expanded or collapsed to show or hide its content.
///
/// The [`AccordionItem`] component must be used underneath the [`Accordion`] component.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::accordion::{
///     Accordion, AccordionContent, AccordionItem, AccordionTrigger,
/// };
///
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         Accordion {
///             AccordionItem {
///                 index: 0,
///                 AccordionTrigger {
///                     "the quick brown fox"
///                 }
///                 AccordionContent {
///                     "Jumped over the lazy dog."
///                 }
///             }
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`AccordionItem`] component defines the following data attributes you can use to control styling:
/// - `data-open`: Indicates if the accordion item is open. values are `true` or `false`.
/// - `data-disabled`: Indicates if the accordion is disabled. values are `true` or `false`.
#[component]
pub fn AccordionItem(props: AccordionItemProps) -> Element {
    let mut ctx: AccordionContext = use_context();
    let aria_id = use_unique_id();

    let item = use_context_provider(|| Item {
        id: ctx.register_item(),
        aria_id,
        disabled: props.disabled,
        on_trigger_click: props.on_trigger_click,
    });

    let disabled = move || ctx.is_disabled() || item.is_disabled();
    let id_signal = use_signal(|| item.id);
    use_focus_entry_disabled(ctx.focus, id_signal, disabled);

    // Open this item if we're set as default.
    use_hook(move || {
        if props.default_open {
            ctx.set_open(item.id);
        }
    });

    // Handle calling `on_change` callback.
    use_effect(move || {
        let open = ctx.is_open(item.id);
        props.on_change.call(open)
    });

    rsx! {
        div {
            "data-open": ctx.is_open(item.id),
            "data-disabled": ctx.is_disabled() || item.is_disabled(),
            ..props.attributes,

            {props.children}
        }
    }
}

/// The props for the [`AccordionContent`] component.
#[derive(Props, Clone, PartialEq)]
pub struct AccordionContentProps {
    /// The id of the accordion content element.
    pub id: ReadSignal<Option<String>>,
    /// Additional attributes to extend the content element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The children of the accordion content element.
    pub children: Element,
}

/// # Accordion Content
///
/// The accordion content component represents the content of an accordion item that can be
/// expanded or collapsed. The contents will only be displayed when the [`AccordionItem`] is open.
///
/// This must be used underneath the [`AccordionItem`] component.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::accordion::{
///     Accordion, AccordionContent, AccordionItem, AccordionTrigger,
/// };
///
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         Accordion {
///             AccordionItem {
///                 index: 0,
///                 AccordionTrigger {
///                     "the quick brown fox"
///                 }
///                 AccordionContent {
///                     "Jumped over the lazy dog."
///                 }
///             }
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`AccordionContent`] component defines the following data attributes you can use to control styling:
/// - `data-open`: Indicates if the accordion item is open. values are `true` or `false`.
#[component]
pub fn AccordionContent(props: AccordionContentProps) -> Element {
    let item: Item = use_context();
    let id = use_id_or(item.aria_id, props.id);
    let ctx: AccordionContext = use_context();
    let open = use_memo(move || ctx.is_open(item.id));

    let render_element = use_animated_open(id, open);

    rsx! {
        if render_element() {
            div {
                id: id,
                "data-open": open,
                ..props.attributes,

                {props.children}
            }
        }
    }
}

/// The props for the [`AccordionTrigger`] component.
#[derive(Props, Clone, PartialEq)]
pub struct AccordionTriggerProps {
    /// THe id of the accordion trigger element.
    pub id: Option<String>,
    /// Additional attributes to extend the trigger element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The children of the accordion trigger element.
    pub children: Element,
}

/// # Accordion Trigger
///
/// The accordion trigger component is a button that toggles the open/closed state of an [`AccordionItem`].
///
/// The [`AccordionTrigger`] component must be used underneath the [`AccordionItem`] component.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::accordion::{
///     Accordion, AccordionContent, AccordionItem, AccordionTrigger,
/// };
///
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         Accordion {
///             AccordionItem {
///                 index: 0,
///                 AccordionTrigger {
///                     "the quick brown fox"
///                 }
///                 AccordionContent {
///                     "Jumped over the lazy dog."
///                 }
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn AccordionTrigger(props: AccordionTriggerProps) -> Element {
    let mut ctx: AccordionContext = use_context();
    let item: Item = use_context();

    let disabled = move || ctx.is_disabled() || item.is_disabled();
    let id_signal = use_signal(|| item.id);
    let onmounted = use_focus_control_disabled(ctx.focus, id_signal, disabled);

    rsx! {
        button {
            id: props.id,
            disabled: disabled(),
            tabindex: "0",
            type: "button",

            aria_controls: item.aria_id(),
            aria_expanded: ctx.is_open(item.id),

            onmounted,
            onfocus: move |_| {
                ctx.focus.set_focus(Some(item.id));
            },
            onkeydown: move |event| {
                let key = event.key();
                let horizontal = ctx.is_horizontal();
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

            onclick: move |_| {
                if disabled() {
                    return;
                }
                item.on_trigger_click.call(());

                // If the item is not controlled, handle state.
                match ctx.is_open(item.id) {
                    true => ctx.set_closed(item.id),
                    false => ctx.set_open(item.id),
                }
            },

            ..props.attributes,

            {props.children}
        }
    }
}

/// Internal accordion-item context.
#[derive(Clone, Copy, PartialEq)]
struct Item {
    id: usize,
    aria_id: Signal<String>,
    disabled: ReadSignal<bool>,
    on_trigger_click: Callback,
}

impl Item {
    pub fn is_disabled(&self) -> bool {
        (self.disabled)()
    }

    pub fn aria_id(&self) -> String {
        (self.aria_id)()
    }
}
