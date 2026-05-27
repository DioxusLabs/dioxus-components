use dioxus::prelude::*;

use crate::components::input::Input;
use crate::components::separator::Separator;
use crate::components::sidebar::{SidebarInset, SidebarProvider, SidebarTrigger};
use crate::components::toast::ToastProvider;
use crate::theme::DarkModeToggle;

mod avatars;
mod compose;
mod filters;
mod list_pane;
mod read_pane;
mod sidebar;
mod state;

use compose::ComposeModal;
use list_pane::ListPane;
use read_pane::ReadPane;
use sidebar::EmailSidebar;
use state::{EmailClientState, EmailClientStateStoreExt, EmailClientStateStoreImplExt};

#[component]
pub fn EmailClient() -> Element {
    let mut state = use_store(EmailClientState::new);

    let visible_ids = use_memo(move || state.visible_message_ids());
    let selected_uid = use_memo(move || state.selected_message_uid(&visible_ids.read()));
    let total_count = use_memo(move || visible_ids.read().len());
    let selected_index = use_memo(move || {
        state.selected_message_index(selected_uid.read().as_deref(), &visible_ids.read())
    });

    let folder_label = state.active_folder_label();
    let read_open = state.read_open().cloned() && selected_uid.read().is_some();

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./email_client.css") }

        ToastProvider {
            SidebarProvider {
                EmailSidebar { state }

                SidebarInset {
                    header { class: "ec-topbar",
                        SidebarTrigger {}
                        Separator { horizontal: false, decorative: true }
                        h1 { class: "ec-title", {folder_label} }
                        Input {
                            r#type: "search",
                            "aria-label": "Search mail",
                            name: "mail-search",
                            value: state.search_query(),
                            oninput: move |event: FormEvent| {
                                state.set_search_query(event.value());
                            },
                            placeholder: "Search mail, people, attachments…",
                        }
                        DarkModeToggle {}
                    }

                    main { class: if read_open { "ec-main ec-reading" } else { "ec-main" },
                        ListPane { state, visible_ids, selected_uid }

                        ReadPane {
                            state,
                            selected_uid,
                            total_count,
                            selected_index,
                        }
                    }

                    ComposeModal { state }
                }
            }
        }
    }
}
