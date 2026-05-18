use super::super::component::{
    DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger,
};
use dioxus::prelude::*;
use strum::IntoEnumIterator;

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
        DropdownMenu { default_open: false,
            DropdownMenuTrigger { "Open Menu" }
            DropdownMenuContent { {operations} }
        }
        if let Some(op) = selected_operation() {
            "Selected: {op}"
        }
    }
}
