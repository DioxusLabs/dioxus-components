use std::sync::LazyLock;

use super::{FolderId, MessageTag};

#[derive(Clone, Copy, PartialEq)]
pub struct Sender {
    pub name: &'static str,
    pub addr: &'static str,
    pub initials: &'static str,
}

const fn sender(name: &'static str, addr: &'static str, initials: &'static str) -> Sender {
    Sender {
        name,
        addr,
        initials,
    }
}

pub const LINEAR: Sender = sender("Linear", "notifications@linear.app", "LN");
pub const GITHUB: Sender = sender("GitHub", "noreply@github.com", "GH");
pub const MAYA: Sender = sender("Maya Chen", "maya@figma-internal.com", "MC");
pub const STRIPE: Sender = sender("Stripe", "receipts@stripe.com", "ST");
pub const CALENDAR: Sender = sender("Calendar", "calendar@workspace.app", "CA");
pub const YOU: Sender = sender("You", "you@yourcompany.com", "Y");
pub const VERCEL: Sender = sender("Vercel", "deploys@vercel.com", "VC");
pub const MARTA: Sender = sender("Marta Liu", "marta.l@yourcompany.com", "ML");
pub const ONEPASSWORD: Sender = sender("1Password", "security@1password.com", "1P");
pub const AWS_BILLING: Sender = sender("AWS Billing", "no-reply@aws.amazon.com", "AW");
pub const NOTION: Sender = sender("Notion", "team@notion.so", "NO");
pub const SENTRY: Sender = sender("Sentry", "alerts@sentry.io", "SE");
pub const CAROLINE: Sender = sender("Caroline Wu", "caroline@yourcompany.com", "CW");
pub const DATADOG: Sender = sender("Datadog", "alerts@datadog.com", "DD");
pub const SLACK: Sender = sender("Slack", "feedback@slack.com", "SL");
pub const FIGMA: Sender = sender("Figma", "no-reply@figma.com", "FG");
pub const PEDRO: Sender = sender("Pedro Carvalho", "pedro@yourcompany.com", "PC");
pub const LINKEDIN: Sender = sender("LinkedIn", "messages-noreply@linkedin.com", "LI");
pub const SUBSTACK: Sender = sender("Substack", "no-reply@substack.com", "SU");
pub const ELI: Sender = sender("Eli Rosen", "eli.r@yourcompany.com", "ER");
pub const AWS_COST: Sender = sender("AWS Cost Explorer", "no-reply@aws.amazon.com", "AW");
pub const SAMIR: Sender = sender("Samir Kapoor", "samir@yourcompany.com", "SK");
pub const APPLE: Sender = sender("Apple", "no-reply@email.apple.com", "AP");
pub const HEX: Sender = sender("Hex", "no-reply@hex.tech", "HX");
pub const CRUNCHBASE: Sender = sender("Crunchbase Daily", "newsletter@crunchbase.com", "CB");
pub const HACKER_NEWS: Sender = sender("Hacker News Daily", "digest@hnrss.com", "HN");
pub const CLOUDFLARE: Sender = sender("Cloudflare", "noreply@cloudflare.com", "CF");

#[derive(Clone, PartialEq)]
pub struct Message {
    pub day: &'static str,
    pub sender: Sender,
    pub subject: String,
    pub time: String,
    pub full_time: String,
    pub unread: bool,
    pub starred: bool,
    pub thread_count: u32,
    pub tags: &'static [MessageTag],
    pub has_attachment: bool,
}

pub const EMAIL_REPEAT_COUNT: usize = 5;
pub const MESSAGE_COUNT: usize = 60;

pub const LOREM_IPSUM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.";

pub const DEFAULT_MESSAGE_FOLDER_ID: FolderId = FolderId::Inbox;

const SENDERS_CYCLE: &[Sender] = &[
    LINEAR,
    GITHUB,
    MAYA,
    STRIPE,
    CALENDAR,
    YOU,
    VERCEL,
    MARTA,
    ONEPASSWORD,
    AWS_BILLING,
    NOTION,
    SENTRY,
    CAROLINE,
    DATADOG,
    SLACK,
    FIGMA,
    PEDRO,
    LINKEDIN,
    SUBSTACK,
    ELI,
    AWS_COST,
    SAMIR,
    APPLE,
    HEX,
    CRUNCHBASE,
    HACKER_NEWS,
    CLOUDFLARE,
];

const SUBJECT_TEMPLATES: &[&str] = &[
    "ENG-{n} · Reading-pane scroll restoration",
    "[claude-cookbook] PR #{n} ready for review",
    "Re: design pass — first review",
    "Your invoice is ready",
    "Daily agenda · this week",
    "Q2 roadmap — final draft for your eyes",
    "Production deploy succeeded · build {n}",
    "Customer call notes (Acme)",
    "New sign-in detected · MacBook Pro",
    "AWS bill is now available",
    "Re: Cycle planning · capacity",
    "[Triggered] high p95 latency · api.search",
    "Mossberg lead · they want a demo",
    "Postmortem draft · search outage",
    "Out of office reminder",
    "Subscription renewed",
];

const TAG_BUCKETS: &[&[MessageTag]] = &[
    &[MessageTag::Work, MessageTag::Urgent, MessageTag::Calendar],
    &[MessageTag::Github, MessageTag::Work],
    &[MessageTag::Design, MessageTag::Work],
    &[MessageTag::Receipts, MessageTag::Work],
    &[MessageTag::Calendar, MessageTag::Work],
    &[MessageTag::Sales, MessageTag::Work, MessageTag::Urgent],
    &[MessageTag::Security, MessageTag::Alerts],
    &[MessageTag::Newsletter, MessageTag::Sales],
    &[MessageTag::Deploys, MessageTag::Work, MessageTag::Alerts],
];

const DAY_BUCKETS: &[(&str, usize)] = &[
    ("Today", 5),
    ("Yesterday", 3),
    ("This week", 11),
    ("Last week", 10),
    ("Earlier in April", 13),
    ("March", 15),
    ("February", 3),
];

const DAY_NAMES: &[&str] = &["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

fn day_for(i: usize) -> &'static str {
    let mut acc = 0;
    for (label, count) in DAY_BUCKETS {
        acc += count;
        if i < acc {
            return label;
        }
    }
    DAY_BUCKETS.last().unwrap().0
}

fn clock(i: usize) -> (u32, u32) {
    let hour = 7 + ((i * 3) % 12) as u32;
    let minute = ((i * 17) % 60) as u32;
    (hour, minute)
}

fn time_for(i: usize) -> String {
    let (hour, minute) = clock(i);
    if i < 5 {
        format!("{hour:02}:{minute:02}")
    } else if i < 8 {
        "Yesterday".to_string()
    } else if i < 29 {
        DAY_NAMES[(i - 8) % DAY_NAMES.len()].to_string()
    } else if i < 42 {
        format!("Apr {}", 12 - (i - 29).min(11))
    } else if i < 57 {
        format!("Mar {}", 30 - (i - 42).min(28))
    } else {
        format!("Feb {}", 26 - (i - 57))
    }
}

fn full_time_for(i: usize) -> String {
    let (hour, minute) = clock(i);
    let (display_hour, suffix) = match hour {
        0 => (12, "AM"),
        1..=11 => (hour, "AM"),
        12 => (12, "PM"),
        _ => (hour - 12, "PM"),
    };
    let (month, day_num) = if i < 29 {
        ("Apr", 28 - (i / 2).min(27))
    } else if i < 42 {
        ("Apr", 12 - (i - 29).min(11))
    } else if i < 57 {
        ("Mar", 30 - (i - 42).min(28))
    } else {
        ("Feb", 26 - (i - 57))
    };
    format!("{month} {day_num}, 2026 · {display_hour}:{minute:02} {suffix}")
}

pub fn folder_override(i: usize) -> Option<FolderId> {
    match i % 13 {
        2 => Some(FolderId::Drafts),
        5 => Some(FolderId::Sent),
        8 => Some(FolderId::Archive),
        11 => Some(FolderId::Trash),
        _ => None,
    }
}

pub static MESSAGES: LazyLock<Vec<Message>> = LazyLock::new(|| {
    (0..MESSAGE_COUNT)
        .map(|i| Message {
            day: day_for(i),
            sender: SENDERS_CYCLE[i % SENDERS_CYCLE.len()],
            subject: SUBJECT_TEMPLATES[i % SUBJECT_TEMPLATES.len()]
                .replace("{n}", &(1000 + i).to_string()),
            time: time_for(i),
            full_time: full_time_for(i),
            unread: i < 3,
            starred: i % 7 == 0,
            thread_count: 1 + (i as u32 % 4),
            tags: TAG_BUCKETS[i % TAG_BUCKETS.len()],
            has_attachment: i % 5 == 0,
        })
        .collect()
});

pub fn lookup_message(source_index: usize) -> &'static Message {
    MESSAGES.get(source_index).unwrap_or(&MESSAGES[0])
}
