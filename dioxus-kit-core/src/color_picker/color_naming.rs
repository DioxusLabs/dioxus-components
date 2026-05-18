//! Human-readable color naming for [`Color`].
//!
//! Converts an sRGB color into Oklch and classifies it into a descriptive
//! label like "very dark grayish blue" or "vibrant orange".

use super::Color;
use palette::{IntoColor, Oklch};

/// Lightness threshold between orange and brown.
const ORANGE_LIGHTNESS_THRESHOLD: f64 = 0.68;

/// Lightness threshold between pure yellow and "yellow green".
const YELLOW_GREEN_LIGHTNESS_THRESHOLD: f64 = 0.85;

/// The maximum lightness considered to be "dark".
const MAX_DARK_LIGHTNESS: f64 = 0.55;

/// The chroma threshold between gray and color.
const GRAY_THRESHOLD: f64 = 0.001;

/// Build a descriptive name for `color` (e.g. `"vibrant red"`, `"very dark grayish blue"`).
pub(super) fn color_name(color: Color) -> String {
    let (l, c, h) = to_oklch(color);

    match l {
        ..0.001 => return String::from("black"),
        0.999.. => return String::from("white"),
        _ => {}
    }

    let (hue, l) = oklch_hue(l, c, h);

    let (lightness, chroma) = color_modifiers(l, c);

    let mut parts = Vec::new();
    if !lightness.is_empty() {
        parts.push(lightness);
    }
    if !chroma.is_empty() {
        parts.push(chroma);
    }
    if !hue.is_empty() {
        parts.push(&hue);
    }

    parts.join(" ")
}

fn color_modifiers(lightness: f64, chroma: f64) -> (&'static str, &'static str) {
    match (lightness, chroma) {
        (..0.3, GRAY_THRESHOLD..=0.1) => ("very dark", "grayish"),
        (..0.3, 0.15..) => ("very dark", "vibrant"),
        (..0.3, _) => ("very dark", ""),

        (0.3..MAX_DARK_LIGHTNESS, GRAY_THRESHOLD..=0.1) => ("dark", "grayish"),
        (0.3..MAX_DARK_LIGHTNESS, 0.15..) => ("dark", "vibrant"),
        (0.3..MAX_DARK_LIGHTNESS, _) => ("dark", ""),

        (MAX_DARK_LIGHTNESS..0.7, GRAY_THRESHOLD..=0.1) => ("", "grayish"),
        (MAX_DARK_LIGHTNESS..0.7, 0.15..) => ("", "vibrant"),
        (MAX_DARK_LIGHTNESS..0.7, _) => ("", ""),

        (0.7..0.85, GRAY_THRESHOLD..=0.1) => ("light", "pale"),
        (0.7..0.85, 0.15..) => ("light", "vibrant"),
        (0.7..0.85, _) => ("light", ""),

        (0.85.., GRAY_THRESHOLD..=0.1) => ("very light", "pale"),
        (0.85.., 0.15..) => ("very light", "vibrant"),
        (0.85.., _) => ("very light", ""),

        (_, GRAY_THRESHOLD..=0.1) => ("very light", "grayish"),
        (_, 0.15..) => ("very light", "vibrant"),
        _ => ("very light", ""),
    }
}

/// Converts the RGB color to a (L, C, h) tuple.
fn to_oklch(color: Color) -> (f64, f64, f64) {
    let oklch: Oklch<f64> = color.into_format::<f64>().into_color();
    let (l, c, h) = oklch.into_components();
    (l, c, h.into_degrees())
}

fn oklch_hue(lightness: f64, chroma: f64, hue: f64) -> (String, f64) {
    if let ..GRAY_THRESHOLD = chroma {
        return ("gray".to_string(), lightness);
    }

    let hue = hue.rem_euclid(360.0);

    match (hue, lightness) {
        (0.0..=7.5, _) | (349.0..360.0, _) => ("pink".to_string(), lightness),
        (7.5..15.0, _) => ("pink red".to_string(), lightness),
        (15.0..=31.5, _) => ("red".to_string(), lightness),
        (31.5..48.0, _) => ("red orange".to_string(), lightness),
        (48.0..=71.0, ..ORANGE_LIGHTNESS_THRESHOLD) => ("brown".to_string(), lightness),
        (71.0..94.0, ..ORANGE_LIGHTNESS_THRESHOLD) => ("brown yellow".to_string(), lightness),
        (48.0..=71.0, _) => (
            "orange".to_string(),
            (lightness - ORANGE_LIGHTNESS_THRESHOLD) + MAX_DARK_LIGHTNESS,
        ),
        (71.0..94.0, _) => (
            "orange yellow".to_string(),
            (lightness - ORANGE_LIGHTNESS_THRESHOLD) + MAX_DARK_LIGHTNESS,
        ),
        (94.0..135.0, ..YELLOW_GREEN_LIGHTNESS_THRESHOLD) => {
            ("yellow green".to_string(), lightness)
        }
        (94.0..=114.5, _) => ("yellow".to_string(), lightness),
        (114.5..135.0, _) => ("yellow green".to_string(), lightness),
        (135.0..=155.0, _) => ("green".to_string(), lightness),
        (155.0..175.0, _) => ("green cyan".to_string(), lightness),
        (175.0..=219.5, _) => ("cyan".to_string(), lightness),
        (219.5..264.0, _) => ("cyan blue".to_string(), lightness),
        (264.0..=274.0, _) => ("blue".to_string(), lightness),
        (274.0..284.0, _) => ("blue purple".to_string(), lightness),
        (284.0..=302.0, _) => ("purple".to_string(), lightness),
        (302.0..320.0, _) => ("purple magenta".to_string(), lightness),
        (320.0..=334.5, _) => ("magenta".to_string(), lightness),
        (334.5..349.0, _) => ("magenta pink".to_string(), lightness),
        _ => unreachable!("Unexpected hue"),
    }
}
