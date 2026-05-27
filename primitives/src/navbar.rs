//! Defines the [`Navbar`] component and its sub-components.

use crate::{
    focus::{
        use_deferred_focus, use_focus_control, use_focus_controlled_item_disabled,
        use_focus_entry_disabled, use_focus_provider, FocusPlacement, FocusState,
    },
    use_animated_open, use_id_or, use_unique_id,
};
use dioxus::prelude::*;

#[derive(Clone, Copy)]
struct NavbarContext {
    // Currently open nav index
    open_nav: Signal<Option<usize>>,
    set_open_nav: Callback<Option<usize>>,
    disabled: ReadSignal<bool>,

    // Focus state
    focus: FocusState,
}

/// The props for the [`Navbar`] component.
#[derive(Props, Clone, PartialEq)]
pub struct NavbarProps {
    /// Whether the navbar is disabled.
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// Whether focus should loop around when reaching the end.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub roving_loop: ReadSignal<bool>,

    /// Additional attributes to apply to the navbar element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The children of the navbar component.
    pub children: Element,
}

/// # Navbar
///
/// The `Navbar` component creates a navigation bar that allows users to navigate through different sections with keyboard and pointer interactions.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::navbar::{Navbar, NavbarContent, NavbarItem, NavbarNav, NavbarTrigger};
///
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         Navbar {
///             aria_label: "Components",
///             NavbarNav { index: 0usize,
///                 NavbarTrigger {
///                     "Inputs"
///                 }
///                 NavbarContent {
///                     NavbarItem {
///                         index: 0usize,
///                         value: "calendar".to_string(),
///                         to: "https://dioxuslabs.github.io/components/component/?name=calendar",
///                         "Calendar"
///                     }
///                     NavbarItem {
///                         index: 1usize,
///                         value: "slider".to_string(),
///                         to: "https://dioxuslabs.github.io/components/component/?name=slider",
///                         "Slider"
///                     }
///                 }
///             }
///             NavbarNav { index: 1usize,
///                 NavbarTrigger {
///                     "Information"
///                 }
///                 NavbarContent {
///                     NavbarItem {
///                         index: 0usize,
///                         value: "toast".to_string(),
///                         to: "https://dioxuslabs.github.io/components/component/?name=toast",
///                         "Toast"
///                     }
///                     NavbarItem {
///                         index: 1usize,
///                         value: "tabs".to_string(),
///                         to: "https://dioxuslabs.github.io/components/component/?name=tabs",
///                         "Tabs"
///                     }
///                 }
///             }
///             NavbarItem {
///                 index: 2usize,
///                 value: "home".to_string(),
///                 to: "https://dioxuslabs.github.io/components",
///                 "Home"
///             }
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`Navbar`] component defines the following data attributes you can use to control styling:
/// - `data-disabled`: Indicates if the navbar is disabled. Values are `true` or `false`.
#[component]
pub fn Navbar(props: NavbarProps) -> Element {
    let mut open_nav = use_signal(|| None);
    let set_open_nav = use_callback(move |idx| open_nav.set(idx));

    let focus = use_focus_provider(props.roving_loop);
    let mut ctx = use_context_provider(|| NavbarContext {
        open_nav,
        set_open_nav,
        disabled: props.disabled,
        focus,
    });
    use_effect(move || {
        let index = ctx.focus.current_focus();
        if ctx.open_nav.peek().is_some() {
            ctx.set_open_nav.call(index);
        }
    });

    let aria_label = props
        .attributes
        .iter()
        .find_map(|attr| (attr.name == "aria-label").then(|| attr.value.clone()));

    rsx! {
        div {
            role: "navigation",
            display: "content",
            aria_label,
            div {
                role: "menubar",
                "data-disabled": (props.disabled)(),
                tabindex: (!ctx.focus.any_focused()).then_some("0"),
                // If the menu receives focus, focus the most recently focused menu item
                onfocus: move |_| {
                    ctx.focus.set_focus(Some(ctx.focus.recent_focus_or_default()));
                },
                onkeydown: move |event: Event<KeyboardData>| {
                    match event.key() {
                        Key::Escape => ctx.set_open_nav.call(None),
                        Key::ArrowLeft => ctx.focus.focus_prev(),
                        Key::ArrowRight => ctx.focus.focus_next(),
                        Key::Home => ctx.focus.focus_first(),
                        Key::End => ctx.focus.focus_last(),
                        _ => return,
                    }
                    event.prevent_default();
                },

                ..props.attributes,

                {props.children}
            }
        }
    }
}

#[derive(Clone, Copy)]
struct NavbarNavContext {
    index: ReadSignal<usize>,
    focus: FocusState,
    is_open: Memo<bool>,
    disabled: ReadSignal<bool>,
    initial_focus: Signal<Option<FocusPlacement>>,
}

impl NavbarNavContext {
    fn focus_next(&mut self) {
        self.focus.focus_next();
    }

    fn focus_prev(&mut self) {
        self.focus.focus_prev();
    }
}

/// The props for the [`NavbarNav`] component.
#[derive(Props, Clone, PartialEq)]
pub struct NavbarNavProps {
    /// The index of this nav item in the navbar. This is used to define the focus order for keyboard navigation.
    pub index: ReadSignal<usize>,

    /// Whether this nav item is disabled.
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// Additional attributes to apply to the nav element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The children of the nav component.
    pub children: Element,
}

/// # NavbarNav
///
/// The `NavbarNav` component represents a single navigation dropdown within a navbar. It contains a [`NavbarTrigger`] and [`NavbarContent`] that can be opened and closed.
///
/// This must be used inside a [`Navbar`] component.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::navbar::{Navbar, NavbarContent, NavbarItem, NavbarNav, NavbarTrigger};
///
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         Navbar {
///             aria_label: "Components",
///             NavbarNav { index: 0usize,
///                 NavbarTrigger {
///                     "Inputs"
///                 }
///                 NavbarContent {
///                     NavbarItem {
///                         index: 0usize,
///                         value: "calendar".to_string(),
///                         to: "https://dioxuslabs.github.io/components/component/?name=calendar",
///                         "Calendar"
///                     }
///                     NavbarItem {
///                         index: 1usize,
///                         value: "slider".to_string(),
///                         to: "https://dioxuslabs.github.io/components/component/?name=slider",
///                         "Slider"
///                     }
///                 }
///             }
///             NavbarNav { index: 1usize,
///                 NavbarTrigger {
///                     "Information"
///                 }
///                 NavbarContent {
///                     NavbarItem {
///                         index: 0usize,
///                         value: "toast".to_string(),
///                         to: "https://dioxuslabs.github.io/components/component/?name=toast",
///                         "Toast"
///                     }
///                     NavbarItem {
///                         index: 1usize,
///                         value: "tabs".to_string(),
///                         to: "https://dioxuslabs.github.io/components/component/?name=tabs",
///                         "Tabs"
///                     }
///                 }
///             }
///             NavbarItem {
///                 index: 2usize,
///                 value: "home".to_string(),
///                 to: "https://dioxuslabs.github.io/components",
///                 "Home"
///             }
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`NavbarNav`] component defines the following data attributes you can use to control styling:
/// - `data-state`: Indicates if the nav is open or closed. Values are `open` or `closed`.
/// - `data-disabled`: Indicates if the nav is disabled. Values are `true` or `false`.
#[component]
pub fn NavbarNav(props: NavbarNavProps) -> Element {
    let mut ctx: NavbarContext = use_context();
    let is_open = use_memo(move || (ctx.open_nav)() == Some(props.index.cloned()));
    let focus = use_focus_provider(ctx.focus.roving_loop);
    let initial_focus = use_signal(|| None);
    let mut nav_ctx = use_context_provider(|| NavbarNavContext {
        index: props.index,
        focus,
        is_open,
        disabled: props.disabled,
        initial_focus,
    });

    use_effect(move || {
        if !is_open() {
            nav_ctx.focus.blur();
            nav_ctx.initial_focus.set(None);
        }
    });

    let disabled = move || (ctx.disabled)() || (props.disabled)();
    use_focus_entry_disabled(ctx.focus, nav_ctx.index, disabled);

    rsx! {
        div {
            role: "menu",
            "data-state": if is_open() { "open" } else { "closed" },
            "data-disabled": (ctx.disabled)() || (props.disabled)(),

            onmouseenter: move |_| {
                if !disabled() {
                    let index = Some(nav_ctx.index.cloned());
                    if (ctx.open_nav)().is_some() {
                        ctx.focus.set_focus(index);
                    } else {
                        ctx.set_open_nav.call(index);
                    }
                }
            },
            onmouseleave: move |_| {
                if is_open() {
                    ctx.focus.set_focus(None);
                }
            },
            onkeydown: move |event: Event<KeyboardData>| {
                match event.key() {
                    Key::Enter if !disabled() => {
                        ctx.set_open_nav.call((!is_open()).then(&*props.index));
                    }
                    Key::ArrowDown if !disabled() => {
                        if !is_open() {
                            nav_ctx.initial_focus.set(Some(FocusPlacement::First));
                            ctx.set_open_nav.call(Some(props.index.cloned()));
                        } else {
                            nav_ctx.focus_next();
                        }
                    },
                    Key::ArrowUp if !disabled() => {
                        if is_open() {
                            nav_ctx.focus_prev();
                        } else {
                            nav_ctx.initial_focus.set(Some(FocusPlacement::Last));
                            ctx.set_open_nav.call(Some(props.index.cloned()));
                        }
                    },
                    _ => return,
                }
                event.prevent_default();
            },

            ..props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`NavbarTrigger`] component.
#[derive(Props, Clone, PartialEq)]
pub struct NavbarTriggerProps {
    /// Additional attributes to apply to the trigger element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The children of the trigger component.
    pub children: Element,
}

/// # NavbarTrigger
///
/// The `NavbarTrigger` component is a button that opens and closes a [`NavbarNav`]. It controls if the associated [`NavbarContent`] is visible or not.
///
/// This must be used inside a [`NavbarNav`] component.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::navbar::{Navbar, NavbarContent, NavbarItem, NavbarNav, NavbarTrigger};
///
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         Navbar {
///             aria_label: "Components",
///             NavbarNav { index: 0usize,
///                 NavbarTrigger {
///                     "Inputs"
///                 }
///                 NavbarContent {
///                     NavbarItem {
///                         index: 0usize,
///                         value: "calendar".to_string(),
///                         to: "https://dioxuslabs.github.io/components/component/?name=calendar",
///                         "Calendar"
///                     }
///                     NavbarItem {
///                         index: 1usize,
///                         value: "slider".to_string(),
///                         to: "https://dioxuslabs.github.io/components/component/?name=slider",
///                         "Slider"
///                     }
///                 }
///             }
///             NavbarNav { index: 1usize,
///                 NavbarTrigger {
///                     "Information"
///                 }
///                 NavbarContent {
///                     NavbarItem {
///                         index: 0usize,
///                         value: "toast".to_string(),
///                         to: "https://dioxuslabs.github.io/components/component/?name=toast",
///                         "Toast"
///                     }
///                     NavbarItem {
///                         index: 1usize,
///                         value: "tabs".to_string(),
///                         to: "https://dioxuslabs.github.io/components/component/?name=tabs",
///                         "Tabs"
///                     }
///                 }
///             }
///             NavbarItem {
///                 index: 2usize,
///                 value: "home".to_string(),
///                 to: "https://dioxuslabs.github.io/components",
///                 "Home"
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn NavbarTrigger(props: NavbarTriggerProps) -> Element {
    let mut ctx: NavbarContext = use_context();
    let nav_ctx: NavbarNavContext = use_context();
    let onmounted = use_focus_control(ctx.focus, nav_ctx.index);
    let is_focused = move || {
        ctx.focus.current_focus() == Some(nav_ctx.index.cloned()) && !nav_ctx.focus.any_focused()
    };
    let disabled = move || (ctx.disabled)() || (nav_ctx.disabled)();
    let is_open = nav_ctx.is_open;

    rsx! {
        button {
            onmounted,
            onpointerdown: move |event| {
                if !disabled() {
                    // Suppress the synthesized focus shift so that tapping a child
                    // menuitem on mobile doesn't blur the trigger and trip onblur,
                    // which would close the nav mid-tap and detach the menuitem
                    // before Playwright (or a real user) can complete the tap.
                    event.prevent_default();
                    let new_open = if is_open() { None } else { Some(nav_ctx.index.cloned()) };
                    ctx.set_open_nav.call(new_open);
                }
            },
            onblur: move |_| {
                if is_focused() {
                    ctx.focus.set_focus(None);
                    ctx.set_open_nav.call(None);
                }
            },
            role: "menuitem",
            type: "button",
            tabindex: if is_focused() { "0" } else { "-1" },
            ..props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`NavbarContent`] component.
#[derive(Props, Clone, PartialEq)]
pub struct NavbarContentProps {
    /// The id of the content element.
    pub id: ReadSignal<Option<String>>,
    /// Additional attributes to apply to the content element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The children of the content component.
    pub children: Element,
}

/// # NavbarContent
///
/// The `NavbarContent` component defines the content of a [`NavbarNav`] that appears when the
/// [`NavbarTrigger`] is activated. It contains the [`NavbarItem`]s inside the navbar dropdown.
///
/// This must be used inside a [`NavbarNav`] component.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::navbar::{Navbar, NavbarContent, NavbarItem, NavbarNav, NavbarTrigger};
///
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         Navbar {
///             aria_label: "Components",
///             NavbarNav { index: 0usize,
///                 NavbarTrigger {
///                     "Inputs"
///                 }
///                 NavbarContent {
///                     NavbarItem {
///                         index: 0usize,
///                         value: "calendar".to_string(),
///                         to: "https://dioxuslabs.github.io/components/component/?name=calendar",
///                         "Calendar"
///                     }
///                     NavbarItem {
///                         index: 1usize,
///                         value: "slider".to_string(),
///                         to: "https://dioxuslabs.github.io/components/component/?name=slider",
///                         "Slider"
///                     }
///                 }
///             }
///             NavbarNav { index: 1usize,
///                 NavbarTrigger {
///                     "Information"
///                 }
///                 NavbarContent {
///                     NavbarItem {
///                         index: 0usize,
///                         value: "toast".to_string(),
///                         to: "https://dioxuslabs.github.io/components/component/?name=toast",
///                         "Toast"
///                     }
///                     NavbarItem {
///                         index: 1usize,
///                         value: "tabs".to_string(),
///                         to: "https://dioxuslabs.github.io/components/component/?name=tabs",
///                         "Tabs"
///                     }
///                 }
///             }
///             NavbarItem {
///                 index: 2usize,
///                 value: "home".to_string(),
///                 to: "https://dioxuslabs.github.io/components",
///                 "Home"
///             }
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`NavbarContent`] component defines the following data attributes you can use to control styling:
/// - `data-state`: Indicates if the nav is open or closed. Values are `open` or `closed`.
/// - `data-open-menu-direction`: Indicates the direction of the open menu relative to this content. Values are `start`, `end`, `open`, or `closed`.
#[component]
pub fn NavbarContent(props: NavbarContentProps) -> Element {
    let ctx: NavbarContext = use_context();
    let nav_ctx: NavbarNavContext = use_context();
    let index = nav_ctx.index.cloned();
    let open_direction = match (ctx.open_nav)() {
        Some(open_index) if open_index > index => "start",
        Some(open_index) if open_index < index => "end",
        Some(_) => "open",
        None => "closed",
    };

    let unique_id = use_unique_id();
    let id = use_id_or(unique_id, props.id);

    let render = use_animated_open(id, nav_ctx.is_open);
    use_deferred_focus(nav_ctx.focus, nav_ctx.initial_focus, render);

    rsx! {
        if render() {
            div {
                id,
                role: "menu",
                "data-state": if (nav_ctx.is_open)() { "open" } else { "closed" },
                "data-open-menu-direction": "{open_direction}",
                ..props.attributes,
                {props.children}
            }
        }
    }
}

/// The props for the [`NavbarItem`] component.
#[derive(Props, Clone, PartialEq)]
pub struct NavbarItemProps {
    /// The index of this item within the nav. This is used to define the focus order for keyboard navigation.
    pub index: ReadSignal<usize>,

    /// The value associated with this nav item. This will be passed to the [`Self::on_select`] callback when the item is selected.
    pub value: String,

    /// Whether this nav item is disabled.
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// Callback fired when the item is selected. The [`Self::value`] will be passed to this callback when the item is selected.
    #[props(default)]
    pub on_select: Callback<String>,

    /// The class attribute for the `a` tag.
    pub class: Option<String>,

    /// A class to apply to the generate HTML anchor tag if the `target` route is active.
    pub active_class: Option<String>,

    /// When [`true`], the `target` route will be opened in a new tab.
    ///
    /// This does not change whether the [`Link`] is active or not.
    #[props(default)]
    pub new_tab: bool,

    /// The onclick event handler.
    pub onclick: Option<EventHandler<MouseEvent>>,

    /// The onmounted event handler.
    /// Fired when the `<a>` element is mounted.
    pub onmounted: Option<EventHandler<MountedEvent>>,

    #[props(default)]
    /// Whether the default behavior should be executed if an `onclick` handler is provided.
    ///
    /// 1. When `onclick` is [`None`] (default if not specified), `onclick_only` has no effect.
    /// 2. If `onclick_only` is [`false`] (default if not specified), the provided `onclick` handler
    ///    will be executed after the links regular functionality.
    /// 3. If `onclick_only` is [`true`], only the provided `onclick` handler will be executed.
    pub onclick_only: bool,

    /// The rel attribute for the generated HTML anchor tag.
    ///
    /// For external `a`s, this defaults to `noopener noreferrer`.
    pub rel: Option<String>,

    /// The navigation target. Roughly equivalent to the href attribute of an HTML anchor tag.
    #[props(into)]
    pub to: NavigationTarget,

    /// Additional attributes to apply to the item element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children to render within the generated HTML anchor tag.
    pub children: Element,
}

/// # NavbarItem
///
/// The `NavbarItem` component represents a link within a navbar. It accepts a superset of the props
/// from the [`Link`] component.
///
/// This must be used inside a [`NavbarContent`] component.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::navbar::{Navbar, NavbarContent, NavbarItem, NavbarNav, NavbarTrigger};
///
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         Navbar {
///             aria_label: "Components",
///             NavbarNav { index: 0usize,
///                 NavbarTrigger {
///                     "Inputs"
///                 }
///                 NavbarContent {
///                     NavbarItem {
///                         index: 0usize,
///                         value: "calendar".to_string(),
///                         to: "https://dioxuslabs.github.io/components/component/?name=calendar",
///                         "Calendar"
///                     }
///                     NavbarItem {
///                         index: 1usize,
///                         value: "slider".to_string(),
///                         to: "https://dioxuslabs.github.io/components/component/?name=slider",
///                         "Slider"
///                     }
///                 }
///             }
///             NavbarNav { index: 1usize,
///                 NavbarTrigger {
///                     "Information"
///                 }
///                 NavbarContent {
///                     NavbarItem {
///                         index: 0usize,
///                         value: "toast".to_string(),
///                         to: "https://dioxuslabs.github.io/components/component/?name=toast",
///                         "Toast"
///                     }
///                     NavbarItem {
///                         index: 1usize,
///                         value: "tabs".to_string(),
///                         to: "https://dioxuslabs.github.io/components/component/?name=tabs",
///                         "Tabs"
///                     }
///                 }
///             }
///             NavbarItem {
///                 index: 2usize,
///                 value: "home".to_string(),
///                 to: "https://dioxuslabs.github.io/components",
///                 "Home"
///             }
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`NavbarItem`] component defines the following data attributes you can use to control styling:
/// - `data-disabled`: Indicates if the item is disabled. Values are `true` or `false`.
#[component]
pub fn NavbarItem(mut props: NavbarItemProps) -> Element {
    let mut ctx: NavbarContext = use_context();
    let mut nav_ctx: Option<NavbarNavContext> = try_use_context();

    let disabled = move || {
        (ctx.disabled)()
            || nav_ctx
                .map(|nav_ctx| (nav_ctx.disabled)())
                .unwrap_or_default()
            || (props.disabled)()
    };
    let focused = move || {
        nav_ctx.map_or_else(
            || ctx.focus.is_focused(props.index.cloned()),
            |nav_ctx| nav_ctx.focus.is_focused(props.index.cloned()) && (nav_ctx.is_open)(),
        )
    };

    let mut onmounted = use_focus_controlled_item_disabled(props.index, disabled);

    props.attributes.push(onkeydown({
        let value = props.value.clone();
        let to = props.to.clone();
        move |event: Event<KeyboardData>| {
            if event.key() == Key::Enter || event.key() == Key::Character(" ".to_string()) {
                if !disabled() {
                    props.on_select.call(value.clone());
                    ctx.set_open_nav.call(None);
                    let navigator = navigator();
                    navigator.push(to.clone());
                }
                event.prevent_default();
                event.stop_propagation();
            }
        }
    }));

    props.attributes.push(onpointerdown(move |_| {
        if !disabled() {
            if let Some(mut nav_ctx) = nav_ctx {
                nav_ctx.focus.set_focus(Some(props.index.cloned()));
            }
        }
    }));

    props.attributes.push(onblur(move |_| {
        if focused() {
            if let Some(nav_ctx) = &mut nav_ctx {
                nav_ctx.focus.blur();
            }
            ctx.focus.set_focus(None);
            ctx.set_open_nav.call(None);
        }
    }));

    let tabindex = if focused()
        || (nav_ctx.is_none() && ctx.focus.recent_focus_or_default() == props.index.cloned())
    {
        "0"
    } else {
        "-1"
    };

    rsx! {
        Link {
            class: props.class,
            active_class: props.active_class,
            new_tab: props.new_tab,
            onclick_only: props.onclick_only,
            rel: props.rel,
            to: props.to,
            role: "menuitem",
            "data-disabled": disabled(),
            tabindex,

            onclick: {
                let value = props.value.clone();
                move |mouse_event| {
                    if !disabled() {
                        props.on_select.call(value.clone());
                        ctx.set_open_nav.call(None);
                    }
                    if let Some(onclick) = props.onclick {
                        onclick.call(mouse_event);
                    }
                }
            },

            onmounted: move |evt: MountedEvent| {
                onmounted(evt.clone());
                if let Some(onmounted) = &props.onmounted {
                    onmounted.call(evt);
                }
            },

            attributes: props.attributes,
            {props.children}
        }
    }
}
