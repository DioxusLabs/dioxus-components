//! Defines the [`AlertDialogRoot`] component and its sub-components.

use crate::use_global_escape_listener;
use crate::{use_animated_open, use_focus_trap, use_id_or, use_unique_id, FOCUS_TRAP_JS};
use dioxus::document;
use dioxus::prelude::*;

#[derive(Clone)]
struct AlertDialogCtx {
    open: Memo<bool>,
    set_open: Callback<bool>,
    inert_background: ReadSignal<bool>,
    labelledby: String,
    describedby: String,
}

/// The props for the [`AlertDialogRoot`] component.
#[derive(Props, Clone, PartialEq)]
pub struct AlertDialogRootProps {
    /// The id of the alert dialog root element. If not provided, a unique id will be generated.
    pub id: ReadSignal<Option<String>>,
    /// Whether the alert dialog should be open by default. This is only used if the `open` signal is not provided.
    #[props(default)]
    pub default_open: bool,
    /// The open state of the alert dialog. If this is provided, it will be used to control the open state of the dialog.
    #[props(default)]
    pub open: ReadSignal<Option<bool>>,
    /// Callback to handle changes in the open state of the dialog.
    #[props(default)]
    pub on_open_change: Callback<bool>,
    /// Whether to mark the content outside of the alert dialog `inert` while it is open, which
    /// takes it out of the accessibility tree and makes it unreachable by pointer and by
    /// programmatic focus. Defaults to true; set it to false if the application manages `inert`
    /// itself.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub inert_background: ReadSignal<bool>,
    /// Additional attributes to extend the root element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The children of the alert dialog root element.
    pub children: Element,
}

/// # AlertDialogRoot
///
/// The entry point for the alert dialog. It manages the open state of the dialog and provides context to its children. You
/// can use it to create a backdrop for the dialog if needed. The contents will only be rendered when the dialog is open.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::alert_dialog::*;
///
/// #[component]
/// fn Demo() -> Element {
///     let mut open = use_signal(|| false);
///
///     rsx! {
///         button {
///             onclick: move |_| open.set(true),
///             "Show Alert Dialog"
///         }
///         AlertDialogRoot {
///             open: open(),
///             on_open_change: move |v| open.set(v),
///             AlertDialogContent {
///                 AlertDialogTitle { "Delete item" }
///                 AlertDialogDescription { "Are you sure you want to delete this item? This action cannot be undone." }
///                 AlertDialogActions {
///                     AlertDialogCancel { "Cancel" }
///                     AlertDialogAction {
///                         on_click: move |_| tracing::info!("Item deleted"),
///                         "Delete"
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
/// The [`AlertDialogRoot`] component defines the following data attributes you can use to control styling:
/// - `data-state`: Indicates if the alert dialog is open or closed. It can be either "open" or "closed".
///
/// ## Accessibility
///
/// While the alert dialog is open, the content outside of it is marked `inert`: it is removed from
/// the accessibility tree and cannot be reached by a pointer or by a programmatic `focus()`. Every
/// element marked this way also carries a `data-inert-by` attribute naming the dialogs that marked
/// it, so stacked dialogs unwind independently, and `inert` the application had already set
/// before the dialog opened is left alone. Set `inert_background` to false to opt out.
#[component]
pub fn AlertDialogRoot(props: AlertDialogRootProps) -> Element {
    let labelledby = use_unique_id().to_string();
    let describedby = use_unique_id().to_string();
    let mut open_signal = use_signal(|| props.default_open);
    let set_open = use_callback(move |v: bool| {
        open_signal.set(v);
        props.on_open_change.call(v);
    });
    let open = use_memo(move || (props.open)().unwrap_or_else(&*open_signal));
    use_context_provider(|| AlertDialogCtx {
        open,
        set_open,
        inert_background: props.inert_background,
        labelledby,
        describedby,
    });

    let id = use_unique_id();
    let id = use_id_or(id, props.id);
    let render_element = use_animated_open(id, open);

    rsx! {
        document::Script {
            src: FOCUS_TRAP_JS,
            defer: true
        }
        if render_element() {
            div {
                id,
                "data-state": if open() { "open" } else { "closed" },
                ..props.attributes,
                {props.children}
            }
        }
    }
}

/// The props for the [`AlertDialogContent`] component.
#[derive(Props, Clone, PartialEq)]
pub struct AlertDialogContentProps {
    /// The id of the alert dialog content element. If not provided, a unique id will be generated.
    pub id: ReadSignal<Option<String>>,

    /// The class to apply to the alert dialog content element.
    #[props(default)]
    pub class: Option<String>,

    /// Additional attributes to extend the content element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The children of the alert dialog content element.
    pub children: Element,
}

/// # AlertDialogContent
///
/// The content of the alert dialog. Any interactive content in the dialog should be placed
/// inside this component. It will trap focus within the dialog while it is open
///
/// This must be used inside an [`AlertDialogRoot`] component.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::alert_dialog::*;
///
/// #[component]
/// fn Demo() -> Element {
///     let mut open = use_signal(|| false);
///
///     rsx! {
///         button {
///             onclick: move |_| open.set(true),
///             "Show Alert Dialog"
///         }
///         AlertDialogRoot {
///             open: open(),
///             on_open_change: move |v| open.set(v),
///             AlertDialogContent {
///                 AlertDialogTitle { "Delete item" }
///                 AlertDialogDescription { "Are you sure you want to delete this item? This action cannot be undone." }
///                 AlertDialogActions {
///                     AlertDialogCancel { "Cancel" }
///                     AlertDialogAction {
///                         on_click: move |_| tracing::info!("Item deleted"),
///                         "Delete"
///                     }
///                 }
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn AlertDialogContent(props: AlertDialogContentProps) -> Element {
    let ctx: AlertDialogCtx = use_context();

    let open = ctx.open;
    let set_open = ctx.set_open;
    let inert_background = ctx.inert_background;

    // Add a escape key listener to the document when the dialog is open. We can't
    // just add this to the dialog itself because it might not be focused if the user
    // is highlighting text or interacting with another element.
    use_global_escape_listener(move || set_open.call(false));

    let gen_id = use_unique_id();
    let id = use_id_or(gen_id, props.id);

    use_focus_trap(id, open, inert_background);

    rsx! {
        div {
            id,
            role: "alertdialog",
            aria_modal: "true",
            aria_labelledby: ctx.labelledby.clone(),
            aria_describedby: ctx.describedby.clone(),
            class: props.class.clone().unwrap_or_else(|| "dx-alert-dialog".to_string()),
            ..props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`AlertDialogTitle`] component.
#[derive(Props, Clone, PartialEq)]
pub struct AlertDialogTitleProps {
    /// Additional attributes to extend the title element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The children of the title element.
    pub children: Element,
}

/// # AlertDialogTitle
///
/// The title of the alert dialog. This will be used to label the dialog for accessibility purposes.
///
/// This must be used inside an [`AlertDialogRoot`] component and should be placed inside an [`AlertDialogContent`] component.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::alert_dialog::*;
///
/// #[component]
/// fn Demo() -> Element {
///     let mut open = use_signal(|| false);
///
///     rsx! {
///         button {
///             onclick: move |_| open.set(true),
///             "Show Alert Dialog"
///         }
///         AlertDialogRoot {
///             open: open(),
///             on_open_change: move |v| open.set(v),
///             AlertDialogContent {
///                 AlertDialogTitle { "Delete item" }
///                 AlertDialogDescription { "Are you sure you want to delete this item? This action cannot be undone." }
///                 AlertDialogActions {
///                     AlertDialogCancel { "Cancel" }
///                     AlertDialogAction {
///                         on_click: move |_| tracing::info!("Item deleted"),
///                         "Delete"
///                     }
///                 }
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn AlertDialogTitle(props: AlertDialogTitleProps) -> Element {
    let ctx: AlertDialogCtx = use_context();
    rsx! {
        h2 { id: ctx.labelledby.clone(), ..props.attributes, {props.children} }
    }
}

/// The props for the [`AlertDialogDescription`] component.
#[derive(Props, Clone, PartialEq)]
pub struct AlertDialogDescriptionProps {
    /// Additional attributes to extend the description element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The children of the description element.
    pub children: Element,
}

/// # AlertDialogDescription
///
/// The description of the alert dialog. This will be used to describe the dialog for accessibility purposes.
///
/// This must be used inside an [`AlertDialogRoot`] component and should be placed inside an [`AlertDialogContent`] component.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::alert_dialog::*;
///
/// #[component]
/// fn Demo() -> Element {
///     let mut open = use_signal(|| false);
///
///     rsx! {
///         button {
///             onclick: move |_| open.set(true),
///             "Show Alert Dialog"
///         }
///         AlertDialogRoot {
///             open: open(),
///             on_open_change: move |v| open.set(v),
///             AlertDialogContent {
///                 AlertDialogTitle { "Delete item" }
///                 AlertDialogDescription { "Are you sure you want to delete this item? This action cannot be undone." }
///                 AlertDialogActions {
///                     AlertDialogCancel { "Cancel" }
///                     AlertDialogAction {
///                         on_click: move |_| tracing::info!("Item deleted"),
///                         "Delete"
///                     }
///                 }
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn AlertDialogDescription(props: AlertDialogDescriptionProps) -> Element {
    let ctx: AlertDialogCtx = use_context();
    rsx! {
        p { id: ctx.describedby.clone(), ..props.attributes, {props.children} }
    }
}

/// The props for the [`AlertDialogActions`] component.
#[derive(Props, Clone, PartialEq)]
pub struct AlertDialogActionsProps {
    /// Additional attributes to extend the actions element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The children of the actions element.
    pub children: Element,
}

/// # AlertDialogActions
///
/// The actions of the alert dialog. This will be used to group the actions.
///
/// This must be used inside an [`AlertDialogRoot`] component and should be placed inside an [`AlertDialogContent`] component.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::alert_dialog::*;
///
/// #[component]
/// fn Demo() -> Element {
///     let mut open = use_signal(|| false);
///
///     rsx! {
///         button {
///             onclick: move |_| open.set(true),
///             "Show Alert Dialog"
///         }
///         AlertDialogRoot {
///             open: open(),
///             on_open_change: move |v| open.set(v),
///             AlertDialogContent {
///                 AlertDialogTitle { "Delete item" }
///                 AlertDialogDescription { "Are you sure you want to delete this item? This action cannot be undone." }
///                 AlertDialogActions {
///                     AlertDialogCancel { "Cancel" }
///                     AlertDialogAction {
///                         on_click: move |_| tracing::info!("Item deleted"),
///                         "Delete"
///                     }
///                 }
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn AlertDialogActions(props: AlertDialogActionsProps) -> Element {
    rsx! {
        div { ..props.attributes, {props.children} }
    }
}

/// The props for the [`AlertDialogAction`] component.
#[derive(Props, Clone, PartialEq)]
pub struct AlertDialogActionProps {
    /// The click event handler for the action button.
    #[props(default)]
    pub on_click: Option<EventHandler<MouseEvent>>,
    /// Additional attributes to extend the action button element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The children of the action button.
    pub children: Element,
}

/// # AlertDialogAction
///
/// An action button for the alert dialog. In addition to running the `on_click` callback, it will also close the dialog when clicked.
///
/// This must be used inside an [`AlertDialogRoot`] component and should be placed inside an [`AlertDialogContent`] component.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::alert_dialog::*;
///
/// #[component]
/// fn Demo() -> Element {
///     let mut open = use_signal(|| false);
///
///     rsx! {
///         button {
///             onclick: move |_| open.set(true),
///             "Show Alert Dialog"
///         }
///         AlertDialogRoot {
///             open: open(),
///             on_open_change: move |v| open.set(v),
///             AlertDialogContent {
///                 AlertDialogTitle { "Delete item" }
///                 AlertDialogDescription { "Are you sure you want to delete this item? This action cannot be undone." }
///                 AlertDialogActions {
///                     AlertDialogCancel { "Cancel" }
///                     AlertDialogAction {
///                         on_click: move |_| tracing::info!("Item deleted"),
///                         "Delete"
///                     }
///                 }
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn AlertDialogAction(props: AlertDialogActionProps) -> Element {
    let ctx: AlertDialogCtx = use_context();
    let open = ctx.open;
    let set_open = ctx.set_open;
    let user_on_click = props.on_click;
    let on_click = use_callback(move |evt: MouseEvent| {
        set_open.call(false);
        if let Some(cb) = &user_on_click {
            cb.call(evt.clone());
        }
    });
    rsx! {
        button {
            tabindex: if open() { "0" } else { "-1" },
            type: "button",
            onclick: on_click,
            ..props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`AlertDialogCancel`] component.
#[derive(Props, Clone, PartialEq)]
pub struct AlertDialogCancelProps {
    /// The click event handler for the cancel button.
    #[props(default)]
    pub on_click: Option<EventHandler<MouseEvent>>,
    /// Additional attributes to extend the cancel button element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The children of the cancel button.
    pub children: Element,
}

/// # AlertDialogCancel
///
/// An cancel button for the alert dialog. In addition to running the `on_click` callback, it will also close the dialog when clicked.
///
/// This must be used inside an [`AlertDialogRoot`] component and should be placed inside an [`AlertDialogContent`] component.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::alert_dialog::*;
///
/// #[component]
/// fn Demo() -> Element {
///     let mut open = use_signal(|| false);
///
///     rsx! {
///         button {
///             onclick: move |_| open.set(true),
///             "Show Alert Dialog"
///         }
///         AlertDialogRoot {
///             open: open(),
///             on_open_change: move |v| open.set(v),
///             AlertDialogContent {
///                 AlertDialogTitle { "Delete item" }
///                 AlertDialogDescription { "Are you sure you want to delete this item? This action cannot be undone." }
///                 AlertDialogActions {
///                     AlertDialogCancel { "Cancel" }
///                     AlertDialogAction {
///                         on_click: move |_| tracing::info!("Item deleted"),
///                         "Delete"
///                     }
///                 }
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn AlertDialogCancel(props: AlertDialogCancelProps) -> Element {
    let ctx: AlertDialogCtx = use_context();
    let open = ctx.open;
    let set_open = ctx.set_open;
    let user_on_click = props.on_click;
    let on_click = use_callback(move |evt: MouseEvent| {
        set_open.call(false);
        if let Some(cb) = &user_on_click {
            cb.call(evt.clone());
        }
    });

    rsx! {
        button {
            tabindex: if open() { "0" } else { "-1" },
            type: "button",
            onclick: on_click,
            ..props.attributes,
            {props.children}
        }
    }
}
