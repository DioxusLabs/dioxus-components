use super::super::component::*;
use dioxus::prelude::*;
use strum::{EnumCount, IntoEnumIterator};

#[derive(Debug, Clone, Copy, PartialEq, strum::EnumCount, strum::EnumIter, strum::Display)]
enum Fruit {
    Apple,
    Banana,
    Orange,
    Orangeade,
    Strawberry,
    Watermelon,
}

impl Fruit {
    const fn emoji(&self) -> &'static str {
        match self {
            Fruit::Apple => "🍎",
            Fruit::Banana => "🍌",
            Fruit::Orange => "🍊",
            Fruit::Orangeade => "🧃",
            Fruit::Strawberry => "🍓",
            Fruit::Watermelon => "🍉",
        }
    }
}

#[component]
pub fn Demo() -> Element {
    let fruits = Fruit::iter().enumerate().map(|(i, f)| {
        rsx! {
            SelectOption::<Option<Fruit>> { index: i, value: f, text_value: "{f}",
                disabled: matches!(f, Fruit::Orange),
                "{f.emoji()} {f}"
            }
        }
    });

    rsx! {
        Select::<Option<Fruit>> {
            width: "12rem",
            SelectGroup {
                SelectGroupLabel { "Fruits" }
                {fruits}
            }
            SelectGroup {
                SelectGroupLabel { "Other" }
                SelectOption::<Option<Fruit>> {
                    index: Fruit::COUNT,
                    value: None,
                    text_value: "Other",
                    "Other"
                }
            }
        }
    }
}
