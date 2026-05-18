//! Defines the [`PopoverRoot`] component and its sub-components.

use dioxus::document;
use dioxus::prelude::*;

use crate::{
    use_animated_open, use_controlled, use_global_escape_listener, use_id_or, use_outside_dismiss,
    use_unique_id, ContentAlign, ContentSide, FOCUS_TRAP_JS,
};

#[derive(Clone, Copy)]
struct PopoverCtx {
    #[allow(unused)]
    open: Memo<bool>,
    #[allow(unused)]
    set_open: Callback<bool>,

    // Whether the dialog is a modal and should capture focus.
    #[allow(unused)]
    is_modal: ReadSignal<bool>,
    labelledby: Signal<String>,
    root_id: Memo<String>,
}

/// The props for the [`PopoverRoot`] component.
#[derive(Props, Clone, PartialEq)]
pub struct PopoverRootProps {
    /// Whether the popover is a modal and should capture focus.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub is_modal: ReadSignal<bool>,

    /// The controlled open state of the popover.
    pub open: ReadSignal<Option<bool>>,

    /// The default open state when uncontrolled.
    #[props(default)]
    pub default_open: bool,

    /// Callback fired when the open state changes.
    #[props(default)]
    pub on_open_change: Callback<bool>,

    /// The id of the popover root element.
    pub id: ReadSignal<Option<String>>,

    /// Additional attributes to apply to the popover root element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the popover root component.
    pub children: Element,
}

/// # PopoverRoot
///
/// The `PopoverRoot` component wraps all the popover components and manages the state. You can define a
/// [`PopoverTrigger`] component to toggle the popover's open state, and a [`PopoverContent`] component
/// to define the content that appears when the popover is open under this component.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_kit_core::popover::*;
///
/// #[component]
/// fn Demo() -> Element {
///     let mut open = use_signal(|| false);
///
///     rsx! {
///         PopoverRoot {
///             open: open(),
///             on_open_change: move |v| open.set(v),
///             PopoverTrigger {
///                 "Show Popover"
///             }
///             PopoverContent {
///                 gap: "0.25rem",
///                 h3 {
///                     padding_top: "0.25rem",
///                     padding_bottom: "0.25rem",
///                     width: "100%",
///                     text_align: "center",
///                     margin: 0,
///                     "Delete Item?"
///                 }
///                 button {
///                     onclick: move |_| {
///                         open.set(false);;
///                     },
///                     "Yes!"
///                 }
///             }
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`PopoverRoot`] component defines the following data attributes you can use to control styling:
/// - `data-state`: Indicates if the popover is open or closed. Values are `open` or `closed`.
#[component]
pub fn PopoverRoot(props: PopoverRootProps) -> Element {
    let labelledby = use_unique_id();
    let gen_root_id = use_unique_id();
    let root_id = use_id_or(gen_root_id, props.id);

    let (open, set_open) = use_controlled(props.open, props.default_open, props.on_open_change);

    use_context_provider(|| PopoverCtx {
        open,
        set_open,
        is_modal: props.is_modal,
        labelledby,
        root_id,
    });

    rsx! {
        div {
            id: root_id,
            "data-state": if open() { "open" } else { "closed" },
            ..props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`PopoverContent`] component.
#[derive(Props, Clone, PartialEq)]
pub struct PopoverContentProps {
    /// The id of the popover content element.
    pub id: ReadSignal<Option<String>>,

    /// CSS class for the popover content.
    #[props(default)]
    pub class: Option<String>,

    /// Side of the trigger to place the popover.
    #[props(default = ContentSide::Bottom)]
    pub side: ContentSide,

    /// Alignment of the popover relative to the trigger.
    #[props(default = ContentAlign::Center)]
    pub align: ContentAlign,

    /// Additional attributes to apply to the content element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the popover content component.
    pub children: Element,
}

/// # PopoverContent
///
/// The `PopoverContent` component defines the content of the popover. This component will
/// only be rendered if the popover is open, and it will handle focus trapping if the popover is modal.
///
/// This must be used inside a [`PopoverRoot`] component.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_kit_core::popover::*;
///
/// #[component]
/// fn Demo() -> Element {
///     let mut open = use_signal(|| false);
///
///     rsx! {
///         PopoverRoot {
///             open: open(),
///             on_open_change: move |v| open.set(v),
///             PopoverTrigger {
///                 "Show Popover"
///             }
///             PopoverContent {
///                 gap: "0.25rem",
///                 h3 {
///                     padding_top: "0.25rem",
///                     padding_bottom: "0.25rem",
///                     width: "100%",
///                     text_align: "center",
///                     margin: 0,
///                     "Delete Item?"
///                 }
///                 button {
///                     onclick: move |_| {
///                         open.set(false);;
///                     },
///                     "Yes!"
///                 }
///             }
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`PopoverContent`] component defines the following data attributes you can use to control styling:
/// - `data-state`: Indicates if the popover is open or closed. Values are `open` or `closed`.
/// - `data-side`: Indicates the side where the popover is positioned relative to the trigger. Possible values are `top`, `right`, `bottom`, and `left`.
/// - `data-align`: Indicates the alignment of the popover relative to the trigger. Possible values are `start`, `center`, and `end`.
#[component]
pub fn PopoverContent(props: PopoverContentProps) -> Element {
    let ctx: PopoverCtx = use_context();
    let open = ctx.open;
    let is_modal = ctx.is_modal;

    let gen_id = use_unique_id();
    let id = use_id_or(gen_id, props.id);

    let render = use_animated_open(id, ctx.open);

    use_effect(move || {
        if !render() {
            return;
        }
        let is_modal = is_modal();
        if !is_modal {
            // If the dialog is not modal, we don't need to trap focus.
            return;
        }

        let eval = document::eval(
            r#"let id = await dioxus.recv();
            let is_open = await dioxus.recv();
            let dialog = document.getElementById(id);

            if (is_open) {
                dialog.trap = window.createFocusTrap(dialog);
            }
            if (!is_open && dialog.trap) {
                dialog.trap.remove();
                dialog.trap = null;
            }"#,
        );
        let _ = eval.send(id.to_string());
        let _ = eval.send(open.cloned());
    });

    rsx! {
        document::Script {
            src: FOCUS_TRAP_JS,
            defer: true
        }
        if render() {
            PopoverContentRendered {
                id,
                class: props.class,
                side: props.side,
                align: props.align,
                attributes: props.attributes,
                children: props.children
            }
        }
    }
}

/// The rendered content of the popover. This is separated out so the global event listener
/// is only added when the popover is actually rendered.
#[component]
pub fn PopoverContentRendered(
    id: String,
    class: Option<String>,
    side: ContentSide,
    align: ContentAlign,
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctx: PopoverCtx = use_context();
    let open = ctx.open;
    let is_open = open();
    let set_open = ctx.set_open;

    // Add a escape key listener to the document when the popover is open. We can't
    // just add this to the popover itself because it might not be focused if the user
    // is highlighting text or interacting with another element.
    use_global_escape_listener(move || set_open.call(false));

    use_outside_dismiss(ctx.root_id, move || set_open.call(false));

    rsx! {
        div {
            id,
            role: "dialog",
            aria_modal: (ctx.is_modal)().then_some("true"),
            aria_labelledby: ctx.labelledby,
            aria_hidden: (!is_open).then_some("true"),
            class: class.unwrap_or_else(|| "dx-popover-content".to_string()),
            "data-state": if is_open { "open" } else { "closed" },
            "data-side": side.as_str(),
            "data-align": align.as_str(),
            ..attributes,
            {children}
        }
    }
}

/// The props for the [`PopoverTrigger`] component.
#[derive(Props, Clone, PartialEq)]
pub struct PopoverTriggerProps {
    /// Additional attributes to apply to the trigger element.
    #[props(extends = GlobalAttributes)]
    #[props(extends = button)]
    pub attributes: Vec<Attribute>,

    /// The children of the trigger component.
    pub children: Element,
}

/// # PopoverTrigger
///
/// The `PopoverTrigger` is a button that toggles the visibility of the [`PopoverContent`].
///
/// This must be used inside a [`PopoverRoot`] component.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_kit_core::popover::*;
///
/// #[component]
/// fn Demo() -> Element {
///     let mut open = use_signal(|| false);
///
///     rsx! {
///         PopoverRoot {
///             open: open(),
///             on_open_change: move |v| open.set(v),
///             PopoverTrigger {
///                 "Show Popover"
///             }
///             PopoverContent {
///                 gap: "0.25rem",
///                 h3 {
///                     padding_top: "0.25rem",
///                     padding_bottom: "0.25rem",
///                     width: "100%",
///                     text_align: "center",
///                     margin: 0,
///                     "Delete Item?"
///                 }
///                 button {
///                     onclick: move |_| {
///                         open.set(false);;
///                     },
///                     "Yes!"
///                 }
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn PopoverTrigger(props: PopoverTriggerProps) -> Element {
    let ctx: PopoverCtx = use_context();
    let mut id = ctx.labelledby;
    let id_attribute = props
        .attributes
        .iter()
        .find(|attr| attr.name == "id")
        .cloned();
    use_effect(use_reactive!(|id_attribute| {
        if let Some(id_attribute) = id_attribute {
            match &id_attribute.value {
                dioxus_core::AttributeValue::Text(val) => id.set(val.to_string()),
                dioxus_core::AttributeValue::Float(val) => id.set(val.to_string()),
                dioxus_core::AttributeValue::Int(val) => id.set(val.to_string()),
                dioxus_core::AttributeValue::Bool(val) => id.set(val.to_string()),
                _ => {}
            }
        }
    }));

    rsx! {
        button {
            id,
            type: "button",
            onclick: move |e| {
                e.stop_propagation();
                ctx.set_open.call(!(ctx.open)());
            },
            ..props.attributes,
            {props.children}
        }
    }
}
