//! Shared state and behavior for select-like listbox components.

use dioxus::html::input_data::MouseButton;
use dioxus::prelude::*;

use crate::{
    focus::{use_focus_entry_disabled, use_focus_provider, FocusState},
    listbox::{use_listbox_option, ListboxOptionContext},
    selection, use_controlled, Controlled,
};

pub(crate) use crate::selection::{OptionState, RcPartialEqValue};

/// Whether selecting an option should replace the current value or toggle it.
#[derive(Clone, Copy, PartialEq)]
pub(crate) enum SelectionMode {
    /// A single value is selected and the popup closes after selection.
    Single,
    /// Multiple values can be selected and the popup stays open after selection.
    Multiple,
}

impl SelectionMode {
    pub(crate) fn is_multiple(self) -> bool {
        matches!(self, Self::Multiple)
    }

    fn closes_on_select(self) -> bool {
        matches!(self, Self::Single)
    }
}

/// Shared context for components built around a selectable listbox.
#[derive(Clone, Copy)]
pub(crate) struct SelectableContext {
    pub(crate) open: Memo<bool>,
    pub(crate) set_open: Callback<bool>,
    pub(crate) values: Memo<Vec<RcPartialEqValue>>,
    pub(crate) set_value: Callback<RcPartialEqValue>,
    pub(crate) selection_mode: SelectionMode,
    pub(crate) options: Signal<Vec<OptionState>>,
    pub(crate) list_id: Signal<Option<String>>,
    pub(crate) focus_state: FocusState,
    pub(crate) initial_focus: Signal<Option<usize>>,
    pub(crate) disabled: ReadSignal<bool>,
}

#[derive(Clone, Copy)]
pub(crate) struct SelectableOption<T: Clone + PartialEq + 'static> {
    pub(crate) id: Memo<String>,
    pub(crate) disabled: Memo<bool>,
    pub(crate) selected: Memo<bool>,
    pub(crate) focused: Memo<bool>,
    pub(crate) down_pos: Signal<Option<(f64, f64)>>,
    pub(crate) index: ReadSignal<usize>,
    pub(crate) value: ReadSignal<T>,
}

pub(crate) struct SelectableOptionConfig<T: Clone + PartialEq + 'static> {
    pub(crate) id: ReadSignal<Option<String>>,
    pub(crate) index: ReadSignal<usize>,
    pub(crate) value: ReadSignal<T>,
    pub(crate) text_value: ReadSignal<Option<String>>,
    pub(crate) option_disabled: ReadSignal<bool>,
    pub(crate) component_name: &'static str,
}

impl SelectableContext {
    pub(crate) fn set_open(&mut self, open: bool) {
        self.set_open.call(open);
    }

    pub(crate) fn toggle_open(&mut self) {
        self.set_open(!self.open.cloned());
    }

    pub(crate) fn selected_text(&self) -> Option<String> {
        let values = self.values.read();
        let options = self.options.read();
        selection::selected_text(values.iter(), &options)
    }

    pub(crate) fn is_selected(&self, value: &RcPartialEqValue) -> bool {
        self.values.read().iter().any(|selected| selected == value)
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.values.read().is_empty()
    }

    pub(crate) fn focused_option_id(&self) -> Option<String> {
        let index = self.focus_state.current_focus()?;
        if !self.focus_state.is_enabled(index) {
            return None;
        }
        self.options
            .read()
            .iter()
            .find(|option| option.tab_index == index && !option.disabled)
            .map(|option| option.id.clone())
    }

    pub(crate) fn select_focused(&mut self) {
        if !self.open.cloned() {
            return;
        }
        let Some(index) = self.focus_state.current_focus() else {
            return;
        };
        if !self.focus_state.is_enabled(index) {
            return;
        }
        let value = self
            .options
            .read()
            .iter()
            .find(|option| option.tab_index == index && !option.disabled)
            .map(|option| option.value.clone());
        if let Some(value) = value {
            self.select_value(value);
        }
    }

    fn matching_enabled_indices(&self, predicate: impl Fn(&OptionState) -> bool) -> Vec<usize> {
        let mut indices: Vec<_> = self
            .options
            .read()
            .iter()
            .filter(|option| !option.disabled && predicate(option))
            .map(|option| option.tab_index)
            .collect();
        indices.sort_unstable();
        indices.dedup();
        indices
    }

    pub(crate) fn first_matching_enabled_index(
        &self,
        predicate: impl Fn(&OptionState) -> bool,
    ) -> Option<usize> {
        self.matching_enabled_indices(predicate).first().copied()
    }

    pub(crate) fn last_matching_enabled_index(
        &self,
        predicate: impl Fn(&OptionState) -> bool,
    ) -> Option<usize> {
        self.matching_enabled_indices(predicate).last().copied()
    }

    pub(crate) fn focus_next_where(&mut self, predicate: impl Fn(&OptionState) -> bool) {
        let indices = self.matching_enabled_indices(predicate);
        self.focus_state.focus_next_from_current(&indices);
    }

    pub(crate) fn focus_prev_where(&mut self, predicate: impl Fn(&OptionState) -> bool) {
        let indices = self.matching_enabled_indices(predicate);
        self.focus_state.focus_prev_from_current(&indices);
    }

    pub(crate) fn focus_first_where(&mut self, predicate: impl Fn(&OptionState) -> bool) {
        let index = self.first_matching_enabled_index(predicate);
        self.focus_state.set_focus(index);
    }

    pub(crate) fn focus_last_where(&mut self, predicate: impl Fn(&OptionState) -> bool) {
        let index = self.last_matching_enabled_index(predicate);
        self.focus_state.set_focus(index);
    }

    pub(crate) fn select_value(&mut self, value: RcPartialEqValue) {
        self.set_value.call(value);
        if self.selection_mode.closes_on_select() {
            self.set_open(false);
        }
    }
}

pub(crate) fn use_single_selectable_value<T: Clone + PartialEq + 'static>(
    controlled_value: Option<ReadSignal<Option<T>>>,
    default_value: Option<T>,
    on_change: Callback<Option<T>>,
    component_name: &'static str,
) -> (Memo<Vec<RcPartialEqValue>>, Callback<RcPartialEqValue>) {
    let mut internal_value: Signal<Option<T>> = use_signal(|| default_value.clone());
    let value = use_memo(move || match controlled_value {
        Some(value) => value.cloned(),
        None => internal_value.cloned(),
    });
    let values = use_memo(move || value().map(RcPartialEqValue::new).into_iter().collect());
    let set_value = use_callback(move |incoming: RcPartialEqValue| {
        let value = incoming
            .as_ref::<T>()
            .unwrap_or_else(|| panic!("{component_name} and option value types must match"))
            .clone();
        internal_value.set(Some(value.clone()));
        on_change.call(Some(value));
    });

    (values, set_value)
}

pub(crate) fn use_selectable_root(
    values: Memo<Vec<RcPartialEqValue>>,
    set_value: Callback<RcPartialEqValue>,
    selection_mode: SelectionMode,
    disabled: ReadSignal<bool>,
    roving_loop: ReadSignal<bool>,
    open: Controlled<bool>,
) -> SelectableContext {
    let (open, set_open) = use_controlled(open.value, open.default.cloned(), open.on_change);
    let options: Signal<Vec<OptionState>> = use_signal(Vec::default);
    let list_id = use_signal(|| None);
    let focus_state = use_focus_provider(roving_loop);
    let initial_focus = use_signal(|| None);

    SelectableContext {
        open,
        set_open,
        values,
        set_value,
        selection_mode,
        options,
        list_id,
        focus_state,
        initial_focus,
        disabled,
    }
}

pub(crate) fn use_selectable_option<T: Clone + PartialEq + 'static>(
    selectable: SelectableContext,
    option: SelectableOptionConfig<T>,
) -> SelectableOption<T> {
    let SelectableOptionConfig {
        id,
        index,
        value,
        text_value,
        option_disabled,
        component_name,
    } = option;
    let disabled = {
        let root_disabled = selectable.disabled;
        use_memo(move || root_disabled.cloned() || option_disabled.cloned())
    };
    let id = use_listbox_option(
        id,
        index,
        value,
        text_value,
        selectable.options,
        move || disabled.cloned(),
        component_name,
    );
    use_focus_entry_disabled(selectable.focus_state, index, move || disabled.cloned());
    let selected = use_memo(move || selectable.is_selected(&RcPartialEqValue::new(value.cloned())));
    let focused = use_memo(move || selectable.focus_state.is_focused(index()));
    let down_pos: Signal<Option<(f64, f64)>> = use_signal(|| None);

    use_context_provider(|| ListboxOptionContext {
        selected: selected.into(),
    });

    SelectableOption {
        id,
        disabled,
        selected,
        focused,
        down_pos,
        index,
        value,
    }
}

pub(crate) fn pointer_select_start(
    event: &Event<PointerData>,
    disabled: bool,
    mut down_pos: Signal<Option<(f64, f64)>>,
) {
    if disabled || event.trigger_button() != Some(MouseButton::Primary) {
        return;
    }
    event.prevent_default();
    let p = event.client_coordinates();
    down_pos.set(Some((p.x, p.y)));
}

pub(crate) fn pointer_select_commit(
    event: &Event<PointerData>,
    disabled: bool,
    mut down_pos: Signal<Option<(f64, f64)>>,
) -> bool {
    if disabled || event.trigger_button() != Some(MouseButton::Primary) {
        return false;
    }
    let Some((x0, y0)) = down_pos.take() else {
        return false;
    };
    if event.pointer_type() == "touch" {
        let p = event.client_coordinates();
        let dx = p.x - x0;
        let dy = p.y - y0;
        if dx * dx + dy * dy > 25.0 {
            return false;
        }
    }
    true
}

pub(crate) fn pointer_select_cancel(mut down_pos: Signal<Option<(f64, f64)>>) {
    down_pos.set(None);
}
