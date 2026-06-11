use dioxus::prelude::*;
use dioxus_icons::lucide::{Check, ChevronsUpDown};
use dioxus_primitives::combobox::{
    self, default_combobox_filter, AutocompleteProps, ComboboxEmptyProps, ComboboxOptionProps,
    MultiSelectProps, PillProps, PillsInputProps, TagsInputProps, VirtualizedComboboxOptionsProps,
};
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

#[css_module("/src/components/combobox/style.css")]
struct Styles;

#[derive(Props, Clone, PartialEq)]
pub struct ComboboxProps<T: Clone + PartialEq + 'static = String> {
    #[props(default)]
    pub value: Option<ReadSignal<Option<T>>>,

    #[props(default)]
    pub default_value: Option<T>,

    #[props(default)]
    pub on_value_change: Callback<Option<T>>,

    #[props(default)]
    pub disabled: ReadSignal<bool>,

    #[props(default)]
    pub open: ReadSignal<Option<bool>>,

    #[props(default)]
    pub default_open: ReadSignal<bool>,

    #[props(default)]
    pub on_open_change: Callback<bool>,

    #[props(default)]
    pub query: ReadSignal<Option<String>>,

    #[props(default)]
    pub default_query: ReadSignal<String>,

    #[props(default)]
    pub on_query_change: Callback<String>,

    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub roving_loop: ReadSignal<bool>,

    #[props(default = Callback::new(|(q, t): (String, String)| default_combobox_filter(&q, &t)))]
    pub filter: Callback<(String, String), bool>,

    #[props(default)]
    pub placeholder: ReadSignal<String>,

    #[props(default)]
    pub aria_label: Option<String>,

    #[props(default)]
    pub list_aria_label: Option<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[derive(Props, Clone, PartialEq)]
pub struct VirtualizedComboboxProps<T: Clone + PartialEq + 'static = String> {
    #[props(default)]
    pub value: Option<ReadSignal<Option<T>>>,

    #[props(default)]
    pub default_value: Option<T>,

    #[props(default)]
    pub on_value_change: Callback<Option<T>>,

    #[props(default)]
    pub disabled: ReadSignal<bool>,

    #[props(default)]
    pub open: ReadSignal<Option<bool>>,

    #[props(default)]
    pub default_open: ReadSignal<bool>,

    #[props(default)]
    pub on_open_change: Callback<bool>,

    #[props(default)]
    pub query: ReadSignal<Option<String>>,

    #[props(default)]
    pub default_query: ReadSignal<String>,

    #[props(default)]
    pub on_query_change: Callback<String>,

    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub roving_loop: ReadSignal<bool>,

    #[props(default = Callback::new(|(q, t): (String, String)| default_combobox_filter(&q, &t)))]
    pub filter: Callback<(String, String), bool>,

    #[props(default)]
    pub placeholder: ReadSignal<String>,

    #[props(default)]
    pub aria_label: Option<String>,

    #[props(default)]
    pub list_aria_label: Option<String>,

    /// The total number of source options before any virtualized visibility mapping is applied.
    pub count: ReadSignal<usize>,

    #[props(default = ReadSignal::new(Signal::new(8)))]
    pub buffer: ReadSignal<usize>,

    /// Optional visible-row to source-option index mapping for virtualized filtering.
    ///
    /// When provided, only these absolute option indices are virtualized and rendered.
    #[props(default)]
    pub visible_indices: Option<ReadSignal<Vec<usize>>>,

    pub estimate_size: Option<Callback<usize, u32>>,

    pub render_option: Callback<usize, Element>,

    #[props(default)]
    pub list_id: ReadSignal<Option<String>>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn Combobox<T: Clone + PartialEq + 'static>(props: ComboboxProps<T>) -> Element {
    let base = attributes!(div {
        class: Styles::dx_combobox
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        combobox::Combobox {
            value: props.value,
            default_value: props.default_value,
            on_value_change: props.on_value_change,
            disabled: props.disabled,
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            query: props.query,
            default_query: props.default_query,
            on_query_change: props.on_query_change,
            roving_loop: props.roving_loop,
            filter: props.filter,
            attributes: merged,
            combobox::ComboboxTarget {
                class: Styles::dx_combobox_input_wrapper,
                combobox::ComboboxSearch {
                    class: Styles::dx_combobox_input,
                    placeholder: props.placeholder,
                    aria_label: props.aria_label.clone(),
                }
                ChevronsUpDown {
                    class: Styles::dx_combobox_expand_icon,
                    size: "16px",
                }
            }
            combobox::ComboboxDropdownTarget {
                combobox::ComboboxOptions {
                    class: Styles::dx_combobox_list,
                    aria_label: props.list_aria_label.clone(),
                    {props.children}
                }
            }
        }
    }
}

#[component]
pub fn VirtualizedCombobox<T: Clone + PartialEq + 'static>(
    props: VirtualizedComboboxProps<T>,
) -> Element {
    let base = attributes!(div {
        class: Styles::dx_combobox
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        combobox::Combobox {
            value: props.value,
            default_value: props.default_value,
            on_value_change: props.on_value_change,
            disabled: props.disabled,
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            query: props.query,
            default_query: props.default_query,
            on_query_change: props.on_query_change,
            roving_loop: props.roving_loop,
            filter: props.filter,
            attributes: merged,
            combobox::ComboboxTarget {
                class: Styles::dx_combobox_input_wrapper,
                combobox::ComboboxSearch {
                    class: Styles::dx_combobox_input,
                    placeholder: props.placeholder,
                    aria_label: props.aria_label.clone(),
                }
                ChevronsUpDown {
                    class: Styles::dx_combobox_expand_icon,
                    size: "16px",
                }
            }
            combobox::ComboboxDropdownTarget {
                combobox::VirtualizedComboboxOptions {
                    class: Styles::dx_combobox_list,
                    aria_label: props.list_aria_label.clone(),
                    count: props.count,
                    visible_indices: props.visible_indices,
                    buffer: props.buffer,
                    estimate_size: props.estimate_size,
                    render_option: props.render_option,
                    id: props.list_id,
                }
            }
        }
    }
}

#[component]
pub fn Autocomplete(props: AutocompleteProps) -> Element {
    let base = attributes!(div {
        class: Styles::dx_combobox
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        combobox::Autocomplete {
            value: props.value,
            default_value: props.default_value,
            on_value_change: props.on_value_change,
            disabled: props.disabled,
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            query: props.query,
            default_query: props.default_query,
            on_query_change: props.on_query_change,
            roving_loop: props.roving_loop,
            filter: props.filter,
            placeholder: props.placeholder,
            attributes: merged,
            {props.children}
        }
    }
}

#[component]
pub fn MultiSelect<T: Clone + PartialEq + 'static>(props: MultiSelectProps<T>) -> Element {
    let base = attributes!(div {
        class: Styles::dx_combobox
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        combobox::MultiSelect::<T> {
            values: props.values,
            default_values: props.default_values,
            on_values_change: props.on_values_change,
            disabled: props.disabled,
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            query: props.query,
            default_query: props.default_query,
            on_query_change: props.on_query_change,
            roving_loop: props.roving_loop,
            filter: props.filter,
            placeholder: props.placeholder,
            max_values: props.max_values,
            render_value: props.render_value,
            attributes: merged,
            {props.children}
        }
    }
}

#[component]
pub fn PillsInput(props: PillsInputProps) -> Element {
    let base = attributes!(div {
        class: Styles::dx_combobox_input
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        combobox::PillsInput {
            disabled: props.disabled,
            attributes: merged,
            {props.children}
        }
    }
}

#[component]
pub fn Pill(props: PillProps) -> Element {
    let base = attributes!(div {
        class: Styles::dx_combobox_pill
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        combobox::Pill {
            on_remove: props.on_remove,
            attributes: merged,
            {props.children}
        }
    }
}

#[component]
pub fn TagsInput(props: TagsInputProps) -> Element {
    let base = attributes!(div {
        class: Styles::dx_combobox
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        combobox::TagsInput {
            values: props.values,
            default_values: props.default_values,
            on_values_change: props.on_values_change,
            placeholder: props.placeholder,
            allow_duplicates: props.allow_duplicates,
            disabled: props.disabled,
            attributes: merged,
        }
    }
}

#[component]
pub fn VirtualizedComboboxOptions(props: VirtualizedComboboxOptionsProps) -> Element {
    let base = attributes!(div {
        class: Styles::dx_combobox_list
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        combobox::VirtualizedComboboxOptions {
            count: props.count,
            visible_indices: props.visible_indices,
            buffer: props.buffer,
            estimate_size: props.estimate_size,
            render_option: props.render_option,
            id: props.id,
            attributes: merged,
        }
    }
}

#[component]
pub fn ComboboxEmpty(props: ComboboxEmptyProps) -> Element {
    let base = attributes!(div {
        class: Styles::dx_combobox_empty
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        combobox::ComboboxEmpty {
            attributes: merged,
            {props.children}
        }
    }
}

#[component]
pub fn ComboboxOption<T: Clone + PartialEq + 'static>(props: ComboboxOptionProps<T>) -> Element {
    let base = attributes!(div {
        class: Styles::dx_combobox_option
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        combobox::ComboboxOption::<T> {
            value: props.value,
            text_value: props.text_value,
            disabled: props.disabled,
            id: props.id,
            index: props.index,
            aria_label: props.aria_label,
            aria_roledescription: props.aria_roledescription,
            attributes: merged,
            {props.children}
            combobox::ComboboxItemIndicator {
                Check {
                    class: Styles::dx_combobox_check_icon,
                    size: "16px",
                }
            }
        }
    }
}
