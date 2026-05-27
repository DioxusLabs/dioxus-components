pub mod icons;
pub mod messages;
pub mod state;

pub use icons::*;
pub use messages::*;
pub use state::*;

#[derive(Clone, Copy, PartialEq)]
pub struct AvatarProfile {
    pub name: &'static str,
    pub initials: &'static str,
    pub src: &'static str,
}

pub const AVATAR_PROFILE_OPTIONS: &[AvatarProfile] = &[
    AvatarProfile {
        name: "ealmloff",
        initials: "EA",
        src: "https://github.com/ealmloff.png",
    },
    AvatarProfile {
        name: "nicoburns",
        initials: "NB",
        src: "https://github.com/nicoburns.png",
    },
    AvatarProfile {
        name: "jkelleyrtp",
        initials: "JK",
        src: "https://github.com/jkelleyrtp.png",
    },
    AvatarProfile {
        name: "DioxusLabs",
        initials: "DX",
        src: "https://github.com/DioxusLabs.png",
    },
];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FolderId {
    Inbox,
    Starred,
    Sent,
    Drafts,
    Archive,
    Trash,
}

impl FolderId {
    pub const fn as_str(self) -> &'static str {
        match self {
            FolderId::Inbox => "inbox",
            FolderId::Starred => "starred",
            FolderId::Sent => "sent",
            FolderId::Drafts => "drafts",
            FolderId::Archive => "archive",
            FolderId::Trash => "trash",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TabId {
    All,
    Unread,
    Flagged,
}

impl TabId {
    pub const fn as_str(self) -> &'static str {
        match self {
            TabId::All => "all",
            TabId::Unread => "unread",
            TabId::Flagged => "flagged",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "all" => Some(TabId::All),
            "unread" => Some(TabId::Unread),
            "flagged" => Some(TabId::Flagged),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Folder {
    pub id: FolderId,
    pub label: &'static str,
    pub icon: IconKind,
    pub count: Option<u32>,
}

#[derive(Clone, Copy, PartialEq)]
pub struct Tab {
    pub id: TabId,
    pub label: &'static str,
    pub count: u32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MessageTag {
    Alerts,
    Calendar,
    Deploys,
    Design,
    Github,
    Newsletter,
    Receipts,
    Sales,
    Security,
    Urgent,
    Work,
}

impl MessageTag {
    pub const ALL: &[MessageTag] = &[
        MessageTag::Alerts,
        MessageTag::Calendar,
        MessageTag::Deploys,
        MessageTag::Design,
        MessageTag::Github,
        MessageTag::Newsletter,
        MessageTag::Receipts,
        MessageTag::Sales,
        MessageTag::Security,
        MessageTag::Urgent,
        MessageTag::Work,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            MessageTag::Alerts => "alerts",
            MessageTag::Calendar => "calendar",
            MessageTag::Deploys => "deploys",
            MessageTag::Design => "design",
            MessageTag::Github => "github",
            MessageTag::Newsletter => "newsletter",
            MessageTag::Receipts => "receipts",
            MessageTag::Sales => "sales",
            MessageTag::Security => "security",
            MessageTag::Urgent => "urgent",
            MessageTag::Work => "work",
        }
    }
}

pub const FOLDERS: &[Folder] = &[
    Folder {
        id: FolderId::Inbox,
        label: "Inbox",
        icon: IconKind::Inbox,
        count: Some(12),
    },
    Folder {
        id: FolderId::Starred,
        label: "Starred",
        icon: IconKind::StarOutline,
        count: None,
    },
    Folder {
        id: FolderId::Sent,
        label: "Sent",
        icon: IconKind::Send,
        count: None,
    },
    Folder {
        id: FolderId::Drafts,
        label: "Drafts",
        icon: IconKind::Pen,
        count: Some(3),
    },
    Folder {
        id: FolderId::Archive,
        label: "Archive",
        icon: IconKind::Archive,
        count: None,
    },
    Folder {
        id: FolderId::Trash,
        label: "Trash",
        icon: IconKind::Trash,
        count: None,
    },
];

pub const TABS: &[Tab] = &[
    Tab {
        id: TabId::All,
        label: "All",
        count: 42,
    },
    Tab {
        id: TabId::Unread,
        label: "Unread",
        count: 12,
    },
    Tab {
        id: TabId::Flagged,
        label: "Flagged",
        count: 5,
    },
];
