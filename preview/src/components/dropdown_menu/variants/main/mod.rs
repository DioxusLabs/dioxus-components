use super::super::component::{
    DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger,
};
use dioxus::prelude::*;
use strum::IntoEnumIterator;

#[css_module("/src/components/dropdown_menu/style.css")]
struct Styles;

#[derive(Clone, Copy, strum::Display, strum::EnumIter, PartialEq)]
enum Operation {
    Edit,
    Undo,
    Duplicate,
    Delete,
}

#[component]
pub fn Demo() -> Element {
    let mut selected_operation = use_signal(|| None);

    let operations = Operation::iter().enumerate().map(|(i, o)| {
        rsx! {
            DropdownMenuItem::<Operation> {
                class: Styles::dx_dropdown_menu_item,
                value: o,
                index: i,
                disabled: matches!(o, Operation::Undo),
                on_select: move |value| {
                    selected_operation.set(Some(value));
                },
                {o.to_string()}
            }
        }
    });

    rsx! {
        DropdownMenu { class: Styles::dx_dropdown_menu, default_open: false,
            DropdownMenuTrigger { class: Styles::dx_dropdown_menu_trigger, "Open Menu" }
            DropdownMenuContent { class: Styles::dx_dropdown_menu_content, {operations} }
        }
        if let Some(op) = selected_operation() {
            "Selected: {op}"
        }
    }
}
