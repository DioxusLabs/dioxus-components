//! Combobox input and search components.

use dioxus::prelude::*;

use super::target::render_combobox_search;

/// Props for [`ComboboxInput`].
#[derive(Props, Clone, PartialEq)]
pub struct ComboboxInputProps {
    /// Placeholder shown when the input is empty.
    #[props(default)]
    pub placeholder: ReadSignal<String>,

    /// Optional id for the input element.
    #[props(default)]
    pub id: ReadSignal<Option<String>>,

    /// Additional attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// Compatibility input that acts as the target, events target, and search input.
#[component]
pub fn ComboboxInput(props: ComboboxInputProps) -> Element {
    render_combobox_search(props.placeholder, props.id, props.attributes, true, true)
}

/// Props for [`ComboboxSearch`].
#[derive(Props, Clone, PartialEq)]
pub struct ComboboxSearchProps {
    /// Placeholder shown when the input is empty.
    #[props(default)]
    pub placeholder: ReadSignal<String>,

    /// Optional id for the input element.
    #[props(default)]
    pub id: ReadSignal<Option<String>>,

    /// Whether to show selected option text while the dropdown is closed.
    #[props(default = true)]
    pub show_selected_text: bool,

    /// Additional attributes.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// Search input for split combobox anatomy.
#[component]
pub fn ComboboxSearch(props: ComboboxSearchProps) -> Element {
    render_combobox_search(
        props.placeholder,
        props.id,
        props.attributes,
        false,
        props.show_selected_text,
    )
}
