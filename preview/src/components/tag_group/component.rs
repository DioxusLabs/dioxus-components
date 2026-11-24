use dioxus::prelude::*;
use dioxus_icons::lucide::X;
use dioxus_primitives::tag_group::{
    self, TagGroupEmptyProps, TagGroupLabelProps, TagGroupMultiProps, TagGroupProps, TagListProps,
};

#[css_module("/src/components/tag_group/style.css")]
struct Styles;

#[component]
pub fn TagGroup(props: TagGroupProps<String>) -> Element {
    rsx! {
        tag_group::TagGroup {
            class: Styles::dx_tag_group,
            value: props.value,
            default_value: props.default_value,
            on_value_change: props.on_value_change,
            disabled: props.disabled,
            selectable: props.selectable,
            allow_empty_selection: props.allow_empty_selection,
            escape_clears_selection: props.escape_clears_selection,
            roving_loop: props.roving_loop,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn TagGroupMulti(props: TagGroupMultiProps<String>) -> Element {
    rsx! {
        tag_group::TagGroupMulti {
            class: Styles::dx_tag_group,
            values: props.values,
            default_values: props.default_values,
            on_values_change: props.on_values_change,
            disabled: props.disabled,
            selectable: props.selectable,
            allow_empty_selection: props.allow_empty_selection,
            escape_clears_selection: props.escape_clears_selection,
            roving_loop: props.roving_loop,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn TagGroupLabel(props: TagGroupLabelProps) -> Element {
    rsx! {
        tag_group::TagGroupLabel {
            class: Styles::dx_tag_group_label,
            id: props.id,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn TagGroupEmpty(props: TagGroupEmptyProps) -> Element {
    rsx! {
        tag_group::TagGroupEmpty {
            class: Styles::dx_tag_group_empty,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn TagList(props: TagListProps) -> Element {
    rsx! {
        tag_group::TagList {
            class: Styles::dx_tag_list,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct TagProps {
    pub value: ReadSignal<String>,
    #[props(default)]
    pub text_value: ReadSignal<Option<String>>,
    pub index: ReadSignal<usize>,
    #[props(default)]
    pub id: ReadSignal<Option<String>>,
    #[props(default)]
    pub disabled: ReadSignal<bool>,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    pub children: Element,
}

#[component]
pub fn Tag(props: TagProps) -> Element {
    rsx! {
        tag_group::TagOption::<String> {
            class: Styles::dx_tag,
            value: props.value,
            text_value: props.text_value,
            disabled: props.disabled,
            id: props.id,
            index: props.index,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn RemoveButton(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        tag_group::TagRemoveButton {
            class: Styles::dx_remove_button,
            attributes,
            {children}
            X { size: "12px" }
        }
    }
}
