use dioxus::prelude::*;
use dioxus_icons::lucide::{Check, ChevronDown};
use dioxus_primitives::select as primitive_select;
use dioxus_primitives::toast::{use_toast, ToastOptions};
use std::rc::Rc;

use crate::components::avatar::{AvatarImageSize, AvatarShape, ImageAvatar};
use crate::components::badge::{Badge, BadgeVariant};
use crate::components::button::{Button, ButtonVariant};
use crate::components::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};
use crate::components::textarea::Textarea;
use crate::components::toolbar::component::{
    Toolbar, ToolbarButton, ToolbarGroup, ToolbarSeparator,
};
use crate::dashboard::common::{
    lookup_message, IconKind, LucideIcon, MessageState, MessageStateStoreExt, MessageTag,
    AVATAR_PROFILE_OPTIONS, LOREM_IPSUM,
};

use super::avatars::avatar_profile_for_key;
use super::state::{EmailClientState, EmailClientStateStoreExt, EmailClientStateStoreImplExt};

#[css_module("/src/components/select/style.css")]
struct SelectStyles;

#[component]
pub(super) fn ReadPane(
    mut state: Store<EmailClientState>,
    selected_uid: ReadSignal<Option<String>>,
    total_count: ReadSignal<usize>,
    selected_index: ReadSignal<usize>,
) -> Element {
    let toasts = use_toast();
    let mut reply_draft = use_signal(String::new);
    let mut reply_box_ref: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    let Some(selected_uid_value) = selected_uid.read().clone() else {
        return rsx! {};
    };
    let Some(selected) = state.messages().get(selected_uid_value.clone()) else {
        return rsx! {};
    };
    let selected: Store<MessageState> = selected.into();
    let selected_static = lookup_message(selected.source_index().cloned());
    let selected_tags = selected.tags().cloned();
    let selected_starred = selected.starred().cloned();
    let selected_flagged = selected.flagged().cloned();
    let counter = format!("{} of {}", selected_index.read(), total_count.read());

    use_effect(move || {
        let is_open = state.read_open().cloned();
        let selected_uid = selected_uid.read().clone();
        if !is_open || selected_uid.is_none() {
            return;
        }

        if let Some(reply_box) = reply_box_ref() {
            spawn(async move {
                let _ = reply_box.set_focus(true).await;
            });
        }
    });

    let archive_uid = selected_uid_value.clone();
    let archive_selected = move |_| {
        state.archive_message(archive_uid.clone());
    };
    let snooze_uid = selected_uid_value.clone();
    let snooze_selected = move |_| {
        state.snooze_message(snooze_uid.clone());
    };
    let delete_uid = selected_uid_value.clone();
    let delete_selected = move |_| {
        state.delete_message(delete_uid.clone());
    };
    let flag_uid = selected_uid_value.clone();
    let toggle_flag_selected = move |_| {
        state.toggle_message_flag(flag_uid.clone());
    };
    let star_uid = selected_uid_value.clone();
    let toggle_star_selected = move |_| {
        state.toggle_message_star(star_uid.clone());
    };
    let tag_edit_uid = selected_uid_value.clone();
    let reply_recipient = selected_static.sender.name.to_string();
    let send_reply = move |_| {
        if reply_draft.read().trim().is_empty() {
            return;
        }

        reply_draft.set(String::new());
        state.close_read_pane();
        toasts.info(
            "Reply sent".to_string(),
            ToastOptions::new().description(format!("Added to {reply_recipient}.")),
        );
    };

    rsx! {
        section { class: "ec-read-pane",
            Toolbar { class: "ec-read-toolbar", aria_label: "Message actions",
                ToolbarGroup {
                    ToolbarButton {
                        index: 0usize,
                        on_click: move |_| state.close_read_pane(),
                        LucideIcon { kind: IconKind::ArrowLeft }
                    }
                }
                ToolbarSeparator { class: "ec-read-toolbar-separator" }
                ToolbarGroup {
                    ToolbarButton { index: 1usize, on_click: archive_selected,
                        LucideIcon { kind: IconKind::Archive }
                        " Archive"
                    }
                    ToolbarButton { index: 2usize, on_click: snooze_selected,
                        LucideIcon { kind: IconKind::Snooze }
                        " Snooze"
                    }
                    ToolbarButton { index: 3usize, on_click: delete_selected,
                        LucideIcon { kind: IconKind::Trash }
                        " Delete"
                    }
                }
                ToolbarSeparator { class: "ec-read-toolbar-separator" }
                ToolbarGroup {
                    ToolbarButton { index: 4usize, on_click: toggle_flag_selected,
                        if selected_flagged {
                            LucideIcon { kind: IconKind::Flag }
                            " Flagged"
                        } else {
                            LucideIcon { kind: IconKind::Flag }
                            " Flag"
                        }
                    }
                    ToolbarButton { index: 5usize, on_click: toggle_star_selected,
                        if selected_starred {
                            LucideIcon { kind: IconKind::StarFilled }
                            " Starred"
                        } else {
                            LucideIcon { kind: IconKind::StarOutline }
                            " Star"
                        }
                    }
                }
                div { class: "ec-toolbar-end",
                    span { class: "ec-muted", {counter} }
                }
            }

            article { class: "ec-read-body ec-thread",
                Card { class: "ec-thread-hero",
                    CardHeader {
                        div { class: "ec-thread-hero-main",
                            div {
                                CardTitle { "{selected_static.subject}" }
                                CardDescription {
                                    div { class: "ec-thread-hero-meta",
                                        span {
                                            "{selected_static.thread_count} message{(selected_static.thread_count > 1).then(|| \"s\").unwrap_or(\"\")} in this thread"
                                        }
                                        primitive_select::SelectMulti::<MessageTag> {
                                            class: SelectStyles::dx_select,
                                            values: Some(selected_tags.clone()),
                                            default_values: selected_tags.clone(),
                                            on_values_change: move |values: Vec<MessageTag>| {
                                                state.set_message_tags(tag_edit_uid.clone(), values);
                                            },
                                            primitive_select::SelectTrigger {
                                                class: format!("{} ec-tag-edit-trigger", SelectStyles::dx_select_trigger),
                                                aria_label: "Add tag",
                                                "+ Tag"
                                                ChevronDown {
                                                    class: "dx-select-expand-icon",
                                                    size: "20px",
                                                    stroke: "var(--primary-color-7)",
                                                }
                                            }
                                            primitive_select::SelectList {
                                                class: format!("{} ec-filter-list", SelectStyles::dx_select_list),
                                                aria_label: "Edit tags",
                                                primitive_select::SelectGroup {
                                                    primitive_select::SelectGroupLabel { class: SelectStyles::dx_select_group_label, "Tags" }
                                                    for (index, tag) in MessageTag::ALL.iter().enumerate() {
                                                        primitive_select::SelectOption::<MessageTag> {
                                                            class: SelectStyles::dx_select_option,
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
                                        for tag in selected_tags.iter() {
                                            Button {
                                                variant: ButtonVariant::Ghost,
                                                key: "{tag.label()}",
                                                r#type: "button",
                                                class: "ec-tag-remove",
                                                "aria-label": "Remove tag {tag.label()}",
                                                onclick: {
                                                    let tag = *tag;
                                                    let uid = selected_uid_value.clone();
                                                    move |_| {
                                                        state.remove_message_tag(uid.clone(), tag);
                                                    }
                                                },
                                                Badge { variant: BadgeVariant::Secondary,
                                                    "{tag.label()} ×"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                Card { class: if selected_static.thread_count > 1 { "ec-thread-msg" } else { "ec-thread-msg ec-thread-msg-current" },
                    CardContent { class: "ec-thread-msg-content",
                        div { class: "ec-thread-msg-head",
                            ImageAvatar {
                                size: AvatarImageSize::Small,
                                shape: AvatarShape::Circle,
                                src: "{avatar_profile_for_key(selected_static.sender.addr).src}",
                                alt: "{selected_static.sender.name}",
                                {selected_static.sender.initials}
                            }
                            div { class: "ec-thread-msg-meta",
                                div { class: "ec-thread-msg-sender",
                                    span { class: "ec-thread-msg-name", {selected_static.sender.name} }
                                    span { class: "ec-thread-msg-addr", {selected_static.sender.addr} }
                                }
                                span { class: "ec-thread-msg-time", "{selected_static.full_time}" }
                            }
                        }
                        div { class: "ec-thread-msg-body",
                            p { {LOREM_IPSUM} }
                        }
                    }
                }

                if selected_static.thread_count > 1 {
                    Card { class: "ec-thread-msg ec-thread-msg-current",
                        CardContent { class: "ec-thread-msg-content",
                            div { class: "ec-thread-msg-head",
                                ImageAvatar {
                                    size: AvatarImageSize::Small,
                                    shape: AvatarShape::Circle,
                                    src: "{AVATAR_PROFILE_OPTIONS[0].src}",
                                    alt: "You",
                                    "Y"
                                }
                                div { class: "ec-thread-msg-meta",
                                    div { class: "ec-thread-msg-sender",
                                        span { class: "ec-thread-msg-name", "You" }
                                        span { class: "ec-thread-msg-addr",
                                            "to {selected_static.sender.name}"
                                        }
                                    }
                                    span { class: "ec-thread-msg-time", "earlier today" }
                                }
                            }
                            div { class: "ec-thread-msg-body",
                                p { {LOREM_IPSUM} }
                            }
                        }
                    }
                }

                Card { class: "ec-thread-compose",
                    CardContent { class: "ec-thread-compose-content",
                        div { class: "ec-thread-compose-row",
                            ImageAvatar {
                                size: AvatarImageSize::Small,
                                shape: AvatarShape::Circle,
                                src: "{AVATAR_PROFILE_OPTIONS[0].src}",
                                alt: "You",
                                "Y"
                            }
                            Textarea {
                                key: "{selected_uid_value}-reply",
                                placeholder: format!("Reply to {}…", selected_static.sender.name),
                                rows: "2",
                                value: "{reply_draft}",
                                onmounted: move |event: MountedEvent| reply_box_ref.set(Some(event.data())),
                                oninput: move |event: FormEvent| reply_draft.set(event.value()),
                            }
                            div { class: "ec-thread-compose-actions",
                                Button {
                                    variant: ButtonVariant::Primary,
                                    r#type: "button",
                                    disabled: reply_draft.read().trim().is_empty(),
                                    onclick: send_reply,
                                    LucideIcon { kind: IconKind::Send, size: 14 }
                                    "Send"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
