//! Combobox target anatomy components.

use dioxus::prelude::*;

use super::super::{context::ComboboxContext, hook::ComboboxDropdownEventSource};
use crate::{dioxus_attributes::attributes, merge_attributes, use_unique_id};

fn active_descendant(ctx: ComboboxContext, open: Memo<bool>) -> Memo<Option<String>> {
    use_memo(move || {
        if !open() {
            return None;
        }
        ctx.focused_option_id()
    })
}

fn handle_events_target_keydown(event: KeyboardEvent, mut ctx: ComboboxContext, open: Memo<bool>) {
    if (ctx.selectable.disabled)() {
        event.prevent_default();
        event.stop_propagation();
        return;
    }

    match event.key() {
        Key::ArrowDown => {
            if !open() {
                ctx.open_with_empty_query_and_focus_first();
            } else {
                ctx.focus_next_visible();
            }
            event.prevent_default();
            event.stop_propagation();
        }
        Key::ArrowUp => {
            if !open() {
                ctx.open_with_empty_query_and_focus_last();
            } else {
                ctx.focus_prev_visible();
            }
            event.prevent_default();
            event.stop_propagation();
        }
        Key::Home if open() => {
            ctx.focus_first_visible();
            event.prevent_default();
            event.stop_propagation();
        }
        Key::End if open() => {
            ctx.focus_last_visible();
            event.prevent_default();
            event.stop_propagation();
        }
        Key::Enter if open() => {
            ctx.select_focused();
            event.prevent_default();
            event.stop_propagation();
        }
        Key::Escape if open() => {
            ctx.set_open(false);
            event.prevent_default();
            event.stop_propagation();
        }
        _ => {}
    }
}

/// Declarative props for the combobox focus target element.
#[derive(Clone, Copy)]
pub struct ComboboxTargetHandle {
    ctx: ComboboxContext,
}

impl ComboboxTargetHandle {
    /// Returns attributes to spread onto the interactive target element.
    pub fn spread(&self) -> Vec<Attribute> {
        let ctx = self.ctx;

        attributes!(div {
            "data-combobox-target": true,
            onmounted: move |event| {
                ctx.store.register_target_mount_ref(event.data());
            },
        })
    }

    /// Focuses the mounted target element.
    pub fn focus(&self) {
        self.ctx.store.focus_target();
    }
}

/// Returns a handle for the combobox focus target element.
pub fn use_combobox_target() -> ComboboxTargetHandle {
    ComboboxTargetHandle {
        ctx: use_context::<ComboboxContext>(),
    }
}

/// Returns attributes that register the current element as the combobox focus target.
///
/// Prefer [`use_combobox_target`] for new code.
pub fn use_combobox_target_attributes() -> Vec<Attribute> {
    use_combobox_target().spread()
}

#[derive(Clone, Copy)]
struct ComboboxTargetWrapperHandle {}

impl ComboboxTargetWrapperHandle {
    fn spread(&self) -> Vec<Attribute> {
        attributes!(div {
            "data-combobox-target": true,
        })
    }
}

fn use_combobox_target_wrapper() -> ComboboxTargetWrapperHandle {
    ComboboxTargetWrapperHandle {}
}

/// Props for [`ComboboxTarget`].
#[derive(Props, Clone, PartialEq)]
pub struct ComboboxTargetProps {
    /// Optional custom element renderer for the target attributes.
    #[props(default)]
    pub r#as: Option<Callback<Vec<Attribute>, Element>>,

    /// Additional attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children rendered inside the target wrapper.
    pub children: Element,
}

/// Renders a structural wrapper around the combobox target area.
///
/// Use [`use_combobox_target`] on a custom interactive element when
/// that element should receive [`ComboboxStore::focus_target`](crate::combobox::ComboboxStore::focus_target).
#[component]
pub fn ComboboxTarget(props: ComboboxTargetProps) -> Element {
    let target = use_combobox_target();
    let wrapper = use_combobox_target_wrapper();

    if let Some(dynamic) = props.r#as {
        let merged = merge_attributes(vec![target.spread(), props.attributes]);
        return dynamic.call(merged);
    }

    let merged = merge_attributes(vec![wrapper.spread(), props.attributes]);

    rsx! {
        div {
            ..merged,
            {props.children}
        }
    }
}

/// Declarative props for the combobox element that owns trigger events.
#[derive(Clone, Copy)]
pub struct ComboboxEventsTargetHandle {
    ctx: ComboboxContext,
    open: Memo<bool>,
    active_descendant: Memo<Option<String>>,
    disabled: ReadSignal<bool>,
}

impl ComboboxEventsTargetHandle {
    /// Returns attributes to spread onto the combobox events target.
    pub fn spread(&self) -> Vec<Attribute> {
        let ctx = self.ctx;
        let open = self.open;
        let active_descendant = self.active_descendant;
        let disabled = self.disabled;

        attributes!(div {
            role: "combobox",
            tabindex: if disabled() { "-1" } else { "0" },
            aria_haspopup: "listbox",
            aria_expanded: open(),
            aria_controls: ctx.selectable.list_id(),
            aria_activedescendant: active_descendant(),
            aria_disabled: disabled(),
            "data-combobox-events-target": true,
            "data-state": if open() { "open" } else { "closed" },
            "data-disabled": disabled(),
            onclick: move |event| {
                if disabled() {
                    event.prevent_default();
                    event.stop_propagation();
                    return;
                }
                if !open() {
                    ctx.set_query.call(String::new());
                }
                ctx.store
                    .toggle_dropdown(ComboboxDropdownEventSource::Mouse);
            },
            onkeydown: move |event| {
                handle_events_target_keydown(event, ctx, open);
            },
        })
    }

    /// Returns whether the dropdown is currently open.
    pub fn opened(&self) -> bool {
        (self.open)()
    }
}

/// Returns a handle for the combobox element that owns trigger events.
pub fn use_combobox_events_target() -> ComboboxEventsTargetHandle {
    let ctx = use_context::<ComboboxContext>();
    let open = use_memo(move || ctx.store.dropdown_opened());
    let active_descendant = active_descendant(ctx, open);

    ComboboxEventsTargetHandle {
        ctx,
        open,
        active_descendant,
        disabled: ctx.selectable.disabled,
    }
}

/// Returns attributes for the combobox events target.
///
/// Prefer [`use_combobox_events_target`] for new code.
pub fn use_combobox_events_target_attributes() -> Vec<Attribute> {
    use_combobox_events_target().spread()
}

/// Props for [`ComboboxEventsTarget`].
#[derive(Props, Clone, PartialEq)]
pub struct ComboboxEventsTargetProps {
    /// Optional custom element renderer for the events target attributes.
    #[props(default)]
    pub r#as: Option<Callback<Vec<Attribute>, Element>>,

    /// Additional attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children rendered inside the events target.
    pub children: Element,
}

/// Element that owns combobox trigger ARIA and keyboard/pointer interactions.
#[component]
pub fn ComboboxEventsTarget(props: ComboboxEventsTargetProps) -> Element {
    let target = use_combobox_events_target();
    let merged = merge_attributes(vec![target.spread(), props.attributes]);

    if let Some(dynamic) = props.r#as {
        return dynamic.call(merged);
    }

    rsx! {
        div {
            ..merged,
            {props.children}
        }
    }
}

/// Declarative props for the dropdown anchoring target.
#[derive(Clone, Copy)]
pub struct ComboboxDropdownTargetHandle {
    open: Memo<bool>,
}

impl ComboboxDropdownTargetHandle {
    /// Returns attributes to spread onto the dropdown anchoring target.
    pub fn spread(&self) -> Vec<Attribute> {
        let open = self.open;

        attributes!(div {
            "data-combobox-dropdown-target": true,
            "data-state": if open() { "open" } else { "closed" },
        })
    }

    /// Returns whether the dropdown is currently open.
    pub fn opened(&self) -> bool {
        (self.open)()
    }
}

/// Returns a handle for the dropdown anchoring target.
pub fn use_combobox_dropdown_target() -> ComboboxDropdownTargetHandle {
    let ctx = use_context::<ComboboxContext>();
    let open = use_memo(move || ctx.store.dropdown_opened());

    ComboboxDropdownTargetHandle { open }
}

/// Returns attributes for an element that marks the dropdown anchoring target.
///
/// Prefer [`use_combobox_dropdown_target`] for new code.
pub fn use_combobox_dropdown_target_attributes() -> Vec<Attribute> {
    use_combobox_dropdown_target().spread()
}

/// Props for [`ComboboxDropdownTarget`].
#[derive(Props, Clone, PartialEq)]
pub struct ComboboxDropdownTargetProps {
    /// Optional custom element renderer for the dropdown target attributes.
    #[props(default)]
    pub r#as: Option<Callback<Vec<Attribute>, Element>>,

    /// Additional attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Children rendered inside the dropdown target.
    pub children: Element,
}

/// Wraps dropdown content when the dropdown target differs from the events target.
#[component]
pub fn ComboboxDropdownTarget(props: ComboboxDropdownTargetProps) -> Element {
    let target = use_combobox_dropdown_target();
    let merged = merge_attributes(vec![target.spread(), props.attributes]);

    if let Some(dynamic) = props.r#as {
        return dynamic.call(merged);
    }

    rsx! {
        div {
            ..merged,
            {props.children}
        }
    }
}

/// Options for [`use_combobox_search`].
#[derive(Clone, Copy)]
pub struct UseComboboxSearchOptions {
    /// Placeholder shown when the input is empty.
    pub placeholder: ReadSignal<String>,
    /// Optional id for the input element.
    pub id: ReadSignal<Option<String>>,
    /// Whether this input should also be the focus target.
    pub register_target: bool,
    /// Whether to show selected option text while the dropdown is closed.
    pub show_selected_text: bool,
}

impl Default for UseComboboxSearchOptions {
    fn default() -> Self {
        Self {
            placeholder: ReadSignal::new(Signal::new(String::new())),
            id: ReadSignal::new(Signal::new(None)),
            register_target: false,
            show_selected_text: true,
        }
    }
}

/// Declarative props and controls for a native combobox search input.
#[derive(Clone, Copy)]
pub struct ComboboxSearchHandle {
    ctx: ComboboxContext,
    placeholder: ReadSignal<String>,
    id: ReadSignal<String>,
    register_target: bool,
    open: Memo<bool>,
    display_value: Memo<String>,
    active_descendant: Memo<Option<String>>,
    disabled: ReadSignal<bool>,
}

impl ComboboxSearchHandle {
    /// Returns attributes to spread onto the search input.
    pub fn spread(&self) -> Vec<Attribute> {
        let mut ctx = self.ctx;
        let placeholder = self.placeholder;
        let id = self.id;
        let register_target = self.register_target;
        let open = self.open;
        let set_query = ctx.set_query;
        let display_value = self.display_value;
        let active_descendant = self.active_descendant;
        let disabled = self.disabled;

        attributes!(input {
            id,
            r#type: "text",
            value: display_value(),
            placeholder,
            autocomplete: "off",
            spellcheck: "false",
            disabled: disabled(),

            role: "combobox",
            aria_autocomplete: "list",
            aria_haspopup: "listbox",
            aria_expanded: open(),
            aria_controls: ctx.selectable.list_id(),
            aria_activedescendant: active_descendant(),

            "data-combobox-search": true,
            "data-state": if open() { "open" } else { "closed" },

            onclick: move |event| {
                if disabled() {
                    event.prevent_default();
                    event.stop_propagation();
                    return;
                }
                if !open() {
                    set_query.call(String::new());
                    ctx.set_open(true);
                }
            },
            oninput: move |event| {
                if disabled() {
                    event.prevent_default();
                    event.stop_propagation();
                    return;
                }
                let was_open = open();
                let value = event.value();
                let next_query = if was_open {
                    value
                } else {
                    ctx.selectable
                        .selected_text()
                        .and_then(|selected| {
                            value
                                .strip_prefix(&selected)
                                .map(ToString::to_string)
                        })
                        .unwrap_or(value)
                };
                set_query.call(next_query);
                if was_open {
                    ctx.selectable.focus_state.set_focus(None);
                    ctx.store.reset_selected_option();
                } else {
                    ctx.set_open(true);
                }
            },
            onkeydown: move |event| {
                handle_events_target_keydown(event, ctx, open);
            },
            onmounted: move |event| {
                if register_target {
                    ctx.store.register_target_mount_ref(event.data());
                }
                ctx.store.register_search_mount_ref(event.data());
            },
            onblur: move |_| {
                if open() {
                    ctx.set_open(false);
                }
            },
        })
    }

    /// Returns the current search query.
    pub fn query(&self) -> String {
        self.ctx.query.cloned()
    }

    /// Updates the search query.
    pub fn search_for(&self, query: impl Into<String>) {
        self.ctx.set_query.call(query.into());
    }

    /// Focuses the mounted search input.
    pub fn focus(&self) {
        self.ctx.store.focus_search_input();
    }

    /// Returns whether the dropdown is currently open.
    pub fn opened(&self) -> bool {
        (self.open)()
    }
}

/// Returns a handle for a native combobox search input.
pub fn use_combobox_search(options: UseComboboxSearchOptions) -> ComboboxSearchHandle {
    let ctx = use_context::<ComboboxContext>();
    let fallback_id = use_unique_id();
    let id = crate::use_id_or(fallback_id, options.id);

    let open = use_memo(move || ctx.store.dropdown_opened());
    let active_descendant = active_descendant(ctx, open);
    let display_value = use_memo(move || {
        if open() {
            ctx.query.cloned()
        } else if options.show_selected_text {
            ctx.selectable.selected_text().unwrap_or_default()
        } else {
            String::new()
        }
    });

    ComboboxSearchHandle {
        ctx,
        placeholder: options.placeholder,
        id: id.into(),
        register_target: options.register_target,
        open,
        display_value,
        active_descendant,
        disabled: ctx.selectable.disabled,
    }
}

/// Returns attributes for a native combobox search input.
///
/// Prefer [`use_combobox_search`] for new code.
pub fn use_combobox_search_attributes(
    placeholder: ReadSignal<String>,
    id: ReadSignal<Option<String>>,
    register_target: bool,
    show_selected_text: bool,
) -> Vec<Attribute> {
    use_combobox_search(UseComboboxSearchOptions {
        placeholder,
        id,
        register_target,
        show_selected_text,
    })
    .spread()
}

pub(super) fn render_combobox_search(
    placeholder: ReadSignal<String>,
    id: ReadSignal<Option<String>>,
    attributes: Vec<Attribute>,
    register_target: bool,
    show_selected_text: bool,
) -> Element {
    let search = use_combobox_search(UseComboboxSearchOptions {
        placeholder,
        id,
        register_target,
        show_selected_text,
    });
    let merged = merge_attributes(vec![search.spread(), attributes]);

    rsx! {
        input {
            ..merged,
        }
    }
}
