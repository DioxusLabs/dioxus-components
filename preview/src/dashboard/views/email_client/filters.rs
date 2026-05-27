use crate::dashboard::common::{lookup_message, FolderId, MessageState, MessageTag, TabId};

pub(super) fn message_matches_folder(state: &MessageState, folder_id: FolderId) -> bool {
    if state.snoozed {
        return false;
    }
    match folder_id {
        FolderId::Starred => state.starred,
        id => state.folder_id == id,
    }
}

pub(super) fn message_matches_tab(state: &MessageState, tab_id: TabId) -> bool {
    match tab_id {
        TabId::Unread => state.unread,
        TabId::Flagged => state.flagged,
        TabId::All => true,
    }
}

pub(super) fn message_matches_search(state: &MessageState, query: &str) -> bool {
    let query = query.trim().to_lowercase();
    if query.is_empty() {
        return true;
    }
    let m = lookup_message(state.source_index);
    m.sender.name.to_lowercase().contains(&query)
        || m.sender.addr.to_lowercase().contains(&query)
        || m.subject.to_lowercase().contains(&query)
        || state.tags.iter().any(|tag| tag.label().contains(&query))
        || (m.has_attachment && "attachment".contains(&query))
}

pub(super) fn message_matches_selected_tags(
    state: &MessageState,
    selected_tags: &[MessageTag],
) -> bool {
    selected_tags
        .iter()
        .all(|s| state.tags.iter().any(|tag| tag == s))
}

pub(super) fn message_matches_filters(
    state: &MessageState,
    folder_id: FolderId,
    tab_id: TabId,
    query: &str,
    selected_tags: &[MessageTag],
) -> bool {
    message_matches_folder(state, folder_id)
        && message_matches_tab(state, tab_id)
        && message_matches_search(state, query)
        && message_matches_selected_tags(state, selected_tags)
}
