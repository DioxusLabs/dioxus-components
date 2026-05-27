use crate::components::{
    button::{Button, ButtonVariant},
    input::Input,
    label::Label,
    sheet::{
        Sheet, SheetClose, SheetContentClose, SheetDescription, SheetFooter, SheetHeader,
        SheetSide, SheetTitle,
    },
};
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut open = use_signal(|| false);
    let mut side = use_signal(|| SheetSide::Right);

    let open_sheet = move |s: SheetSide| {
        move |_| {
            side.set(s);
            open.set(true);
        }
    };

    rsx! {
        div { display: "flex", gap: "0.5rem",
            Button { variant: ButtonVariant::Outline, onclick: open_sheet(SheetSide::Top), "Top" }
            Button { variant: ButtonVariant::Outline, onclick: open_sheet(SheetSide::Right), "Right" }
            Button { variant: ButtonVariant::Outline, onclick: open_sheet(SheetSide::Bottom), "Bottom" }
            Button { variant: ButtonVariant::Outline, onclick: open_sheet(SheetSide::Left), "Left" }
        }
        Sheet { open: open(), on_open_change: move |v| open.set(v), "data-side": side().as_str(),
            SheetHeader {
                SheetTitle { "Sheet Title" }
                SheetDescription { "Sheet description goes here." }
            }

            div {
                display: "grid",
                flex: "1 1 0%",
                grid_auto_rows: "min-content",
                gap: "1.5rem",
                padding: "0 1rem",
                div { display: "grid", gap: "0.75rem",
                    Label { html_for: "sheet-demo-name", "Name" }
                    Input {
                        id: "sheet-demo-name",
                        initial_value: "Dioxus",
                    }
                }
                div { display: "grid", gap: "0.75rem",
                    Label { html_for: "sheet-demo-username", "Username" }
                    Input {
                        id: "sheet-demo-username",
                        initial_value: "@dioxus",
                    }
                }
            }

            SheetFooter {
                Button { "Save changes" }
                SheetClose {
                    as: |attributes| rsx! {
                        Button { variant: ButtonVariant::Outline, attributes, "Cancel" }
                    },
                }
            }
            SheetContentClose {}
        }
    }
}
