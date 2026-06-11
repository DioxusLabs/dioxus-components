//! Shared state for the combobox component.

use super::hook::{ComboboxDropdownEventSource, ComboboxIndexTarget, ComboboxStore};
use crate::selectable::{OptionState, SelectableContext};
use dioxus::prelude::*;

/// The default case-insensitive substring filter.
pub fn default_combobox_filter(query: &str, text: &str) -> bool {
    let query = query.trim().to_lowercase();
    query.is_empty() || text.to_lowercase().contains(&query)
}

#[derive(Clone, Copy)]
pub(super) struct ComboboxContext {
    pub selectable: SelectableContext,
    pub store: ComboboxStore,
    pub query: Memo<String>,
    pub set_query: Callback<String>,
    pub filter: Callback<(String, String), bool>,
}

impl ComboboxContext {
    fn matches_query_text(&self, text: String) -> bool {
        self.filter.call((self.query.cloned(), text))
    }

    pub fn set_open(&mut self, open: bool) {
        if open {
            self.selectable.focus_state.set_focus(None);
            self.store
                .open_dropdown(ComboboxDropdownEventSource::Unknown);
        } else {
            self.store
                .close_dropdown(ComboboxDropdownEventSource::Unknown);
        }
    }

    fn predicate_for(&self, query: String) -> impl Fn(&OptionState) -> bool {
        let filter = self.filter;
        move |option| filter.call((query.clone(), option.text_value.clone()))
    }

    fn predicate(&self) -> impl Fn(&OptionState) -> bool {
        self.predicate_for(self.query.cloned())
    }

    pub fn is_visible_text(&self, tab_index: usize, text: String) -> bool {
        let predicate = self.predicate();
        self.selectable
            .options
            .read()
            .iter()
            .find(|option| option.tab_index == tab_index)
            .map_or_else(|| self.matches_query_text(text), predicate)
    }

    pub fn has_visible_options(&self) -> bool {
        self.selectable.options.read().iter().any(self.predicate())
    }

    pub fn open_with_empty_query_and_focus_first(&mut self) {
        let query = String::new();
        self.set_query.call(query.clone());
        let initial_focus = self
            .selectable
            .first_matching_enabled_index(self.predicate_for(query));
        self.selectable.initial_focus.set(initial_focus);
        self.store.update_selected_option_index(
            initial_focus.map_or(ComboboxIndexTarget::None, |_| ComboboxIndexTarget::First),
        );
        self.set_open(true);
    }

    pub fn open_with_empty_query_and_focus_last(&mut self) {
        let query = String::new();
        self.set_query.call(query.clone());
        let initial_focus = self
            .selectable
            .last_matching_enabled_index(self.predicate_for(query));
        self.selectable.initial_focus.set(initial_focus);
        self.store.update_selected_option_index(
            initial_focus.map_or(ComboboxIndexTarget::None, |_| ComboboxIndexTarget::Last),
        );
        self.set_open(true);
    }

    pub fn focused_option_id(&self) -> Option<String> {
        self.store
            .highlighted_option_index()
            .and_then(|index| {
                self.selectable
                    .options
                    .read()
                    .iter()
                    .find(|option| option.tab_index == index && !option.disabled)
                    .map(|option| option.id.clone())
            })
            .or_else(|| self.selectable.focused_option_id())
    }

    pub fn focus_next_visible(&mut self) {
        self.selectable.focus_next_where(self.predicate());
        if let Some(index) = self.selectable.focus_state.current_focus() {
            self.store.select_option(index);
        }
    }

    pub fn focus_prev_visible(&mut self) {
        self.selectable.focus_prev_where(self.predicate());
        if let Some(index) = self.selectable.focus_state.current_focus() {
            self.store.select_option(index);
        }
    }

    pub fn focus_first_visible(&mut self) {
        self.selectable.focus_first_where(self.predicate());
        if let Some(index) = self.selectable.focus_state.current_focus() {
            self.store.select_option(index);
        }
    }

    pub fn focus_last_visible(&mut self) {
        self.selectable.focus_last_where(self.predicate());
        if let Some(index) = self.selectable.focus_state.current_focus() {
            self.store.select_option(index);
        }
    }

    pub fn select_focused(&mut self) {
        if let Some(index) = self.selectable.focus_state.current_focus() {
            self.submit_index(index, ComboboxDropdownEventSource::Keyboard);
        }
    }

    pub fn submit_index(&mut self, index: usize, source: ComboboxDropdownEventSource) {
        if self.store.select_option(index).is_none() {
            return;
        }
        self.store.submit_selected_option();
        let Some(value) = self
            .selectable
            .options
            .read()
            .iter()
            .find(|option| option.tab_index == index && !option.disabled)
            .map(|option| option.value.clone())
        else {
            return;
        };
        self.selectable.select_value(value);
        if !self.selectable.selection_mode.is_multiple() {
            self.store.close_dropdown(source);
        }
    }
}
