use dioxus::prelude::*;
use dioxus_icons::lucide::{
    AlarmClock, Archive as ArchiveIcon, ArrowLeft as ArrowLeftIcon, Flag as FlagIcon,
    Funnel as FilterIcon, Inbox as InboxIcon, Paperclip as PaperclipIcon, Pencil, Send as SendIcon,
    Star, Trash as TrashIcon, X as XIcon,
};

#[derive(Clone, Copy, PartialEq)]
pub enum IconKind {
    Inbox,
    Send,
    Pen,
    Archive,
    Trash,
    StarOutline,
    StarFilled,
    Paperclip,
    Filter,
    ArrowLeft,
    Flag,
    Snooze,
    X,
}

#[component]
pub fn LucideIcon(kind: IconKind, #[props(default = 16)] size: u32) -> Element {
    let size = format!("{size}px");
    match kind {
        IconKind::Inbox => rsx! {
            InboxIcon { size, stroke_width: "1.75", "aria-hidden": "true" }
        },
        IconKind::Send => rsx! {
            SendIcon { size, stroke_width: "1.75", "aria-hidden": "true" }
        },
        IconKind::Pen => rsx! {
            Pencil { size, stroke_width: "1.75", "aria-hidden": "true" }
        },
        IconKind::Archive => rsx! {
            ArchiveIcon { size, stroke_width: "1.75", "aria-hidden": "true" }
        },
        IconKind::Trash => rsx! {
            TrashIcon { size, stroke_width: "1.75", "aria-hidden": "true" }
        },
        IconKind::StarOutline => rsx! {
            Star { size, stroke_width: "1.75", "aria-hidden": "true" }
        },
        IconKind::StarFilled => rsx! {
            Star { size, fill: "currentColor", stroke_width: "1.75", "aria-hidden": "true" }
        },
        IconKind::Paperclip => rsx! {
            PaperclipIcon { size, stroke_width: "1.75", "aria-hidden": "true" }
        },
        IconKind::Filter => rsx! {
            FilterIcon { size, stroke_width: "1.75", "aria-hidden": "true" }
        },
        IconKind::ArrowLeft => rsx! {
            ArrowLeftIcon { size, stroke_width: "1.75", "aria-hidden": "true" }
        },
        IconKind::Flag => rsx! {
            FlagIcon { size, stroke_width: "1.75", "aria-hidden": "true" }
        },
        IconKind::Snooze => rsx! {
            AlarmClock { size, stroke_width: "1.75", "aria-hidden": "true" }
        },
        IconKind::X => rsx! {
            XIcon { size, stroke_width: "1.75", "aria-hidden": "true" }
        },
    }
}
