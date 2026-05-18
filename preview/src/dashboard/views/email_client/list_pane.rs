use dioxus::prelude::*;
use dioxus_icons::lucide::{Check, ChevronDown};
use dioxus_kit_core::select as primitive_select;

use crate::components::avatar::{AvatarImageSize, AvatarShape, ImageAvatar};
use crate::components::button::{Button, ButtonVariant};
use crate::components::item::{
    Item, ItemActions, ItemContent, ItemDescription, ItemMedia, ItemMediaVariant, ItemTitle,
};
use crate::components::tabs::component::{TabList, TabTrigger, Tabs};
use crate::components::virtual_list::VirtualList;
use crate::dashboard::common::{
    lookup_message, IconKind, LucideIcon, MessageState, MessageStateStoreExt, MessageTag, TabId,
    LOREM_IPSUM, TABS,
};

use super::avatars::avatar_profile_for_key;
use super::state::{EmailClientState, EmailClientStateStoreExt, EmailClientStateStoreImplExt};

#[derive(Clone, PartialEq)]
pub(super) enum ListRow {
    DayHeader(&'static str),
    Message(String),
}

pub(super) fn flatten_rows(state: Store<EmailClientState>, message_ids: &[String]) -> Vec<ListRow> {
    let messages_store = state.messages();
    let messages = messages_store.read();
    let mut out = Vec::with_capacity(message_ids.len() + 4);
    let mut last_day: Option<&'static str> = None;
    for uid in message_ids {
        let Some(message) = messages.get(uid.as_str()) else {
            continue;
        };
        let day = lookup_message(message.source_index).day;
        if last_day != Some(day) {
            out.push(ListRow::DayHeader(day));
            last_day = Some(day);
        }
        out.push(ListRow::Message(uid.clone()));
    }
    out
}

#[component]
pub(super) fn ListPane(
    mut state: Store<EmailClientState>,
    visible_ids: ReadSignal<Vec<String>>,
    selected_uid: ReadSignal<Option<String>>,
) -> Element {
    let rows = use_memo(move || flatten_rows(state, &visible_ids.read()));
    let row_count = rows.read().len();
    let query = state.search_query().cloned();
    let tags = state.selected_tags().cloned();

    rsx! {
        section { class: "ec-list-pane",
            div { class: "ec-list-toolbar",
                Tabs {
                    class: "ec-mail-tabs",
                    default_value: TabId::All.as_str().to_string(),
                    horizontal: true,
                    on_value_change: move |v: String| {
                        if let Some(tab) = TabId::from_str(&v) {
                            state.set_active_tab(tab);
                        }
                    },
                    TabList { class: "ec-mail-tabs-list",
                        for (idx, tab) in TABS.iter().enumerate() {
                            TabTrigger {
                                class: "ec-mail-tab",
                                key: "{tab.id.as_str()}",
                                value: tab.id.as_str().to_string(),
                                index: idx,
                                {tab.label}
                                span { class: "ec-muted",
                                    " {state.tab_count(tab.id, query.as_str(), &tags)}"
                                }
                            }
                        }
                    }
                }
                primitive_select::SelectMulti::<MessageTag> {
                    class: "dx-select",
                    values: Some(tags.clone()),
                    default_values: vec![],
                    on_values_change: move |values| {
                        state.set_selected_tags(values);
                    },
                    primitive_select::SelectTrigger {
                        class: "dx-select-trigger ec-filter-trigger",
                        aria_label: "Filter by tag",
                        LucideIcon { kind: IconKind::Filter }
                        if !tags.is_empty() {
                            span { class: "ec-filter-count", "{tags.len()}" }
                        }
                        ChevronDown {
                            class: "dx-select-expand-icon",
                            size: "20px",
                            stroke: "var(--primary-color-7)",
                        }
                    }
                    primitive_select::SelectList {
                        class: "dx-select-list ec-filter-list",
                        aria_label: "Filter by tag",
                        primitive_select::SelectGroup {
                            primitive_select::SelectGroupLabel { class: "dx-select-group-label", "Tags" }
                            for (index, tag) in MessageTag::ALL.iter().enumerate() {
                                primitive_select::SelectOption::<MessageTag> {
                                    class: "dx-select-option",
                                    key: "{tag.label()}",
                                    index,
                                    value: *tag,
                                    text_value: "{tag.label()}",
                                    {tag.label()}
                                    primitive_select::SelectItemIndicator {
                                        Check {
                                            size: "1rem",
                                            stroke: "var(--secondary-color-5)",
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            VirtualList {
                class: "ec-list-scroll",
                count: row_count,
                buffer: 6usize,
                estimate_size: move |idx: usize| match &rows.read()[idx] {
                    ListRow::DayHeader(_) => 34,
                    ListRow::Message(_) => 130,
                },
                render_item: move |idx: usize| {
                    let row = rows.read()[idx].clone();
                    match row {
                        ListRow::DayHeader(day) => rsx! {
                            div { class: "ec-day", {day} }
                        },
                        ListRow::Message(uid) => {
                            match state.messages().get(uid.clone()) {
                                Some(message) => rsx! {
                                    MessageRow {
                                        key: "{uid}",
                                        state,
                                        message,
                                        selected_uid: selected_uid.read().clone(),
                                    }
                                },
                                None => rsx! {},
                            }
                        }
                    }
                },
            }
        }
    }
}

#[component]
fn MessageRow(
    mut state: Store<EmailClientState>,
    message: Store<MessageState>,
    selected_uid: Option<String>,
) -> Element {
    let uid = message.uid().cloned();
    let is_selected = selected_uid.as_deref() == Some(uid.as_str());
    let uid_for_click = uid.clone();
    let uid_for_key = uid.clone();
    let uid_for_trash = uid.clone();
    let uid_for_star = uid.clone();
    let m = lookup_message(message.source_index().cloned());
    let starred = message.starred().cloned();
    let unread = message.unread().cloned();
    let flagged = message.flagged().cloned();
    let tags = message.tags().cloned();
    let mut classes = String::from("ec-row");
    if unread {
        classes.push_str(" ec-unread");
    }
    if starred {
        classes.push_str(" ec-starred");
    }
    if flagged {
        classes.push_str(" ec-flagged");
    }

    rsx! {
        Item {
            class: classes,
            role: "button",
            tabindex: 0,
            onclick: move |_| {
                state.select_message(uid_for_click.clone());
            },
            onkeydown: move |event: KeyboardEvent| {
                let key = event.key().to_string();
                if key == "Enter" || key == " " {
                    event.prevent_default();
                    state.select_message(uid_for_key.clone());
                }
            },
            "data-selected": is_selected,

            ItemMedia { variant: ItemMediaVariant::Icon,
                ImageAvatar {
                    size: AvatarImageSize::Small,
                    shape: AvatarShape::Circle,
                    src: "{avatar_profile_for_key(m.sender.addr).src}",
                    alt: "{m.sender.name}",
                    {m.sender.initials}
                }
            }
            ItemContent {
                ItemTitle {
                    span { class: "ec-row-subject", "{m.subject}" }
                }
                div { class: "ec-row-sender",
                    span { class: "ec-row-from", {m.sender.name} }
                }
                ItemDescription { class: "ec-row-snippet", {LOREM_IPSUM} }
                if !tags.is_empty() || m.has_attachment || flagged {
                    div { class: "ec-muted ec-row-tags",
                        if flagged {
                            LucideIcon { kind: IconKind::Flag, size: 12 }
                        }
                        for (i, tag) in tags.iter().enumerate() {
                            span { key: "{tag.label()}",
                                if i > 0 {
                                    " · "
                                }
                                {tag.label()}
                            }
                        }
                        if m.has_attachment {
                            LucideIcon { kind: IconKind::Paperclip, size: 12 }
                        }
                    }
                }
            }
            ItemActions {
                span { class: "ec-muted ec-row-time", "{m.time}" }
                div { class: "ec-row-action-group",
                    Button {
                        variant: ButtonVariant::Ghost,
                        r#type: "button",
                        class: "ec-row-action ec-row-action-trash",
                        aria_label: "Move to trash",
                        onkeydown: move |e: KeyboardEvent| e.stop_propagation(),
                        onclick: move |e: Event<MouseData>| {
                            e.stop_propagation();
                            state.move_message_to_trash(uid_for_trash.clone());
                        },
                        LucideIcon { kind: IconKind::Trash, size: 16 }
                    }
                    Button {
                        variant: ButtonVariant::Ghost,
                        r#type: "button",
                        class: "ec-row-action ec-row-action-star",
                        "data-active": starred,
                        aria_label: if starred { "Unstar message" } else { "Star message" },
                        onkeydown: move |e: KeyboardEvent| e.stop_propagation(),
                        onclick: move |e: Event<MouseData>| {
                            e.stop_propagation();
                            state.toggle_message_star(uid_for_star.clone());
                        },
                        LucideIcon {
                            kind: if starred { IconKind::StarFilled } else { IconKind::StarOutline },
                            size: 16,
                        }
                    }
                }
            }
        }
    }
}
