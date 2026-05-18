//! Context types and implementations for the select component.

use dioxus::prelude::*;
use dioxus_core::Task;
use dioxus_sdk_time::sleep;

use std::time::Duration;

use super::text_search::AdaptiveKeyboard;
use crate::selectable::SelectableContext;

/// Main context for the select component containing all shared state
#[derive(Clone, Copy)]
pub(super) struct SelectContext {
    /// Shared selectable listbox state.
    pub selectable: SelectableContext,
    /// Adaptive keyboard system for multi-language support
    pub adaptive_keyboard: Signal<AdaptiveKeyboard>,
    /// The typeahead buffer for searching options
    pub typeahead_buffer: Signal<String>,
    /// The ID of the list for ARIA attributes
    pub typeahead_clear_task: Signal<Option<Task>>,
    /// Timeout before clearing typeahead buffer
    pub typeahead_timeout: ReadSignal<Duration>,
}

impl SelectContext {
    pub fn set_open(&mut self, open: bool) {
        self.selectable.set_open(open);
    }

    pub fn multi(&self) -> bool {
        self.selectable.selection_mode.is_multiple()
    }

    /// Select the currently focused item
    pub fn select_current_item(&mut self) {
        self.selectable.select_focused();
    }

    /// Learn from a keyboard event mapping physical key to logical character
    pub fn learn_from_keyboard_event(&mut self, physical_code: &str, logical_char: char) {
        let mut adaptive = self.adaptive_keyboard.write();
        let logical_char = logical_char.to_lowercase().next().unwrap_or(logical_char);
        adaptive.learn_from_event(physical_code, logical_char);
    }

    /// Add text to the typeahead buffer for searching
    pub fn add_to_typeahead_buffer(&mut self, text: &str) {
        // Cancel any existing clear task to prevent race conditions
        if let Some(existing_task) = self.typeahead_clear_task.write().take() {
            existing_task.cancel();
        }

        // Update the buffer and get the current content
        let typeahead = {
            let mut typeahead_buffer = self.typeahead_buffer.write();
            typeahead_buffer.push_str(text);
            typeahead_buffer.clone()
        };

        // Create references for the async closure
        let mut typeahead_buffer_signal = self.typeahead_buffer;
        let mut typeahead_clear_task_signal = self.typeahead_clear_task;

        // Spawn a new task to clear the buffer after the configured timeout
        let timeout = self.typeahead_timeout.cloned();
        let new_task = spawn(async move {
            sleep(timeout).await;

            // Clear the buffer
            typeahead_buffer_signal.write().clear();

            // Remove our own task handle to indicate no task is active
            typeahead_clear_task_signal.write().take();
        });

        // Store the new task handle
        self.typeahead_clear_task.write().replace(new_task);

        // Focus the best match using adaptive keyboard
        let options = self.selectable.options.read();
        let keyboard = self.adaptive_keyboard.read();

        if let Some(best_match_index) =
            super::text_search::best_match(&keyboard, &typeahead, &options)
        {
            self.selectable
                .focus_state
                .set_focus(Some(best_match_index));
        }
    }
}

/// Context for select group components
#[derive(Clone, Copy)]
pub(super) struct SelectGroupContext {
    /// ID of the element that labels this group
    pub labeled_by: Signal<Option<String>>,
}
