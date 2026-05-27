use super::super::component::*;
use dioxus::prelude::*;
use dioxus_icons::lucide::ArrowUpRight;

#[component]
pub fn Demo() -> Element {
    rsx! {
        style { {SIZE_DEMO_STYLE} }
        div { class: "dx-button-size-demo",

            div {
                display: "flex",
                align_items: "flex-start",
                gap: "0.5rem",
                Button { size: ButtonSize::Xs, variant: ButtonVariant::Outline, "Extra Small" }
                Button {
                    size: ButtonSize::IconXs,
                    variant: ButtonVariant::Outline,
                    aria_label: "Submit",
                    ArrowUpRightIcon {}
                }
            }

            div {
                display: "flex",
                align_items: "flex-start",
                gap: "0.5rem",
                Button { size: ButtonSize::Sm, variant: ButtonVariant::Outline, "Small" }
                Button {
                    size: ButtonSize::IconSm,
                    variant: ButtonVariant::Outline,
                    aria_label: "Submit",
                    ArrowUpRightIcon {}
                }
            }

            div {
                display: "flex",
                align_items: "flex-start",
                gap: "0.5rem",
                Button { variant: ButtonVariant::Outline, "Default" }
                Button {
                    size: ButtonSize::Icon,
                    variant: ButtonVariant::Outline,
                    aria_label: "Submit",
                    ArrowUpRightIcon {}
                }
            }

            div {
                display: "flex",
                align_items: "flex-start",
                gap: "0.5rem",
                Button { size: ButtonSize::Lg, variant: ButtonVariant::Outline, "Large" }
                Button {
                    size: ButtonSize::IconLg,
                    variant: ButtonVariant::Outline,
                    aria_label: "Submit",
                    ArrowUpRightIcon {}
                }
            }
        }
    }
}

const SIZE_DEMO_STYLE: &str = r#"
.dx-button-size-demo {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  gap: 2rem;
}

@media (width >= 40rem) {
  .dx-button-size-demo {
    flex-direction: row;
  }
}
"#;

#[component]
fn ArrowUpRightIcon() -> Element {
    rsx! {
        ArrowUpRight {}
    }
}
