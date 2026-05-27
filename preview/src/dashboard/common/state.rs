use dioxus::prelude::*;

use super::{
    folder_override, FolderId, MessageTag, DEFAULT_MESSAGE_FOLDER_ID, EMAIL_REPEAT_COUNT, MESSAGES,
};

#[derive(Clone, PartialEq, Store)]
pub struct MessageState {
    pub uid: String,
    pub source_index: usize,
    pub folder_id: FolderId,
    pub tags: Vec<MessageTag>,
    pub unread: bool,
    pub starred: bool,
    pub flagged: bool,
    pub snoozed: bool,
}

pub fn seed_message_states() -> Vec<MessageState> {
    (0..EMAIL_REPEAT_COUNT)
        .flat_map(|rep| {
            MESSAGES
                .iter()
                .enumerate()
                .map(move |(idx, msg)| MessageState {
                    uid: format!("{idx}#{rep}"),
                    source_index: idx,
                    folder_id: folder_override(idx).unwrap_or(DEFAULT_MESSAGE_FOLDER_ID),
                    tags: msg.tags.to_vec(),
                    unread: msg.unread,
                    starred: msg.starred,
                    flagged: msg.starred,
                    snoozed: false,
                })
        })
        .collect()
}
