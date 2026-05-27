use dioxus::prelude::*;

use super::super::component::*;
use dioxus_primitives::color_picker::Color;
use palette::{encoding, Hsv, IntoColor};

#[component]
pub fn Demo() -> Element {
    let mut color = use_signal(|| -> Hsv<encoding::Srgb, f64> {
        Color::new(155, 128, 255).into_format::<f64>().into_color()
    });

    rsx! {
        ColorPicker {
            label: "Pick a color",
            color: color(),
            on_color_change: move |c| {
                tracing::info!("Color changed: {:?}", c);
                color.set(c);
            },
        }
    }
}
