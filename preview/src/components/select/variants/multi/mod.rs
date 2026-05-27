use super::super::component::*;
use dioxus::prelude::*;
use strum::IntoEnumIterator;

#[derive(Debug, Clone, Copy, PartialEq, strum::EnumIter, strum::Display)]
enum Topping {
    Pepperoni,
    Mushroom,
    Onion,
    Olive,
    Pineapple,
}

impl Topping {
    const fn emoji(&self) -> &'static str {
        match self {
            Topping::Pepperoni => "🍕",
            Topping::Mushroom => "🍄",
            Topping::Onion => "🧅",
            Topping::Olive => "🫒",
            Topping::Pineapple => "🍍",
        }
    }
}

#[component]
pub fn Demo() -> Element {
    let toppings = Topping::iter().enumerate().map(|(i, t)| {
        rsx! {
            SelectOption::<Topping> { index: i, value: t, text_value: "{t}",
                "{t.emoji()} {t}"
            }
        }
    });

    rsx! {
        SelectMulti::<Topping> {
            default_values: vec![Topping::Pepperoni, Topping::Mushroom],
            width: "16rem",
            SelectGroup {
                SelectGroupLabel { "Toppings" }
                {toppings}
            }
        }
    }
}
