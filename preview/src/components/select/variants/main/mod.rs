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

#[derive(Debug, Clone, Copy, PartialEq, strum::EnumCount, strum::EnumIter, strum::Display)]
enum Vegetable {
    Broccoli,
    Carrot,
    Corn,
    Cucumber,
    Potato,
}

impl Vegetable {
    const fn emoji(&self) -> &'static str {
        match self {
            Vegetable::Broccoli => "🥦",
            Vegetable::Carrot => "🥕",
            Vegetable::Corn => "🌽",
            Vegetable::Cucumber => "🥒",
            Vegetable::Potato => "🥔",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Food {
    Fruit(Fruit),
    Vegetable(Vegetable),
}

#[component]
pub fn Demo() -> Element {
    let fruits = Fruit::iter().enumerate().map(|(i, f)| {
        rsx! {
            SelectOption::<Option<Food>> { index: i, value: Food::Fruit(f), text_value: "{f}",
                disabled: matches!(f, Fruit::Orange),
                "{f.emoji()} {f}"
            }
        }
    });

    let vegetables = Vegetable::iter().enumerate().map(|(i, v)| {
        rsx! {
            SelectOption::<Option<Food>> { index: Fruit::COUNT + i, value: Food::Vegetable(v), text_value: "{v}",
                "{v.emoji()} {v}"
            }
        }
    });

    rsx! {
        Select::<Option<Food>> {
            width: "12rem",
            SelectGroup {
                SelectGroupLabel { "Fruits" }
                {fruits}
            }
            SelectGroup {
                disabled: true,
                SelectGroupLabel { "Vegetables" }
                {vegetables}
            }
            SelectGroup {
                SelectGroupLabel { "Other" }
                SelectOption::<Option<Food>> {
                    index: Fruit::COUNT + Vegetable::COUNT,
                    value: None,
                    text_value: "Other",
                    "Other"
                }
            }
        }
    }
}
