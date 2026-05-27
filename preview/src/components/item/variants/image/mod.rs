use super::super::component::*;
use dioxus::prelude::*;

const TRACKS: &[(&str, &str, &str, &str)] = &[
    (
        "Midnight City Lights",
        "Neon Dreams",
        "Electric Nights",
        "3:45",
    ),
    (
        "Coffee Shop Conversations",
        "The Morning Brew",
        "Urban Stories",
        "4:05",
    ),
    ("Digital Rain", "Cyber Symphony", "Binary Beats", "3:30"),
];

#[component]
pub fn Demo() -> Element {
    rsx! {
        div {
            display: "flex",
            flex_direction: "column",
            width: "100%",
            max_width: "28rem",

            ItemGroup { gap: "1rem",
                for (title , artist , album , duration) in TRACKS.iter() {
                    Item {
                        variant: ItemVariant::Outline,
                        as: move |attrs: Vec<Attribute>| rsx! {
                            a { href: "#", ..attrs,
                                ItemMedia { variant: ItemMediaVariant::Image,
                                    img {
                                        src: "https://avatar.vercel.sh/{title}",
                                        alt: "{title}",
                                        style: "filter: grayscale(1)",
                                    }
                                }
                                ItemContent {
                                    ItemTitle { "{title} — {album}" }
                                    ItemDescription { "{artist}" }
                                }
                                ItemContent { flex: "none",
                                    ItemDescription { "{duration}" }
                                }
                            }
                        },
                    }
                }
            }
        }
    }
}
