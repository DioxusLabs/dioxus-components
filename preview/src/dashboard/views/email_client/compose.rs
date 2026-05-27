use dioxus::prelude::*;
use dioxus_primitives::toast::{use_toast, ToastOptions};

use crate::components::button::{Button, ButtonVariant};
use crate::components::dialog::{Dialog, DialogDescription, DialogTitle};
use crate::components::input::Input;
use crate::components::label::Label;
use crate::components::separator::Separator;
use crate::components::textarea::{Textarea, TextareaVariant};
use crate::dashboard::common::{IconKind, LucideIcon};

use super::state::{EmailClientState, EmailClientStateStoreExt, EmailClientStateStoreImplExt};

#[component]
pub(super) fn ComposeModal(mut state: Store<EmailClientState>) -> Element {
    let toasts = use_toast();
    let open = state.compose_open().cloned();
    let to = state.compose_to().cloned();
    let subject = state.compose_subject().cloned();
    let body = state.compose_body().cloned();
    let recipient = to.clone();

    let send = move |evt: FormEvent| {
        evt.prevent_default();
        let description = if recipient.trim().is_empty() {
            "Your message is on its way.".to_string()
        } else {
            format!("Delivered to {}.", recipient.trim())
        };
        state.discard_compose();
        toasts.info(
            "Email sent".to_string(),
            ToastOptions::new().description(description),
        );
    };

    rsx! {
        Dialog {
            open: Some(open),
            on_open_change: move |v: bool| state.set_compose_open(v),
            class: "ec-compose-dialog",
            form { class: "ec-compose-form", onsubmit: send,
                div { class: "ec-compose-head",
                    div { class: "ec-compose-head-text",
                        DialogTitle { class: "ec-compose-title", "New message" }
                        DialogDescription { class: "ec-compose-desc", "Send a message to your team or contacts." }
                    }
                    Button {
                        variant: ButtonVariant::Ghost,
                        r#type: "button",
                        class: "ec-compose-close",
                        aria_label: "Close",
                        onclick: move |_| state.discard_compose(),
                        LucideIcon { kind: IconKind::X, size: 16 }
                    }
                }

                Separator {
                    horizontal: true,
                    decorative: true,
                    class: "ec-compose-sep",
                }

                div { class: "ec-compose-field",
                    Label { html_for: "ec-compose-to", "To" }
                    Input {
                        id: "ec-compose-to",
                        r#type: "email",
                        required: true,
                        value: to.clone(),
                        placeholder: "name@company.com",
                        oninput: move |e: FormEvent| state.set_compose_to(e.value()),
                    }
                    span { class: "ec-compose-error", "Enter a valid email address." }
                }

                div { class: "ec-compose-field",
                    Label { html_for: "ec-compose-subject", "Subject" }
                    Input {
                        id: "ec-compose-subject",
                        value: subject.clone(),
                        placeholder: "What's this about?",
                        oninput: move |e: FormEvent| state.set_compose_subject(e.value()),
                    }
                }

                div { class: "ec-compose-field",
                    Label { html_for: "ec-compose-body", "Message" }
                    Textarea {
                        id: "ec-compose-body",
                        variant: TextareaVariant::Default,
                        required: true,
                        rows: "10",
                        value: body.clone(),
                        placeholder: "Write your message…",
                        oninput: move |e: FormEvent| state.set_compose_body(e.value()),
                    }
                }

                div { class: "ec-compose-foot",
                    Button {
                        variant: ButtonVariant::Ghost,
                        r#type: "button",
                        onclick: move |_| state.discard_compose(),
                        "Discard"
                    }
                    Button { variant: ButtonVariant::Primary, r#type: "submit",
                        LucideIcon { kind: IconKind::Send, size: 14 }
                        "Send"
                    }
                }
            }
        }
    }
}
