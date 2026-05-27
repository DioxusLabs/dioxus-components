//! Defines the [`ContextMenu`] component and its subcomponents, which provide a context menu interface.

use crate::{
    focus::{use_focus_controlled_item_disabled, use_focus_provider, FocusState},
    selectable::{pointer_select_cancel, pointer_select_commit, pointer_select_start},
    use_animated_open, use_controlled, use_effect_with_cleanup, use_id_or, use_outside_dismiss,
    use_unique_id,
};
use dioxus::prelude::*;
use dioxus_core::Task;
use dioxus_sdk_time::sleep;
use std::time::Duration;

/// How long a touch must be held before the context menu opens.
const LONG_PRESS_DURATION: Duration = Duration::from_millis(500);
/// Pointer drift (in CSS pixels, squared) that cancels an in-flight long press.
const LONG_PRESS_MOVE_TOLERANCE_SQ: f64 = 100.0;

/// `position: fixed` needs layout-viewport coords. In Safari, `clientX/Y` is
/// visual-viewport relative (off by the pan offset under pinch-zoom), so we add
/// `visualViewport.offsetLeft/Top` — the same trick Floating UI does for
/// `strategy: 'fixed'` on WebKit. Chrome reports `clientX/Y` in layout-viewport
/// coords (matching `position: fixed`), so the correction must not be applied
/// there — it would double-count the pan offset and shift the menu by 2× the
/// pan distance.
async fn visual_viewport_offset() -> (f64, f64) {
    let mut eval = dioxus::document::eval(
        "const vv = window.visualViewport; \
         const ua = navigator.userAgent; \
         const isWebKit = ua.includes('AppleWebKit') && !ua.includes('Chrome'); \
         dioxus.send(isWebKit ? [vv ? vv.offsetLeft : 0, vv ? vv.offsetTop : 0] : [0, 0]);",
    );
    eval.recv::<(f64, f64)>().await.unwrap_or((0.0, 0.0))
}

#[derive(Clone, Copy)]
struct ContextMenuCtx {
    // State
    open: Memo<bool>,
    set_open: Callback<bool>,
    disabled: ReadSignal<bool>,

    // Position of the context menu
    position: Signal<(i32, i32)>,

    // Focus state
    focus: FocusState,

    // Id on the root wrapper — covers both trigger and content, so
    // `use_outside_dismiss` treats them as "inside".
    root_id: Signal<String>,

    // Set briefly after a touch long-press opens the menu. Used to (a) swallow
    // Android Chrome's spurious `contextmenu` ~500ms later, and (b) ignore the
    // pointer re-dispatch Android sends to whatever element is now under the
    // still-held finger — that would otherwise look like an instant tap on the
    // menu item the menu just rendered over.
    long_press_just_fired: Signal<bool>,
}

/// The props for the [`ContextMenu`] component.
#[derive(Props, Clone, PartialEq)]
pub struct ContextMenuProps {
    /// Whether the context menu is disabled
    #[props(default = ReadSignal::new(Signal::new(false)))]
    pub disabled: ReadSignal<bool>,

    /// Whether the context menu is open
    pub open: ReadSignal<Option<bool>>,

    /// Default open state
    #[props(default)]
    pub default_open: bool,

    /// Callback when open state changes
    #[props(default)]
    pub on_open_change: Callback<bool>,

    /// Whether focus should loop around when reaching the end.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub roving_loop: ReadSignal<bool>,

    /// Additional attributes for the context menu element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the context menu component.
    pub children: Element,
}

/// # ContextMenu
///
/// The [`ContextMenu`] component is a container that can be used to create a context menu. You can
/// use the [`ContextMenuTrigger`] to open the menu on a right-click, and the [`ContextMenuContent`] to define the menu item.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::context_menu::{
///     ContextMenu, ContextMenuContent, ContextMenuItem, ContextMenuTrigger,
/// };
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         ContextMenu {
///             ContextMenuTrigger {
///                 "right click here"
///             }
///             ContextMenuContent {
///                 ContextMenuItem {
///                     value: "edit".to_string(),
///                     index: 0usize,
///                     on_select: move |value| {
///                         tracing::info!("Selected item: {}", value);
///                     },
///                     "Edit"
///                 }
///                 ContextMenuItem {
///                     value: "undo".to_string(),
///                     index: 1usize,
///                     disabled: true,
///                     on_select: move |value| {
///                         tracing::info!("Selected item: {}", value);
///                     },
///                     "Undo"
///                 }
///             }
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`ContextMenu`] component defines the following data attributes you can use to control styling:
/// - `data-state`: Indicates if the state of the context menu. Values are `open` or `closed`.
/// - `data-disabled`: Indicates if the context menu is disabled. values are `true` or `false`.
#[component]
pub fn ContextMenu(props: ContextMenuProps) -> Element {
    let (open, set_open) = use_controlled(props.open, props.default_open, props.on_open_change);
    let position = use_signal(|| (0, 0));
    let root_id = use_unique_id();
    let long_press_just_fired = use_signal(|| false);

    let focus = use_focus_provider(props.roving_loop);
    let mut ctx = use_context_provider(|| ContextMenuCtx {
        open,
        set_open,
        disabled: props.disabled,
        position,
        focus,
        root_id,
        long_press_just_fired,
    });

    use_effect(move || {
        let focused = focus.any_focused();
        if *ctx.open.peek() != focused {
            (ctx.set_open)(focused);
        }
    });

    // Handle escape key to close the menu
    let handle_keydown = move |event: Event<KeyboardData>| {
        if open() && event.key() == Key::Escape {
            event.prevent_default();
            set_open.call(false);
            ctx.focus.blur();
        }
    };

    rsx! {
        div {
            id: root_id,
            tabindex: 0, // Make the menu container focusable
            onkeydown: handle_keydown,
            "data-state": if open() { "open" } else { "closed" },
            "data-disabled": (props.disabled)(),
            ..props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`ContextMenuTrigger`] component.
#[derive(Props, Clone, PartialEq)]
pub struct ContextMenuTriggerProps {
    /// Additional attributes for the context menu trigger element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the context menu trigger.
    pub children: Element,
}

/// # ContextMenuTrigger
///
/// The [`ContextMenuTrigger`] component is used to define the element that will trigger the context menu when right-clicked.
///
/// This must be used inside a [`ContextMenu`] component.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::context_menu::{
///     ContextMenu, ContextMenuContent, ContextMenuItem, ContextMenuTrigger,
/// };
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         ContextMenu {
///             ContextMenuTrigger {
///                 "right click here"
///             }
///             ContextMenuContent {
///                 ContextMenuItem {
///                     value: "edit".to_string(),
///                     index: 0usize,
///                     on_select: move |value| {
///                         tracing::info!("Selected item: {}", value);
///                     },
///                     "Edit"
///                 }
///                 ContextMenuItem {
///                     value: "undo".to_string(),
///                     index: 1usize,
///                     disabled: true,
///                     on_select: move |value| {
///                         tracing::info!("Selected item: {}", value);
///                     },
///                     "Undo"
///                 }
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn ContextMenuTrigger(props: ContextMenuTriggerProps) -> Element {
    let ctx: ContextMenuCtx = use_context();
    // iOS Safari does not deliver `contextmenu` from a long-press on touch, so
    // we run a manual timer keyed on the initial touch position and fire it
    // ourselves once the finger has held still long enough.
    let mut long_press_task: Signal<Option<Task>> = use_signal(|| None);
    let mut long_press_start: Signal<Option<(f64, f64)>> = use_signal(|| None);
    let mut long_press_just_fired = ctx.long_press_just_fired;

    let cancel_long_press =
        move |mut task: Signal<Option<Task>>, mut start: Signal<Option<(f64, f64)>>| {
            if let Some(t) = task.write().take() {
                t.cancel();
            }
            start.set(None);
        };

    let handle_context_menu = move |event: Event<MouseData>| {
        if !(ctx.disabled)() {
            // Android Chrome dispatches `contextmenu` ~500ms after a touch long
            // press, which can race our own timer. Defuse the race so only one
            // open lands.
            cancel_long_press(long_press_task, long_press_start);
            if long_press_just_fired.cloned() {
                // Timer already opened the menu; suppress the browser context
                // menu but don't open a second time. Leave the flag set —
                // the timer task clears it ~700ms after the menu opened, and
                // the menu items rely on that window to ignore Android's
                // re-dispatched pointerdown/up on the element under the
                // still-held finger.
                event.prevent_default();
                return;
            }
            let p = event.data().client_coordinates();
            let set_open = ctx.set_open;
            let mut position = ctx.position;
            spawn(async move {
                let (off_x, off_y) = visual_viewport_offset().await;
                position.set(((p.x + off_x) as i32, (p.y + off_y) as i32));
                set_open.call(true);
            });
            event.prevent_default();
        }
    };

    let handle_pointer_down = move |event: Event<PointerData>| {
        // Long-press fires for touch and pen (Apple Pencil etc.); mouse keeps
        // using the native `contextmenu` event.
        if event.pointer_type() == "mouse" || (ctx.disabled)() {
            return;
        }
        cancel_long_press(long_press_task, long_press_start);
        let p = event.client_coordinates();
        long_press_start.set(Some((p.x, p.y)));
        let set_open = ctx.set_open;
        let mut position = ctx.position;
        let task = spawn(async move {
            sleep(LONG_PRESS_DURATION).await;
            long_press_task.set(None);
            let (off_x, off_y) = visual_viewport_offset().await;
            position.set(((p.x + off_x) as i32, (p.y + off_y) as i32));
            set_open.call(true);
            // Stay armed long enough to catch Android's compat `contextmenu`,
            // then disarm so future mouse right-clicks aren't suppressed.
            long_press_just_fired.set(true);
            sleep(Duration::from_millis(700)).await;
            long_press_just_fired.set(false);
        });
        long_press_task.set(Some(task));
    };

    let handle_pointer_move = move |event: Event<PointerData>| {
        let Some((sx, sy)) = long_press_start.cloned() else {
            return;
        };
        let p = event.client_coordinates();
        let dx = p.x - sx;
        let dy = p.y - sy;
        if dx * dx + dy * dy > LONG_PRESS_MOVE_TOLERANCE_SQ {
            cancel_long_press(long_press_task, long_press_start);
        }
    };

    let handle_pointer_end = move |_event: Event<PointerData>| {
        cancel_long_press(long_press_task, long_press_start);
    };

    rsx! {
        div {
            oncontextmenu: handle_context_menu,
            onpointerdown: handle_pointer_down,
            onpointermove: handle_pointer_move,
            onpointerup: handle_pointer_end,
            onpointercancel: handle_pointer_end,
            role: "button",
            aria_haspopup: "menu",
            aria_expanded: (ctx.open)(),
            // Suppress iOS Safari's long-press behaviors (callout sheet, text
            // selection magnifier, gray tap-flash) and the system's own touch
            // gestures so our timer is the only thing that fires.
            style: "-webkit-touch-callout: none; user-select: none; -webkit-user-select: none; -webkit-tap-highlight-color: transparent; touch-action: none;",
            ..props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`ContextMenuContent`] component.
#[derive(Props, Clone, PartialEq)]
pub struct ContextMenuContentProps {
    /// The ID of the context menu content element.
    pub id: ReadSignal<Option<String>>,

    /// Additional attributes for the context menu content element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the context menu content.
    pub children: Element,
}

/// # ContextMenuContent
///
/// The [`ContextMenuContent`] component is used to define the content of the context menu. It is only rendered
/// when the context menu is open.
///
/// This must be used inside a [`ContextMenu`] component.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::context_menu::{
///     ContextMenu, ContextMenuContent, ContextMenuItem, ContextMenuTrigger,
/// };
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         ContextMenu {
///             ContextMenuTrigger {
///                 "right click here"
///             }
///             ContextMenuContent {
///                 ContextMenuItem {
///                     value: "edit".to_string(),
///                     index: 0usize,
///                     on_select: move |value| {
///                         tracing::info!("Selected item: {}", value);
///                     },
///                     "Edit"
///                 }
///                 ContextMenuItem {
///                     value: "undo".to_string(),
///                     index: 1usize,
///                     disabled: true,
///                     on_select: move |value| {
///                         tracing::info!("Selected item: {}", value);
///                     },
///                     "Undo"
///                 }
///             }
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`ContextMenuContent`] component defines the following data attributes you can use to control styling:
/// - `data-state`: Indicates if the state of the context menu. Values are `open` or `closed`.
#[component]
pub fn ContextMenuContent(props: ContextMenuContentProps) -> Element {
    let mut ctx: ContextMenuCtx = use_context();
    let position = ctx.position;
    let (x, y) = position();

    let open = ctx.open;

    let onkeydown = move |event: Event<KeyboardData>| {
        match event.key() {
            Key::Escape => ctx.focus.blur(),
            Key::ArrowDown => {
                ctx.focus.focus_next();
            }
            Key::ArrowUp => {
                if open() {
                    ctx.focus.focus_prev();
                }
            }
            Key::Home => ctx.focus.focus_first(),
            Key::End => ctx.focus.focus_last(),
            _ => return,
        }
        event.prevent_default();
    };

    let mut menu_ref: Signal<Option<std::rc::Rc<MountedData>>> = use_signal(|| None);
    let focused = move || open() && !ctx.focus.any_focused();
    // If the menu is open, but no item is focused, focus the div itself to capture events
    use_effect(move || {
        let Some(menu) = menu_ref() else {
            return;
        };
        if focused() {
            spawn(async move {
                // Focus the menu itself to capture keyboard events
                _ = menu.set_focus(true).await;
            });
        }
    });

    let unique_id = use_unique_id();
    let id = use_id_or(unique_id, props.id);

    let render = use_animated_open(id, open);

    use_outside_dismiss(ctx.root_id, move || {
        ctx.focus.blur();
        ctx.set_open.call(false);
    });

    // A `position: fixed` menu pinned to a click point drifts away from the
    // click target as soon as the page scrolls. Native context menus block
    // scroll while open; match that by suppressing wheel/touchmove outside
    // the menu without mutating page-level overflow styles.
    use_effect_with_cleanup(move || {
        if !open() {
            return Box::new(|| {}) as Box<dyn FnOnce()>;
        }
        let root = ctx.root_id;
        let eval = dioxus::document::eval(
            "const id = await dioxus.recv(); \
             const f = (e) => { \
                 const r = document.getElementById(id); \
                 if (!r || !r.contains(e.target)) e.preventDefault(); \
             }; \
             window.addEventListener('wheel', f, { capture: true, passive: false }); \
             window.addEventListener('touchmove', f, { capture: true, passive: false }); \
             await dioxus.recv(); \
             window.removeEventListener('wheel', f, true); \
             window.removeEventListener('touchmove', f, true);",
        );
        let _ = eval.send(root.cloned());
        Box::new(move || {
            let _ = eval.send(true);
        })
    });

    rsx! {
        if render() {
            div {
                id,
                role: "menu",
                aria_orientation: "vertical",
                position: "fixed",
                left: "{x}px",
                top: "{y}px",
                tabindex: if focused() { "0" } else { "-1" },
                pointer_events: open().then_some("auto"),
                "data-state": if open() { "open" } else { "closed" },
                onkeydown,
                onblur: move |_| {
                    if focused() {
                        ctx.focus.blur();
                    }
                },
                onmounted: move |evt| menu_ref.set(Some(evt.data())),
                ..props.attributes,

                {props.children}
            }
        }
    }
}

/// The props for the [`ContextMenuItem`] component.
#[derive(Props, Clone, PartialEq)]
pub struct ContextMenuItemProps {
    /// Whether the item is disabled
    #[props(default = ReadSignal::new(Signal::new(false)))]
    pub disabled: ReadSignal<bool>,

    /// The value of the menu item
    pub value: ReadSignal<String>,

    /// The index of the item in the menu
    pub index: ReadSignal<usize>,

    /// Callback when the item is selected
    #[props(default)]
    pub on_select: Callback<String>,

    /// Additional attributes for the context menu item element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the context menu item
    pub children: Element,
}

/// # ContextMenuItem
///
/// The [`ContextMenuItem`] component defines an individual item in the context menu. You must define an index that
/// controls the order items are focused when navigating the menu with the keyboard.
///
/// When an item is selected with either the pointer or the keyboard, the menu is closed and the `on_select` callback is called with the item's value.
///
/// This must be used inside a [`ContextMenuContent`] component.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::context_menu::{
///     ContextMenu, ContextMenuContent, ContextMenuItem, ContextMenuTrigger,
/// };
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         ContextMenu {
///             ContextMenuTrigger {
///                 "right click here"
///             }
///             ContextMenuContent {
///                 ContextMenuItem {
///                     value: "edit".to_string(),
///                     index: 0usize,
///                     on_select: move |value| {
///                         tracing::info!("Selected item: {}", value);
///                     },
///                     "Edit"
///                 }
///                 ContextMenuItem {
///                     value: "undo".to_string(),
///                     index: 1usize,
///                     disabled: true,
///                     on_select: move |value| {
///                         tracing::info!("Selected item: {}", value);
///                     },
///                     "Undo"
///                 }
///             }
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`ContextMenuItem`] component defines the following data attributes you can use to control styling:
/// - `data-disabled`: Indicates if the item is disabled. Possible values are `true` or `false`.
#[component]
pub fn ContextMenuItem(props: ContextMenuItemProps) -> Element {
    let mut ctx: ContextMenuCtx = use_context();

    let disabled = move || (props.disabled)() || (ctx.disabled)();
    let focused = move || ctx.focus.is_focused(props.index.cloned());

    let onmounted = use_focus_controlled_item_disabled(props.index, disabled);

    let tab_index = use_memo(move || if focused() { "0" } else { "-1" });

    // Touch sequences from the long-press that opened the menu shouldn't
    // count as selecting an item. Recording on pointerdown and committing on
    // pointerup means a pointerup without a matching pointerdown on this
    // item is ignored — exactly the long-press-then-lift case.
    let down_pos: Signal<Option<(f64, f64)>> = use_signal(|| None);
    let value = props.value;
    let mut select = move || {
        if !disabled() {
            props.on_select.call((value)());
            ctx.focus.blur();
            ctx.set_open.call(false);
        }
    };

    let handle_keydown = move |event: Event<KeyboardData>| {
        if event.key() == Key::Enter || event.key() == Key::Character(" ".to_string()) {
            select();
            event.prevent_default();
            event.stop_propagation();
        }
    };

    rsx! {
        div {
            role: "menuitem",
            tabindex: tab_index,
            onpointerdown: move |event| {
                pointer_select_start(&event, disabled(), down_pos);
            },
            onpointerup: move |event| {
                if pointer_select_commit(&event, disabled(), down_pos) {
                    select();
                    event.prevent_default();
                    event.stop_propagation();
                }
            },
            onpointercancel: move |_| {
                pointer_select_cancel(down_pos);
            },
            onkeydown: handle_keydown,
            onblur: move |_| {
                if focused() {
                    ctx.focus.blur();
                }
            },
            onmounted,
            aria_disabled: disabled(),
            "data-disabled": disabled(),
            ..props.attributes,

            {props.children}
        }
    }
}
