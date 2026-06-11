//! Value-agnostic combobox interaction store.

use std::rc::Rc;

use dioxus::prelude::*;

use crate::disclosure::{use_disclosure, Disclosure, UseDisclosureOptions};

/// The user interaction source that requested a dropdown state change.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComboboxDropdownEventSource {
    /// Keyboard interaction.
    Keyboard,
    /// Pointer or mouse interaction.
    Mouse,
    /// Programmatic or unknown interaction.
    Unknown,
}

/// Target used when updating the highlighted option index.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComboboxIndexTarget {
    /// Clear the highlighted option.
    None,
    /// Highlight the first enabled and visible option.
    First,
    /// Highlight the last enabled and visible option.
    Last,
    /// Highlight the active option when one is registered.
    Active,
    /// Keep the current highlighted option if still enabled and visible.
    Selected,
}

/// Stable option key returned by combobox navigation methods.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ComboboxOptionKey {
    /// DOM id registered by the option.
    pub id: String,
    /// Navigation index registered by the option.
    pub index: usize,
}

/// Metadata for an option submitted through the combobox store.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ComboboxSubmittedOption {
    /// DOM id registered by the option.
    pub id: String,
    /// Navigation index registered by the option.
    pub index: usize,
}

/// Options for [`use_combobox`].
#[derive(Clone, Copy)]
pub struct UseComboboxOptions {
    /// Controlled dropdown open state.
    pub opened: ReadSignal<Option<bool>>,
    /// Initial dropdown open state when uncontrolled.
    pub default_opened: ReadSignal<bool>,
    /// Callback fired when the dropdown open state changes.
    pub on_opened_change: Callback<bool>,
    /// Callback fired on closed-to-open dropdown transitions.
    pub on_dropdown_open: Option<EventHandler<ComboboxDropdownEventSource>>,
    /// Callback fired on open-to-closed dropdown transitions.
    pub on_dropdown_close: Option<EventHandler<ComboboxDropdownEventSource>>,
    /// Whether keyboard navigation wraps at list boundaries.
    pub loop_navigation: ReadSignal<bool>,
}

/// Options for [`use_virtualized_combobox`].
#[derive(Clone, Copy, Default)]
pub struct UseVirtualizedComboboxOptions {
    /// Base combobox interaction options.
    pub combobox: UseComboboxOptions,
}

/// Store returned by [`use_virtualized_combobox`].
pub type VirtualizedComboboxStore = ComboboxStore;

impl Default for UseComboboxOptions {
    fn default() -> Self {
        Self {
            opened: ReadSignal::new(Signal::new(None)),
            default_opened: ReadSignal::new(Signal::new(false)),
            on_opened_change: Callback::default(),
            on_dropdown_open: None,
            on_dropdown_close: None,
            loop_navigation: ReadSignal::new(Signal::new(true)),
        }
    }
}

#[derive(Clone, PartialEq)]
struct ComboboxOptionState {
    key: ComboboxOptionKey,
    disabled: bool,
    visible: bool,
    active: bool,
}

impl ComboboxOptionState {
    fn enabled_visible(&self) -> bool {
        !self.disabled && self.visible
    }
}

/// A cloneable combobox store handle.
#[derive(Clone, Copy)]
pub struct ComboboxStore {
    disclosure: Disclosure,
    options: Signal<Vec<ComboboxOptionState>>,
    highlighted_index: Signal<Option<usize>>,
    submitted_option: Signal<Option<ComboboxSubmittedOption>>,
    target_mount: Signal<Option<Rc<MountedData>>>,
    search_mount: Signal<Option<Rc<MountedData>>>,
    on_dropdown_open: Option<EventHandler<ComboboxDropdownEventSource>>,
    on_dropdown_close: Option<EventHandler<ComboboxDropdownEventSource>>,
    loop_navigation: ReadSignal<bool>,
}

impl ComboboxStore {
    /// Returns whether the dropdown is open.
    pub fn dropdown_opened(&self) -> bool {
        self.disclosure.opened()
    }

    /// Opens the dropdown and reports the event source on transition.
    pub fn open_dropdown(&self, source: ComboboxDropdownEventSource) {
        if self.disclosure.open() {
            if let Some(on_open) = self.on_dropdown_open {
                on_open.call(source);
            }
        }
    }

    /// Closes the dropdown and reports the event source on transition.
    pub fn close_dropdown(&self, source: ComboboxDropdownEventSource) {
        if self.disclosure.close() {
            self.reset_selected_option();
            if let Some(on_close) = self.on_dropdown_close {
                on_close.call(source);
            }
        }
    }

    /// Toggles the dropdown and reports the event source on transition.
    pub fn toggle_dropdown(&self, source: ComboboxDropdownEventSource) {
        match self.disclosure.toggle() {
            Some(true) => {
                if let Some(on_open) = self.on_dropdown_open {
                    on_open.call(source);
                }
            }
            Some(false) => {
                self.reset_selected_option();
                if let Some(on_close) = self.on_dropdown_close {
                    on_close.call(source);
                }
            }
            None => {}
        }
    }

    /// Returns the currently highlighted option index.
    pub fn highlighted_option_index(&self) -> Option<usize> {
        (self.highlighted_index)()
    }

    /// Highlights the enabled, visible option with the given index.
    pub fn select_option(&self, index: usize) -> Option<ComboboxOptionKey> {
        let key = self
            .options
            .read()
            .iter()
            .find(|option| option.key.index == index && option.enabled_visible())
            .map(|option| option.key.clone())?;
        let mut highlighted_index = self.highlighted_index;
        highlighted_index.set(Some(key.index));
        Some(key)
    }

    /// Highlights the first enabled and visible option.
    pub fn select_first_option(&self) -> Option<ComboboxOptionKey> {
        let key = first_enabled_visible(&self.options.read())?;
        let mut highlighted_index = self.highlighted_index;
        highlighted_index.set(Some(key.index));
        Some(key)
    }

    /// Highlights the active option, or the first enabled visible option when no
    /// active option is registered.
    pub fn select_active_option(&self) -> Option<ComboboxOptionKey> {
        let options = self.options.read();
        let key = options
            .iter()
            .find(|option| option.active && option.enabled_visible())
            .or_else(|| options.iter().find(|option| option.enabled_visible()))
            .map(|option| option.key.clone())?;
        drop(options);
        let mut highlighted_index = self.highlighted_index;
        highlighted_index.set(Some(key.index));
        Some(key)
    }

    /// Highlights the next enabled and visible option.
    pub fn select_next_option(&self) -> Option<ComboboxOptionKey> {
        let key = next_enabled_visible(
            &self.options.read(),
            self.highlighted_option_index(),
            (self.loop_navigation)(),
        )?;
        let mut highlighted_index = self.highlighted_index;
        highlighted_index.set(Some(key.index));
        Some(key)
    }

    /// Highlights the previous enabled and visible option.
    pub fn select_previous_option(&self) -> Option<ComboboxOptionKey> {
        let key = previous_enabled_visible(
            &self.options.read(),
            self.highlighted_option_index(),
            (self.loop_navigation)(),
        )?;
        let mut highlighted_index = self.highlighted_index;
        highlighted_index.set(Some(key.index));
        Some(key)
    }

    /// Clears the highlighted option.
    pub fn reset_selected_option(&self) {
        let mut highlighted_index = self.highlighted_index;
        highlighted_index.set(None);
    }

    /// Updates the highlighted option according to the requested target.
    pub fn update_selected_option_index(&self, target: ComboboxIndexTarget) {
        match target {
            ComboboxIndexTarget::None => self.reset_selected_option(),
            ComboboxIndexTarget::First => {
                self.select_first_option();
            }
            ComboboxIndexTarget::Last => {
                if let Some(key) = last_enabled_visible(&self.options.read()) {
                    let mut highlighted_index = self.highlighted_index;
                    highlighted_index.set(Some(key.index));
                }
            }
            ComboboxIndexTarget::Active => {
                self.select_active_option();
            }
            ComboboxIndexTarget::Selected => {
                if self
                    .highlighted_option_index()
                    .and_then(|index| self.select_option(index))
                    .is_none()
                {
                    self.reset_selected_option();
                }
            }
        }
    }

    /// Returns the most recently submitted option metadata.
    pub fn submitted_option(&self) -> Option<ComboboxSubmittedOption> {
        (self.submitted_option)()
    }

    /// Requests submission of the currently highlighted option.
    pub fn submit_selected_option(&self) -> Option<ComboboxSubmittedOption> {
        let index = self.highlighted_option_index()?;
        let key = self.select_option(index)?;
        let submitted = ComboboxSubmittedOption {
            id: key.id,
            index: key.index,
        };
        let mut submitted_option = self.submitted_option;
        submitted_option.set(Some(submitted.clone()));
        Some(submitted)
    }

    /// Focuses the element registered through
    /// [`use_combobox_target`](crate::combobox::use_combobox_target)
    /// when mounted.
    pub fn focus_target(&self) {
        focus_mounted(self.target_mount);
    }

    /// Focuses the input registered through
    /// [`use_combobox_search`](crate::combobox::use_combobox_search)
    /// when mounted.
    pub fn focus_search_input(&self) {
        focus_mounted(self.search_mount);
    }

    pub(crate) fn register_option(
        &self,
        id: String,
        index: usize,
        disabled: bool,
        visible: bool,
        active: bool,
    ) {
        let mut options = self.options;
        sync_combobox_option(
            &mut options.write(),
            ComboboxOptionState {
                key: ComboboxOptionKey { id, index },
                disabled,
                visible,
                active,
            },
        );
        if self
            .highlighted_option_index()
            .is_some_and(|idx| !has_enabled_visible_index(&self.options.read(), idx))
        {
            self.reset_selected_option();
        }
    }

    pub(crate) fn unregister_option(&self, id: &str) {
        let mut options = self.options;
        options.write().retain(|option| option.key.id != id);
        if self
            .highlighted_option_index()
            .is_some_and(|idx| !has_enabled_visible_index(&self.options.read(), idx))
        {
            self.reset_selected_option();
        }
    }

    pub(crate) fn register_target_mount_ref(&self, mounted: Rc<MountedData>) {
        let mut target_mount = self.target_mount;
        target_mount.set(Some(mounted));
    }

    pub(crate) fn register_search_mount_ref(&self, mounted: Rc<MountedData>) {
        let mut search_mount = self.search_mount;
        search_mount.set(Some(mounted));
    }
}

/// Create a value-agnostic combobox interaction store.
pub fn use_combobox(options: UseComboboxOptions) -> ComboboxStore {
    let disclosure = use_disclosure(UseDisclosureOptions {
        opened: options.opened,
        default_opened: options.default_opened,
        on_opened_change: options.on_opened_change,
    });
    let options_signal = use_signal(Vec::new);
    let highlighted_index = use_signal(|| None);
    let submitted_option = use_signal(|| None);
    let target_mount = use_signal(|| None);
    let search_mount = use_signal(|| None);

    ComboboxStore {
        disclosure,
        options: options_signal,
        highlighted_index,
        submitted_option,
        target_mount,
        search_mount,
        on_dropdown_open: options.on_dropdown_open,
        on_dropdown_close: options.on_dropdown_close,
        loop_navigation: options.loop_navigation,
    }
}

/// Create a combobox store for virtualized listbox usage.
///
/// Virtualization is handled by [`VirtualizedComboboxOptions`](crate::combobox::VirtualizedComboboxOptions);
/// this hook keeps the same value-agnostic interaction surface as [`use_combobox`].
pub fn use_virtualized_combobox(
    options: UseVirtualizedComboboxOptions,
) -> VirtualizedComboboxStore {
    use_combobox(options.combobox)
}

fn focus_mounted(mount: Signal<Option<Rc<MountedData>>>) {
    if let Some(md) = mount() {
        spawn(async move {
            let _ = md.set_focus(true).await;
        });
    }
}

fn sync_combobox_option(options: &mut Vec<ComboboxOptionState>, option: ComboboxOptionState) {
    if let Some(position) = options.iter().position(|item| item.key.id == option.key.id) {
        if options[position].key.index == option.key.index {
            options[position] = option;
        } else {
            options.remove(position);
            insert_combobox_option(options, option);
        }
    } else {
        insert_combobox_option(options, option);
    }
}

fn insert_combobox_option(options: &mut Vec<ComboboxOptionState>, option: ComboboxOptionState) {
    let insert_at = options.partition_point(|item| item.key.index <= option.key.index);
    options.insert(insert_at, option);
}

fn has_enabled_visible_index(options: &[ComboboxOptionState], index: usize) -> bool {
    options
        .iter()
        .any(|option| option.key.index == index && option.enabled_visible())
}

fn first_enabled_visible(options: &[ComboboxOptionState]) -> Option<ComboboxOptionKey> {
    options
        .iter()
        .find(|option| option.enabled_visible())
        .map(|option| option.key.clone())
}

fn last_enabled_visible(options: &[ComboboxOptionState]) -> Option<ComboboxOptionKey> {
    options
        .iter()
        .rev()
        .find(|option| option.enabled_visible())
        .map(|option| option.key.clone())
}

fn next_enabled_visible(
    options: &[ComboboxOptionState],
    current: Option<usize>,
    loop_navigation: bool,
) -> Option<ComboboxOptionKey> {
    match current {
        Some(current) => options
            .iter()
            .find(|option| option.key.index > current && option.enabled_visible())
            .or_else(|| {
                loop_navigation
                    .then(|| options.iter().find(|option| option.enabled_visible()))
                    .flatten()
            }),
        None => options.iter().find(|option| option.enabled_visible()),
    }
    .map(|option| option.key.clone())
}

fn previous_enabled_visible(
    options: &[ComboboxOptionState],
    current: Option<usize>,
    loop_navigation: bool,
) -> Option<ComboboxOptionKey> {
    match current {
        Some(current) => options
            .iter()
            .rev()
            .find(|option| option.key.index < current && option.enabled_visible())
            .or_else(|| {
                loop_navigation
                    .then(|| options.iter().rev().find(|option| option.enabled_visible()))
                    .flatten()
            }),
        None if loop_navigation => options.iter().rev().find(|option| option.enabled_visible()),
        None => options.iter().find(|option| option.enabled_visible()),
    }
    .map(|option| option.key.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn option(id: &str, index: usize) -> ComboboxOptionState {
        ComboboxOptionState {
            key: ComboboxOptionKey {
                id: id.to_string(),
                index,
            },
            disabled: false,
            visible: true,
            active: false,
        }
    }

    #[test]
    fn sync_combobox_option_keeps_index_order() {
        let mut options = vec![option("a", 0), option("c", 2)];

        sync_combobox_option(&mut options, option("b", 1));

        let ids: Vec<_> = options
            .iter()
            .map(|option| option.key.id.as_str())
            .collect();
        assert_eq!(ids, ["a", "b", "c"]);
    }

    #[test]
    fn navigation_skips_disabled_and_invisible_options() {
        let options = vec![
            option("a", 0),
            ComboboxOptionState {
                disabled: true,
                ..option("b", 1)
            },
            ComboboxOptionState {
                visible: false,
                ..option("c", 2)
            },
            option("d", 3),
        ];

        assert_eq!(
            next_enabled_visible(&options, Some(0), true).unwrap().id,
            "d"
        );
        assert_eq!(
            previous_enabled_visible(&options, Some(3), true)
                .unwrap()
                .id,
            "a"
        );
    }

    #[test]
    fn navigation_respects_loop_navigation_setting() {
        let options = vec![option("a", 0), option("b", 1)];

        assert_eq!(
            next_enabled_visible(&options, Some(1), true).unwrap().id,
            "a"
        );
        assert!(next_enabled_visible(&options, Some(1), false).is_none());

        assert_eq!(
            previous_enabled_visible(&options, Some(0), true)
                .unwrap()
                .id,
            "b"
        );
        assert!(previous_enabled_visible(&options, Some(0), false).is_none());
    }

    #[test]
    fn navigation_without_current_uses_first_or_last_consistently() {
        let options = vec![option("a", 0), option("b", 1)];

        assert_eq!(next_enabled_visible(&options, None, true).unwrap().id, "a");
        assert_eq!(
            previous_enabled_visible(&options, None, true).unwrap().id,
            "b"
        );
        assert_eq!(
            previous_enabled_visible(&options, None, false).unwrap().id,
            "a"
        );
    }

    #[test]
    fn first_last_and_active_skip_unselectable_options() {
        let options = vec![
            ComboboxOptionState {
                disabled: true,
                active: true,
                ..option("a", 0)
            },
            ComboboxOptionState {
                visible: false,
                active: true,
                ..option("b", 1)
            },
            option("c", 2),
        ];

        assert_eq!(first_enabled_visible(&options).unwrap().id, "c");
        assert_eq!(last_enabled_visible(&options).unwrap().id, "c");
    }
}
