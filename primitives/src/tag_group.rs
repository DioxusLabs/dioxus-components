//! Defines the [`TagGroup`] and [`TagGroupMulti`] components and their sub-components.

use dioxus::prelude::*;

use crate::{
    collection::{collection_item, use_collection_provider, use_item, CollectionState},
    selectable::SelectionMode,
    selection::{option_text_value, RcPartialEqValue},
    use_controlled, use_effect_cleanup, use_effect_with_cleanup, use_id_or, use_unique_id,
};

/// Selection and focus state for a tag group.
#[derive(Clone, Copy)]
struct TagGroupState {
    values: Memo<Vec<RcPartialEqValue>>,
    set_value: Callback<RcPartialEqValue>,
    set_values: Callback<Vec<RcPartialEqValue>>,
    clear_selection: Callback<()>,
    selection_mode: SelectionMode,
    items: Signal<Vec<TagItem>>,
    focus: CollectionState,
    disabled: ReadSignal<bool>,
    selectable: ReadSignal<bool>,
    allow_empty_selection: ReadSignal<bool>,
}

/// Context provided by [`TagGroup`] / [`TagGroupMulti`] to descendants.
#[derive(Clone, Copy)]
pub struct TagGroupCtx {
    labeled_by: Signal<Option<String>>,
    escape_clears_selection: ReadSignal<bool>,
    state: TagGroupState,
}

/// Provided by [`TagList`] for [`TagGroupEmpty`].
#[derive(Clone, Copy)]
struct TagListCtx {
    show_empty: Memo<bool>,
}

#[derive(Clone)]
struct TagOptionCtx {
    id: Signal<String>,
    /// Number of mounted [`TagRemoveButton`]s in this tag. The tag is removable
    /// when this is greater than zero, so removability is driven purely by the
    /// presence of a remove button rather than a separate prop.
    remove_button_count: Signal<usize>,
}

#[derive(Clone, PartialEq)]
struct TagItem {
    id: String,
    index: usize,
    value: RcPartialEqValue,
    text_value: String,
    disabled: bool,
    removable: bool,
    removed: bool,
}

struct TagGroupSharedProps {
    disabled: ReadSignal<bool>,
    selectable: ReadSignal<bool>,
    allow_empty_selection: ReadSignal<bool>,
    escape_clears_selection: ReadSignal<bool>,
    roving_loop: ReadSignal<bool>,
    attributes: Vec<Attribute>,
    children: Element,
}

struct TagGroupSelection {
    values: Memo<Vec<RcPartialEqValue>>,
    set_value: Callback<RcPartialEqValue>,
    set_values: Callback<Vec<RcPartialEqValue>>,
    clear_selection: Callback<()>,
    selection_mode: SelectionMode,
}

impl TagGroupSharedProps {
    fn from_single<T: Clone + PartialEq + 'static>(props: &TagGroupProps<T>) -> Self {
        Self {
            disabled: props.disabled,
            selectable: props.selectable,
            allow_empty_selection: props.allow_empty_selection,
            escape_clears_selection: props.escape_clears_selection,
            roving_loop: props.roving_loop,
            attributes: props.attributes.clone(),
            children: props.children.clone(),
        }
    }

    fn from_multi<T: Clone + PartialEq + 'static>(props: &TagGroupMultiProps<T>) -> Self {
        Self {
            disabled: props.disabled,
            selectable: props.selectable,
            allow_empty_selection: props.allow_empty_selection,
            escape_clears_selection: props.escape_clears_selection,
            roving_loop: props.roving_loop,
            attributes: props.attributes.clone(),
            children: props.children.clone(),
        }
    }
}

impl TagItem {
    fn is_focusable(&self) -> bool {
        !self.disabled && !self.removed
    }

    fn can_remove(&self) -> bool {
        self.is_focusable() && self.removable
    }
}

impl TagGroupCtx {
    fn is_empty(&self) -> bool {
        self.state.items.read().iter().all(|item| item.removed)
    }
}

impl TagGroupState {
    fn register_or_update_item(&mut self, item: TagItem) {
        sync_tag_item(&mut self.items.write(), item);
    }

    fn unregister_item(&mut self, id: &str) {
        self.items.write().retain(|item| item.id != id);
    }

    fn is_removed(&self, id: &str) -> bool {
        self.items
            .read()
            .iter()
            .find(|item| item.id == id)
            .map(|item| item.removed)
            .unwrap_or(false)
    }

    fn text_value(&self, id: &str) -> String {
        self.items
            .read()
            .iter()
            .find(|item| item.id == id)
            .map(|item| item.text_value.clone())
            .unwrap_or_default()
    }

    fn can_remove_item(&self, id: &str) -> bool {
        self.items
            .read()
            .iter()
            .find(|item| item.id == id)
            .is_some_and(TagItem::can_remove)
    }

    fn focus_item(&mut self, id: &str) {
        let index = self
            .items
            .read()
            .iter()
            .find(|item| item.id == id && item.is_focusable())
            .map(|item| item.index);
        self.focus.set_focus(index);
    }

    fn is_selected(&self, value: &RcPartialEqValue) -> bool {
        self.values.read().iter().any(|v| v == value)
    }

    fn toggle_value(&self, value: RcPartialEqValue) {
        if !(self.selectable)() {
            return;
        }

        let deselecting = self.is_selected(&value);
        if !deselecting {
            self.set_value.call(value);
            return;
        }

        let can_clear = match self.selection_mode {
            SelectionMode::Single => (self.allow_empty_selection)(),
            SelectionMode::Multiple => {
                (self.allow_empty_selection)() || self.values.read().len() > 1
            }
        };

        if can_clear {
            match self.selection_mode {
                SelectionMode::Single => self.clear_selection.call(()),
                SelectionMode::Multiple => self.set_value.call(value),
            }
        }
    }

    fn remove_item_from_button(&mut self, id: &str) -> bool {
        self.remove_items(vec![id.to_string()])
    }

    fn remove_focused_from_keyboard(&mut self, focused_id: &str) -> bool {
        let ids = self.keyboard_remove_item_ids(focused_id);
        self.remove_items(ids)
    }

    fn keyboard_remove_item_ids(&self, focused_id: &str) -> Vec<String> {
        let items = self.items.read();
        let Some(focused) = items.iter().find(|item| item.id == focused_id) else {
            return Vec::new();
        };
        if !focused.can_remove() {
            return Vec::new();
        }

        let selected_values = self.values.read().clone();
        let focused_selected = selected_values.iter().any(|value| value == &focused.value);
        if !focused_selected {
            return vec![focused.id.clone()];
        }

        items
            .iter()
            .filter(|item| {
                item.can_remove()
                    && selected_values
                        .iter()
                        .any(|selected| selected == &item.value)
            })
            .map(|item| item.id.clone())
            .collect()
    }

    fn remove_items(&mut self, ids: Vec<String>) -> bool {
        let items = self.items.read();
        let selected_values = self.values.read().clone();
        let mut removal_ids = Vec::new();
        let mut removed_selected_values: Vec<RcPartialEqValue> = Vec::new();

        for id in ids {
            if removal_ids.iter().any(|existing| existing == &id) {
                continue;
            }
            let Some(item) = items.iter().find(|item| item.id == id) else {
                continue;
            };
            if !item.can_remove() {
                continue;
            }
            if selected_values
                .iter()
                .any(|selected| selected == &item.value)
                && !removed_selected_values
                    .iter()
                    .any(|selected| selected == &item.value)
            {
                removed_selected_values.push(item.value.clone());
            }
            removal_ids.push(item.id.clone());
        }

        if removal_ids.is_empty() {
            return false;
        }

        let focus_target = self.focus.focused_index().and_then(|focused_index| {
            items
                .iter()
                .any(|item| {
                    item.index == focused_index
                        && removal_ids.iter().any(|removed_id| removed_id == &item.id)
                })
                .then(|| {
                    next_focus_after_removal(
                        &items,
                        focused_index,
                        &removal_ids,
                        self.focus.loops(),
                    )
                })
        });
        let remaining_selected_values =
            selected_values_after_removal(&selected_values, &removed_selected_values);

        drop(items);
        drop(selected_values);

        if let Some(target) = focus_target {
            self.focus.set_focus(target);
        }

        {
            let mut items = self.items.write();
            for item in items.iter_mut() {
                if removal_ids.iter().any(|id| id == &item.id) {
                    item.removed = true;
                }
            }
        }

        if !removed_selected_values.is_empty() {
            match self.selection_mode {
                SelectionMode::Single => self.clear_selection.call(()),
                SelectionMode::Multiple => {
                    self.set_values.call(remaining_selected_values);
                }
            }
        }

        true
    }
}

fn sync_tag_item(items: &mut Vec<TagItem>, mut item: TagItem) {
    if let Some(position) = items.iter().position(|existing| existing.id == item.id) {
        item.removed = items[position].removed && items[position].value == item.value;
        items.remove(position);
    }
    insert_tag_item(items, item);
}

fn insert_tag_item(items: &mut Vec<TagItem>, item: TagItem) {
    let insert_at = items.partition_point(|existing| existing.index <= item.index);
    items.insert(insert_at, item);
}

fn selected_values_after_removal(
    selected_values: &[RcPartialEqValue],
    removed_selected_values: &[RcPartialEqValue],
) -> Vec<RcPartialEqValue> {
    selected_values
        .iter()
        .filter(|selected| {
            !removed_selected_values
                .iter()
                .any(|removed| removed == *selected)
        })
        .cloned()
        .collect()
}

fn next_focus_after_removal(
    items: &[TagItem],
    focused_index: usize,
    removal_ids: &[String],
    roving_loop: bool,
) -> Option<usize> {
    let candidates: Vec<&TagItem> = items
        .iter()
        .filter(|item| {
            item.is_focusable() && !removal_ids.iter().any(|removed_id| removed_id == &item.id)
        })
        .collect();

    if candidates.is_empty() {
        return None;
    }

    let next_position = candidates.partition_point(|item| item.index <= focused_index);
    if let Some(next) = candidates.get(next_position) {
        return Some(next.index);
    }
    if roving_loop {
        return candidates.first().map(|item| item.index);
    }

    let prev_position = candidates.partition_point(|item| item.index < focused_index);
    prev_position
        .checked_sub(1)
        .and_then(|position| candidates.get(position).map(|item| item.index))
}

/// Props for [`TagGroup`] (single selection).
#[derive(Props, Clone, PartialEq)]
pub struct TagGroupProps<T: Clone + PartialEq + 'static = String> {
    /// Controlled selected value. `None` in the signal means no tag is selected.
    #[props(default)]
    pub value: Option<ReadSignal<Option<T>>>,

    /// Initial value when uncontrolled.
    #[props(default)]
    pub default_value: Option<T>,

    /// Called when the selected value changes.
    #[props(default)]
    pub on_value_change: Callback<Option<T>>,

    /// Whether the entire tag group is disabled.
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// Whether tags can be selected. When `false`, tags remain focusable but not selectable.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub selectable: ReadSignal<bool>,

    /// Whether clicking or pressing Space/Enter on the selected tag clears the selection.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub allow_empty_selection: ReadSignal<bool>,

    /// Whether pressing Escape clears the current selection.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub escape_clears_selection: ReadSignal<bool>,

    /// Whether keyboard focus loops from the last tag to the first and vice versa.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub roving_loop: ReadSignal<bool>,

    /// Additional attributes for the root element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the tag group, typically a [`TagList`] with [`TagOption`] children.
    pub children: Element,
}

/// Props for [`TagGroupMulti`] (multiple selection).
#[derive(Props, Clone, PartialEq)]
pub struct TagGroupMultiProps<T: Clone + PartialEq + 'static = String> {
    /// Controlled selected values.
    #[props(default)]
    pub values: ReadSignal<Option<Vec<T>>>,

    /// Initial values when uncontrolled.
    #[props(default)]
    pub default_values: Vec<T>,

    /// Called when the selected values change.
    #[props(default)]
    pub on_values_change: Callback<Vec<T>>,

    /// Whether the entire tag group is disabled.
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// Whether tags can be selected. When `false`, tags remain focusable but not selectable.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub selectable: ReadSignal<bool>,

    /// Whether clicking or pressing Space/Enter on a selected tag deselects it.
    /// When `false`, the last remaining selected tag cannot be deselected.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub allow_empty_selection: ReadSignal<bool>,

    /// Whether pressing Escape clears the current selection.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub escape_clears_selection: ReadSignal<bool>,

    /// Whether keyboard focus loops from the last tag to the first and vice versa.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub roving_loop: ReadSignal<bool>,

    /// Additional attributes for the root element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the tag group, typically a [`TagList`] with [`TagOption`] children.
    pub children: Element,
}

/// # TagGroup
///
/// A focusable group of tags with single selection.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::tag_group::{TagGroup, TagGroupLabel, TagList, TagOption};
///
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         TagGroup::<&'static str> {
///             default_value: Some("bug"),
///             TagGroupLabel { "Labels" }
///             TagList {
///                 TagOption::<&'static str> { index: 0usize, value: "bug", "bug" }
///                 TagOption::<&'static str> { index: 1usize, value: "feature", disabled: true, "feature" }
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn TagGroup<T: Clone + PartialEq + 'static>(props: TagGroupProps<T>) -> Element {
    let mut internal_value: Signal<Option<T>> = use_signal(|| props.default_value.clone());
    let value = use_memo(move || match props.value {
        Some(value) => value.cloned(),
        None => internal_value.cloned(),
    });
    let values = use_memo(move || value().map(RcPartialEqValue::new).into_iter().collect());
    let on_change = props.on_value_change;
    let set_value = use_callback(move |incoming: RcPartialEqValue| {
        let value = incoming
            .as_ref::<T>()
            .unwrap_or_else(|| panic!("TagGroup and TagOption value types must match"))
            .clone();
        internal_value.set(Some(value.clone()));
        on_change.call(Some(value));
    });
    let set_values = use_callback(move |incoming: Vec<RcPartialEqValue>| {
        if let Some(incoming) = incoming.into_iter().next() {
            let value = incoming
                .as_ref::<T>()
                .unwrap_or_else(|| panic!("TagGroup and TagOption value types must match"))
                .clone();
            internal_value.set(Some(value.clone()));
            on_change.call(Some(value));
        } else {
            internal_value.set(None);
            on_change.call(None);
        }
    });
    let clear_selection = use_callback(move |_| {
        internal_value.set(None);
        on_change.call(None);
    });

    use_tag_group_inner(
        TagGroupSharedProps::from_single(&props),
        TagGroupSelection {
            values,
            set_value,
            set_values,
            clear_selection,
            selection_mode: SelectionMode::Single,
        },
    )
}

/// # TagGroupMulti
///
/// A focusable group of tags with multiple selection.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::tag_group::{TagGroupLabel, TagGroupMulti, TagList, TagOption};
///
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         TagGroupMulti::<&'static str> {
///             default_values: vec!["bug"],
///             TagGroupLabel { "Labels" }
///             TagList {
///                 TagOption::<&'static str> { index: 0usize, value: "bug", "bug" }
///                 TagOption::<&'static str> { index: 1usize, value: "feature", "feature" }
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn TagGroupMulti<T: Clone + PartialEq + 'static>(props: TagGroupMultiProps<T>) -> Element {
    let (multi_values, set_multi_internal) = use_controlled(
        props.values,
        props.default_values.clone(),
        props.on_values_change,
    );

    let values = use_memo(move || {
        multi_values()
            .into_iter()
            .map(RcPartialEqValue::new)
            .collect()
    });
    let set_value = use_callback(move |value: RcPartialEqValue| {
        let value_t = value
            .as_ref::<T>()
            .unwrap_or_else(|| panic!("TagGroupMulti and TagOption value types must match"))
            .clone();
        let mut current = multi_values();
        if let Some(pos) = current.iter().position(|v| v == &value_t) {
            current.remove(pos);
        } else {
            current.push(value_t);
        }
        set_multi_internal.call(current);
    });
    let set_values = use_callback(move |values: Vec<RcPartialEqValue>| {
        let values = values
            .into_iter()
            .map(|value| {
                value
                    .as_ref::<T>()
                    .unwrap_or_else(|| panic!("TagGroupMulti and TagOption value types must match"))
                    .clone()
            })
            .collect();
        set_multi_internal.call(values);
    });
    let clear_selection = use_callback(move |_| {
        set_multi_internal.call(Vec::new());
    });

    use_tag_group_inner(
        TagGroupSharedProps::from_multi(&props),
        TagGroupSelection {
            values,
            set_value,
            set_values,
            clear_selection,
            selection_mode: SelectionMode::Multiple,
        },
    )
}

fn use_tag_group_inner(shared: TagGroupSharedProps, selection: TagGroupSelection) -> Element {
    let TagGroupSharedProps {
        disabled,
        selectable,
        allow_empty_selection,
        escape_clears_selection,
        roving_loop,
        attributes,
        children,
    } = shared;
    let TagGroupSelection {
        values,
        set_value,
        set_values,
        clear_selection,
        selection_mode,
    } = selection;

    let items: Signal<Vec<TagItem>> = use_signal(Vec::default);
    let focus = use_collection_provider(roving_loop);

    let state = TagGroupState {
        values,
        set_value,
        set_values,
        clear_selection,
        selection_mode,
        items,
        focus,
        disabled,
        selectable,
        allow_empty_selection,
    };

    let ctx = TagGroupCtx {
        labeled_by: use_signal(|| None),
        escape_clears_selection,
        state,
    };
    use_context_provider(|| ctx);

    rsx! {
        div {
            ..attributes,
            {children}
        }
    }
}

/// Props for [`TagGroupLabel`].
#[derive(Props, Clone, PartialEq)]
pub struct TagGroupLabelProps {
    /// Optional ID for the label element.
    #[props(default)]
    pub id: ReadSignal<Option<String>>,

    /// Additional attributes for the label.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Label content referenced by [`TagList`] via `aria-labelledby`.
    pub children: Element,
}

/// Visible label for a [`TagGroup`] or [`TagGroupMulti`], wired to the tag list through `aria-labelledby`.
///
/// Must be used inside [`TagGroup`] or [`TagGroupMulti`].
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::tag_group::{TagGroup, TagGroupLabel, TagList, TagOption};
///
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         TagGroup::<&'static str> {
///             TagGroupLabel { "Labels" }
///             TagList {
///                 TagOption::<&'static str> { index: 0usize, value: "bug", "bug" }
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn TagGroupLabel(props: TagGroupLabelProps) -> Element {
    let mut ctx: TagGroupCtx = use_context();

    let id = use_unique_id();
    let id = use_id_or(id, props.id);

    use_effect(move || {
        ctx.labeled_by.set(Some(id()));
    });

    rsx! {
        div {
            id: id(),
            ..props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`TagList`] component.
#[derive(Props, Clone, PartialEq)]
pub struct TagListProps {
    /// Additional attributes for the grid element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// [`TagOption`] children and an optional [`TagGroupEmpty`].
    pub children: Element,
}

/// Grid container for [`TagOption`] children.
#[component]
pub fn TagList(props: TagListProps) -> Element {
    let ctx = use_context::<TagGroupCtx>();
    let mut state = ctx.state;
    let mut mounted = use_signal(|| false);
    use_effect(move || mounted.set(true));
    let show_empty = use_memo(move || mounted() && ctx.is_empty());

    use_context_provider(|| TagListCtx { show_empty });

    let list_tabbable = use_memo(move || {
        !state.focus.any_focused() && state.focus.first_available_index().is_some()
    });

    rsx! {
        div {
            role: "grid",
            aria_labelledby: ctx.labeled_by,
            tabindex: if list_tabbable() { "0" } else { "-1" },
            aria_multiselectable: if state.selection_mode == SelectionMode::Multiple
                && (state.selectable)()
            {
                "true"
            },
            aria_colcount: "1",
            onfocus: move |_| {
                if !state.focus.any_focused() {
                    state.focus.focus_first();
                }
            },
            ..props.attributes,
            {props.children}
        }
    }
}

/// Props for [`TagGroupEmpty`].
#[derive(Props, Clone, PartialEq)]
pub struct TagGroupEmptyProps {
    /// Additional attributes for the empty state element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Content shown when every tag in the list has been removed.
    pub children: Element,
}

/// Renders when there are no tags left in the [`TagList`].
///
/// Must be used inside [`TagList`].
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::tag_group::{TagGroup, TagGroupEmpty, TagList, TagOption};
///
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         TagGroup::<&'static str> {
///             TagList {
///                 TagGroupEmpty { "No tags" }
///                 TagOption::<&'static str> { index: 0usize, value: "bug", "bug" }
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn TagGroupEmpty(props: TagGroupEmptyProps) -> Element {
    let list = use_context::<TagListCtx>();

    if !(list.show_empty)() {
        return rsx! {};
    }

    rsx! {
        div {
            role: "row",
            ..props.attributes,
            div {
                role: "gridcell",
                aria_colindex: "1",
                display: "contents",
                {props.children}
            }
        }
    }
}

/// Props for [`TagOption`].
#[derive(Props, Clone, PartialEq)]
pub struct TagOptionProps<T: Clone + PartialEq + 'static = String> {
    /// Programmatic value for this tag (selection and removal).
    pub value: ReadSignal<T>,

    /// Text used for the remove button label when no [`TagOptionProps::text_value`] is set.
    #[props(default)]
    pub text_value: ReadSignal<Option<String>>,

    /// Index for focus order and `aria-rowindex`.
    pub index: ReadSignal<usize>,

    /// Optional ID for the tag row element.
    #[props(default)]
    pub id: ReadSignal<Option<String>>,

    /// Whether this tag is disabled.
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// Additional attributes for the tag row element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The tag label; add a [`TagRemoveButton`] to make the tag removable
    /// (via click and via Delete/Backspace).
    pub children: Element,
}

fn tag_option_on_keydown(
    e: Event<KeyboardData>,
    ctx: TagGroupCtx,
    mut state: TagGroupState,
    id: String,
    value: RcPartialEqValue,
    is_disabled: bool,
    removable: bool,
) {
    if is_disabled {
        return;
    }

    let key = e.key();
    let mut prevent_default = false;

    match key {
        Key::Named(NamedKey::Escape) if (ctx.escape_clears_selection)() => {
            state.clear_selection.call(());
            prevent_default = true;
        }
        Key::Character(s) if s == " " => {
            state.toggle_value(value.clone());
            prevent_default = true;
        }
        Key::Named(NamedKey::Enter) => {
            state.toggle_value(value.clone());
            prevent_default = true;
        }
        Key::Named(NamedKey::Backspace) | Key::Named(NamedKey::Delete) if removable => {
            prevent_default = state.remove_focused_from_keyboard(&id);
        }
        Key::Named(NamedKey::ArrowUp) | Key::Named(NamedKey::ArrowLeft) => {
            state.focus.focus_prev();
            prevent_default = true;
        }
        Key::Named(NamedKey::ArrowDown) | Key::Named(NamedKey::ArrowRight) => {
            state.focus.focus_next();
            prevent_default = true;
        }
        Key::Named(NamedKey::Home) => {
            state.focus.focus_first();
            prevent_default = true;
        }
        Key::Named(NamedKey::End) => {
            state.focus.focus_last();
            prevent_default = true;
        }
        _ => {}
    }

    if prevent_default {
        e.prevent_default();
    }
}

/// A single tag inside [`TagList`]. Must be used within [`TagGroup`] or [`TagGroupMulti`].
#[component]
pub fn TagOption<T: Clone + PartialEq + 'static>(props: TagOptionProps<T>) -> Element {
    let ctx: TagGroupCtx = use_context();
    let mut state = ctx.state;
    let index = props.index;
    let option_disabled = props.disabled;
    // Removability is driven by the presence of `TagRemoveButton` children, which
    // increment this counter while mounted (see `TagRemoveButton`).
    let remove_button_count = use_signal(|| 0usize);
    let is_removable = use_memo(move || remove_button_count() > 0);
    let text_value_signal = props.text_value;
    let option_value = props.value;
    let value = use_memo(move || RcPartialEqValue::new(option_value.cloned()));

    let disabled = {
        let root_disabled = state.disabled;
        use_memo(move || root_disabled.cloned() || option_disabled.cloned())
    };

    let id = use_id_or(use_unique_id(), props.id);
    let item_id = use_unique_id();
    let text_value = use_memo(move || {
        option_text_value(&*option_value.read(), text_value_signal(), "TagOption")
    });
    let is_removed = use_memo(move || state.is_removed(&item_id()));

    use_effect(move || {
        let option_id = item_id();
        state.register_or_update_item(TagItem {
            id: option_id.clone(),
            index: index(),
            value: value(),
            text_value: text_value.cloned(),
            disabled: disabled(),
            removable: is_removable(),
            removed: false,
        });
    });
    let mut cleanup_state = state;
    use_effect_cleanup(move || {
        cleanup_state.unregister_item(&item_id());
    });

    let selected = use_memo(move || state.selectable.cloned() && state.is_selected(&value()));

    use_context_provider(|| TagOptionCtx {
        id: item_id,
        remove_button_count,
    });

    let item = use_item(
        collection_item(state.focus, index)
            .key(move || Some(item_id()))
            .disabled(move || disabled.cloned())
            .hidden(move || is_removed.cloned()),
    );
    let tabindex = item.tabindex;
    let onmounted = item.onmounted();

    if is_removed() {
        return rsx! {};
    }

    rsx! {
        div {
            role: "row",
            id: id(),
            tabindex,
            aria_rowindex: (index.cloned() as i32) + 1,
            aria_selected: (state.selectable)().then_some(selected()),
            aria_disabled: disabled(),
            "data-selected": selected(),
            "data-disabled": disabled(),
            onmounted,
            onfocus: move |_| state.focus_item(&item_id()),
            onclick: move |_| {
                if !disabled() {
                    state.toggle_value(value());
                }
            },
            onkeydown: move |e| {
                tag_option_on_keydown(
                    e,
                    ctx,
                    state,
                    item_id(),
                    value(),
                    disabled(),
                    is_removable(),
                );
            },
            ..props.attributes,
            div {
                role: "gridcell",
                aria_colindex: "1",
                display: "contents",
                {props.children}
            }
        }
    }
}

/// Remove button for the enclosing [`TagOption`].
///
/// Must be used inside [`TagOption`]. Rendering this button makes the enclosing
/// tag removable, both via click and via Delete/Backspace keyboard removal.
#[component]
pub fn TagRemoveButton(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctx: TagGroupCtx = use_context();
    let mut state = ctx.state;
    let option: TagOptionCtx = use_context();

    // Mark the enclosing tag removable while this button is mounted.
    let mut remove_button_count = option.remove_button_count;
    use_effect_with_cleanup(move || {
        *remove_button_count.write() += 1;
        move || {
            *remove_button_count.write() -= 1;
        }
    });

    let label = use_memo(move || {
        let text = state.text_value(&(option.id)());
        format!("Remove item {text}")
    });
    let can_remove = use_memo(move || state.can_remove_item(&(option.id)()));

    rsx! {
        button {
            r#type: "button",
            tabindex: "-1",
            disabled: !can_remove(),
            aria_label: "{label}",
            onclick: move |e| {
                e.stop_propagation();
                state.remove_item_from_button(&(option.id)());
            },
            ..attributes,
            {children}
        }
    }
}
