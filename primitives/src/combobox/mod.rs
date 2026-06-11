//! Autocomplete input with a filterable popup list.
//!
//! `ComboboxInput` is the text input and trigger. `ComboboxList` contains
//! `ComboboxOption` children. Split anatomy is available through
//! `ComboboxTarget`, `ComboboxEventsTarget`, `ComboboxDropdownTarget`,
//! `ComboboxSearch`, and `ComboboxOptions`.

mod components;
mod context;
mod hook;

pub use components::{
    use_combobox_dropdown_target, use_combobox_dropdown_target_attributes,
    use_combobox_events_target, use_combobox_events_target_attributes, use_combobox_search,
    use_combobox_search_attributes, use_combobox_target, use_combobox_target_attributes,
    Autocomplete, AutocompleteProps, Combobox, ComboboxDropdownTarget,
    ComboboxDropdownTargetHandle, ComboboxDropdownTargetProps, ComboboxEmpty, ComboboxEmptyProps,
    ComboboxEventsTarget, ComboboxEventsTargetHandle, ComboboxEventsTargetProps, ComboboxInput,
    ComboboxInputProps, ComboboxItemIndicator, ComboboxItemIndicatorProps, ComboboxList,
    ComboboxListProps, ComboboxOption, ComboboxOptionProps, ComboboxOptions, ComboboxOptionsProps,
    ComboboxProps, ComboboxSearch, ComboboxSearchHandle, ComboboxSearchProps, ComboboxTarget,
    ComboboxTargetHandle, ComboboxTargetProps, MultiSelect, MultiSelectProps, Pill, PillProps,
    PillsInput, PillsInputProps, TagsInput, TagsInputProps, UseComboboxSearchOptions,
    VirtualizedComboboxOptions, VirtualizedComboboxOptionsProps,
};

pub use context::default_combobox_filter;
pub use hook::{
    use_combobox, use_virtualized_combobox, ComboboxDropdownEventSource, ComboboxIndexTarget,
    ComboboxOptionKey, ComboboxStore, ComboboxSubmittedOption, UseComboboxOptions,
    UseVirtualizedComboboxOptions, VirtualizedComboboxStore,
};
