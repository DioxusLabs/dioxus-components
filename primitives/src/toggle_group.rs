//! Defines the [`ToggleGroup`] component and its sub-components, which manage a group of toggle buttons with single or multiple selection.

use crate::{
    focus::{use_focus_controlled_item_disabled, use_focus_provider, FocusState},
    toggle::Toggle,
    use_controlled,
};
use dioxus::prelude::*;
use std::collections::HashSet;

// Todo: docs, test controlled version

#[derive(Clone, Copy)]
struct ToggleGroupCtx {
    // State
    disabled: ReadSignal<bool>,
    pressed: Memo<HashSet<usize>>,
    set_pressed: Callback<HashSet<usize>>,

    allow_multiple_pressed: ReadSignal<bool>,

    // Focus state
    focus: FocusState,

    horizontal: ReadSignal<bool>,
    roving_loop: ReadSignal<bool>,
}

impl ToggleGroupCtx {
    fn orientation(&self) -> &'static str {
        match (self.horizontal)() {
            true => "horizontal",
            false => "vertical",
        }
    }

    fn is_pressed(&self, id: usize) -> bool {
        let pressed = (self.pressed)();
        pressed.contains(&id)
    }

    fn set_pressed(&self, id: usize, pressed: bool) {
        let mut new_pressed = (self.pressed)();
        match pressed {
            false => new_pressed.remove(&id),
            true => {
                if !(self.allow_multiple_pressed)() {
                    new_pressed.clear();
                }
                new_pressed.insert(id)
            }
        };
        self.set_pressed.call(new_pressed);
    }

    fn is_horizontal(&self) -> bool {
        (self.horizontal)()
    }

    fn focus_next(&mut self) {
        if !(self.roving_loop)() {
            return;
        }

        self.focus.focus_next();
    }

    fn focus_prev(&mut self) {
        if !(self.roving_loop)() {
            return;
        }

        self.focus.focus_prev();
    }

    fn is_roving_loop(&self) -> bool {
        (self.roving_loop)()
    }
}

/// The props for the [`ToggleGroup`] component
#[derive(Props, Clone, PartialEq)]
pub struct ToggleGroupProps {
    /// The default pressed items if the component is not controlled.
    #[props(default)]
    pub default_pressed: HashSet<usize>,

    /// The currently pressed items. This can be used to drive the component when controlled.
    pub pressed: ReadSignal<Option<HashSet<usize>>>,

    /// Callback to handle changes in pressed state
    #[props(default)]
    pub on_pressed_change: Callback<HashSet<usize>>,

    /// Whether the toggle group is disabled
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// If multiple items can be pressed at the same time. If this is false, only one item can be pressed at a time (radio-style).
    #[props(default)]
    pub allow_multiple_pressed: ReadSignal<bool>,

    /// Whether the toggle group is horizontal or vertical.
    #[props(default)]
    pub horizontal: ReadSignal<bool>,

    /// Whether focus should loop around when reaching the end.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub roving_loop: ReadSignal<bool>,

    /// Additional attributes to apply to the toggle group element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the toggle group, which should include multiple [`ToggleItem`] components.
    pub children: Element,
}

/// # ToggleGroup
///
/// The `ToggleGroup` component manages a group of toggle buttons. It supports both single (radio-style) and multiple selection modes with keyboard navigation.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::toggle_group::{ToggleGroup, ToggleItem};
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         ToggleGroup { horizontal: true, allow_multiple_pressed: true,
///             ToggleItem { index: 0usize, em { "B" } }
///             ToggleItem { index: 1usize, i { "I" } }
///             ToggleItem { index: 2usize, u { "U" } }
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`ToggleGroup`] component defines the following data attributes you can use to control styling:
/// - `data-orientation`: Indicates the orientation of the toggle group. Values are `horizontal` or `vertical`.
/// - `data-allow-multiple-pressed`: Indicates if multiple items can be pressed at the same time. Values are `true` or `false`.
#[component]
pub fn ToggleGroup(props: ToggleGroupProps) -> Element {
    let (pressed, set_pressed) = use_controlled(
        props.pressed,
        props.default_pressed,
        props.on_pressed_change,
    );

    let focus = use_focus_provider(props.roving_loop);
    let mut ctx = use_context_provider(|| ToggleGroupCtx {
        pressed,
        set_pressed,
        allow_multiple_pressed: props.allow_multiple_pressed,
        disabled: props.disabled,

        focus,
        horizontal: props.horizontal,
        roving_loop: props.roving_loop,
    });

    rsx! {
        div {
            onfocusout: move |_| ctx.focus.set_focus(None),

            "data-orientation": ctx.orientation(),
            "data-allow-multiple-pressed": ctx.allow_multiple_pressed,
            ..props.attributes,

            {props.children}
        }
    }
}

/// The props for the [`ToggleItem`] component
#[derive(Props, Clone, PartialEq)]
pub struct ToggleItemProps {
    /// The index of the item within the [`ToggleGroup`]. This is used to order the items for keyboard navigation.
    pub index: ReadSignal<usize>,

    /// Whether the toggle item is disabled.
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// Additional attributes to apply to the toggle item element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the toggle item
    pub children: Element,
}

/// # ToggleItem
///
/// An individual toggle button within a [`ToggleGroup`] component.
///
/// This must be used inside a [`ToggleGroup`] component.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::toggle_group::{ToggleGroup, ToggleItem};
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         ToggleGroup { horizontal: true, allow_multiple_pressed: true,
///             ToggleItem { index: 0usize, em { "B" } }
///             ToggleItem { index: 1usize, i { "I" } }
///             ToggleItem { index: 2usize, u { "U" } }
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`ToggleItem`] component defines the following data attributes you can use to control styling:
/// - `data-state`: Indicates the state of the toggle. Values are `on` or `off`.
/// - `data-disabled`: Indicates if the toggle is disabled. Values are `true` or `false`.
/// - `data-orientation`: Indicates the orientation of the toggle group. Values are `horizontal` or `vertical`.
#[component]
pub fn ToggleItem(props: ToggleItemProps) -> Element {
    let mut ctx: ToggleGroupCtx = use_context();

    // We need a kept-alive signal to control the toggle.
    let mut pressed = use_signal(|| ctx.is_pressed(props.index.cloned()));
    use_effect(move || {
        let is_pressed = ctx.is_pressed(props.index.cloned());
        pressed.set(is_pressed);
    });

    // Tab index for roving index
    let tab_index = use_memo(move || {
        if !ctx.is_roving_loop() {
            return "0";
        }

        match ctx.focus.recent_focus_or_default() == props.index.cloned() {
            true => "0",
            false => "-1",
        }
    });

    let disabled = move || (ctx.disabled)() || (props.disabled)();
    let onmounted = use_focus_controlled_item_disabled(props.index, disabled);

    rsx! {
        Toggle {
            onmounted,
            onfocus: move |_| ctx.focus.set_focus(Some(props.index.cloned())),
            onkeydown: move |event: Event<KeyboardData>| {
                let key = event.key();
                let horizontal = ctx.is_horizontal();
                let mut prevent_default = true;

                match key {
                    Key::ArrowUp if !horizontal => ctx.focus_prev(),
                    Key::ArrowDown if !horizontal => ctx.focus_next(),
                    Key::ArrowLeft if horizontal => ctx.focus_prev(),
                    Key::ArrowRight if horizontal => ctx.focus_next(),
                    Key::Home => ctx.focus.focus_first(),
                    Key::End => ctx.focus.focus_last(),
                    _ => prevent_default = false,
                };

                if prevent_default {
                    event.prevent_default();
                }
            },

            tabindex: tab_index,
            disabled: disabled(),
            "data-orientation": ctx.orientation(),

            pressed: pressed(),
            on_pressed_change: move |pressed| {
                ctx.set_pressed(props.index.cloned(), pressed);
            },

            attributes: props.attributes.clone(),

            {props.children}
        }
    }
}
