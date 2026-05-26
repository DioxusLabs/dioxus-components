use std::collections::BTreeMap;

use dioxus::prelude::*;

use crate::dashboard::common::{
    seed_message_states, FolderId, MessageState, MessageTag, TabId, FOLDERS,
};

use super::filters::{message_matches_filters, message_matches_folder};

#[derive(Clone, PartialEq, Store)]
pub(super) struct EmailClientState {
    pub messages: BTreeMap<String, MessageState>,
    pub message_order: Vec<String>,
    pub active_folder: FolderId,
    pub active_tab: TabId,
    pub search_query: String,
    pub selected_tags: Vec<MessageTag>,
    pub selected_id: String,
    pub read_open: bool,
    pub compose_open: bool,
    pub compose_to: String,
    pub compose_subject: String,
    pub compose_body: String,
}

impl EmailClientState {
    pub fn new() -> Self {
        let seeded = seed_message_states();
        let message_order = seeded.iter().map(|message| message.uid.clone()).collect();
        let messages = seeded
            .into_iter()
            .map(|message| (message.uid.clone(), message))
            .collect();

        Self {
            messages,
            message_order,
            active_folder: FolderId::Inbox,
            active_tab: TabId::All,
            search_query: String::new(),
            selected_tags: Vec::new(),
            selected_id: String::from("0#0"),
            read_open: false,
            compose_open: false,
            compose_to: String::new(),
            compose_subject: String::new(),
            compose_body: String::new(),
        }
    }
}

#[store]
impl<Lens> Store<EmailClientState, Lens> {
    pub fn active_folder_label(&self) -> &'static str {
        let active_folder = self.active_folder().cloned();
        FOLDERS
            .iter()
            .find(|folder| folder.id == active_folder)
            .map(|folder| folder.label)
            .unwrap_or("Inbox")
    }

    pub fn visible_message_ids(&self) -> Vec<String> {
        let active_folder = self.active_folder().cloned();
        let active_tab = self.active_tab().cloned();
        let search_query = self.search_query().cloned();
        let selected_tags = self.selected_tags().cloned();
        let messages_store = self.messages();
        let order_store = self.message_order();
        let messages = messages_store.read();
        let order = order_store.read();

        order
            .iter()
            .filter(|uid| {
                messages.get(uid.as_str()).is_some_and(|message| {
                    message_matches_filters(
                        message,
                        active_folder,
                        active_tab,
                        search_query.as_str(),
                        &selected_tags,
                    )
                })
            })
            .cloned()
            .collect()
    }

    pub fn selected_message_uid(&self, visible_ids: &[String]) -> Option<String> {
        let selected_id = self.selected_id().cloned();
        if visible_ids.iter().any(|uid| uid == &selected_id) {
            return Some(selected_id);
        }

        visible_ids.first().cloned()
    }

    pub fn selected_message_index(&self, selected_uid: Option<&str>, visible_ids: &[String]) -> usize {
        selected_uid
            .and_then(|selected_uid| {
                visible_ids
                    .iter()
                    .position(|uid| uid == selected_uid)
                    .map(|index| index + 1)
            })
            .unwrap_or(0)
    }

    pub fn folder_count(&self, folder_id: FolderId) -> u32 {
        let messages_store = self.messages();
        let count = messages_store
            .read()
            .values()
            .filter(|message| message_matches_folder(message, folder_id))
            .count() as u32;
        count
    }

    pub fn tab_count(&self, tab_id: TabId, visible_query: &str, selected_tags: &[MessageTag]) -> u32 {
        let active_folder = self.active_folder().cloned();
        let messages_store = self.messages();
        let count = messages_store
            .read()
            .values()
            .filter(|message| {
                message_matches_filters(
                    message,
                    active_folder,
                    tab_id,
                    visible_query,
                    selected_tags,
                )
            })
            .count() as u32;
        count
    }

    pub fn set_active_folder(&mut self, folder_id: FolderId) {
        self.active_folder().set(folder_id);
        self.close_read_pane();
    }

    pub fn set_active_tab(&mut self, tab_id: TabId) {
        self.active_tab().set(tab_id);
        self.close_read_pane();
    }

    pub fn set_search_query(&mut self, query: String) {
        self.search_query().set(query);
        self.close_read_pane();
    }

    pub fn set_selected_tags(&mut self, tags: Vec<MessageTag>) {
        self.selected_tags().set(tags);
        self.close_read_pane();
    }

    pub fn select_message(&mut self, uid: String) {
        self.selected_id().set(uid);
        self.read_open().set(true);
    }

    pub fn close_read_pane(&mut self) {
        self.read_open().set(false);
    }

    pub fn open_compose(&mut self) {
        self.compose_open().set(true);
    }

    pub fn set_compose_open(&mut self, open: bool) {
        self.compose_open().set(open);
    }

    pub fn discard_compose(&mut self) {
        self.compose_open().set(false);
        self.compose_to().set(String::new());
        self.compose_subject().set(String::new());
        self.compose_body().set(String::new());
    }

    pub fn set_compose_to(&mut self, value: String) {
        self.compose_to().set(value);
    }

    pub fn set_compose_subject(&mut self, value: String) {
        self.compose_subject().set(value);
    }

    pub fn set_compose_body(&mut self, value: String) {
        self.compose_body().set(value);
    }

    pub fn archive_message(&mut self, uid: String) {
        self.update_message(uid.clone(), |message| {
            message.folder_id = FolderId::Archive;
            message.unread = false;
        });
        self.close_if_selected(&uid);
    }

    pub fn snooze_message(&mut self, uid: String) {
        self.update_message(uid.clone(), |message| {
            message.snoozed = true;
        });
        self.close_if_selected(&uid);
    }

    pub fn delete_message(&mut self, uid: String) {
        self.update_message(uid.clone(), |message| {
            message.folder_id = FolderId::Trash;
            message.unread = false;
        });
        self.close_if_selected(&uid);
    }

    pub fn toggle_message_flag(&mut self, uid: String) {
        self.update_message(uid, |message| {
            message.flagged = !message.flagged;
        });
    }

    pub fn toggle_message_star(&mut self, uid: String) {
        self.update_message(uid, |message| {
            message.starred = !message.starred;
        });
    }

    pub fn move_message_to_trash(&mut self, uid: String) {
        self.update_message(uid.clone(), |message| {
            message.folder_id = FolderId::Trash;
        });
        self.close_if_selected(&uid);
    }

    pub fn remove_message_tag(&mut self, uid: String, tag: MessageTag) {
        self.update_message(uid, |message| {
            message.tags.retain(|current| *current != tag);
        });
    }

    pub fn set_message_tags(&mut self, uid: String, tags: Vec<MessageTag>) {
        self.update_message(uid, |message| {
            message.tags = tags;
        });
    }

    pub fn update_message(&mut self, uid: String, update: impl FnOnce(&mut MessageState)) {
        if let Some(mut message) = self.messages().get(uid) {
            let mut message = message.write();
            update(&mut message);
        }
    }

    pub fn close_if_selected(&mut self, uid: &str) {
        if self.selected_id().cloned() == uid {
            self.close_read_pane();
        }
    }
}
