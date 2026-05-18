//! Text search and matching algorithms for the select component.

use crate::selectable::OptionState;
use core::f32;
use std::collections::HashMap;

/// Find the best matching option based on typeahead input
pub(super) fn best_match(
    keyboard: &AdaptiveKeyboard,
    typeahead: &str,
    options: &[OptionState],
) -> Option<usize> {
    if typeahead.is_empty() {
        return None;
    }

    let typeahead_characters: Box<[_]> = typeahead.chars().collect();

    options
        .iter()
        .filter(|opt| !opt.disabled)
        .map(|opt| {
            let value = &opt.text_value;
            let value_characters: Box<[_]> = value.chars().collect();
            let distance = normalized_distance(&typeahead_characters, &value_characters, keyboard);
            (distance, opt.tab_index)
        })
        .min_by(|(d1, _), (d2, _)| f32::total_cmp(d1, d2))
        .map(|(_, value)| value)
}

/// Calculate normalized distance between typeahead and value characters
pub(super) fn normalized_distance(
    typeahead_characters: &[char],
    value_characters: &[char],
    keyboard: &AdaptiveKeyboard,
) -> f32 {
    // Only use the the start of the value characters
    let value_characters =
        &value_characters[..value_characters.len().min(typeahead_characters.len())];
    // Only use the end of the typeahead characters
    let typeahead_characters = &typeahead_characters[typeahead_characters
        .len()
        .saturating_sub(value_characters.len())..];

    levenshtein_distance(typeahead_characters, value_characters, |a, b| {
        keyboard.substitution_cost(a, b)
    })
}

/// The recency bias of the levenshtein distance function
pub(super) fn recency_bias(char_index: usize, total_length: usize) -> f32 {
    ((char_index as f32 + 1.5).ln() / (total_length as f32 + 1.5).ln()).powi(4)
}

// We use a weighted Levenshtein distance to account for the recency of characters
// More recent characters have a higher weight, while older characters have a lower weight
// The first few characters in the value are weighted more heavily
//
// When substitution is required, the substitution is cheaper for characters that are closer together on the keyboard
fn levenshtein_distance(
    typeahead: &[char],
    value: &[char],
    substitution_cost: impl Fn(char, char) -> f32,
) -> f32 {
    let mut dp = vec![vec![0.0; value.len() + 1]; typeahead.len() + 1];

    let mut prev = 0.0;
    for (j, item) in dp[0].iter_mut().enumerate().take(value.len() + 1) {
        let new = prev + (1.0 - recency_bias(j, value.len())) * 0.5;
        prev = new;
        *item = new;
    }
    let mut prev = 0.0;
    for (i, row) in dp.iter_mut().enumerate().take(typeahead.len() + 1) {
        let new = prev + recency_bias(i, typeahead.len()) * 0.5;
        prev = new;
        row[0] = new;
    }

    for i in 1..=typeahead.len() {
        for j in 1..=value.len() {
            let cost = if typeahead[i - 1] == value[j - 1] {
                0.0
            } else {
                substitution_cost(typeahead[i - 1], value[j - 1])
            };

            dp[i][j] = f32::min(
                f32::min(
                    // Insertion is cheaper for old characters in the typeahead
                    dp[i - 1][j] + recency_bias(i, typeahead.len()),
                    // Deletion is cheaper for untyped characters in the value
                    dp[i][j - 1] + (1.0 - recency_bias(j, value.len())),
                ),
                // Substitution
                dp[i - 1][j - 1] + cost * 2.0 * recency_bias(i, typeahead.len()),
            );
        }
    }

    let result = dp[typeahead.len()][value.len()];

    let max_possible = dp[typeahead.len()][0].max(dp[0][value.len()]);

    // Normalize the result to a range of 0.0 to 1.0
    result / max_possible
}

/// Adaptive keyboard learning system for multi-language support
#[derive(Debug, Clone)]
pub struct AdaptiveKeyboard {
    /// Physical key position mappings learned from events
    physical_mappings: HashMap<String, char>,
    /// Our current best guess of the keyboard layout based on learned mappings
    layout: KeyboardLayout,
}

impl Default for AdaptiveKeyboard {
    fn default() -> Self {
        Self::new()
    }
}

impl AdaptiveKeyboard {
    /// Create a new adaptive keyboard system
    pub fn new() -> Self {
        Self {
            physical_mappings: HashMap::new(),
            layout: KeyboardLayout::Qwerty,
        }
    }

    /// Learn from a keyboard event mapping physical key to logical character
    pub fn learn_from_event(&mut self, physical_code: &str, logical_char: char) {
        self.physical_mappings
            .insert(physical_code.to_string(), logical_char);
        self.layout = KeyboardLayout::guess(&self.physical_mappings);
    }

    /// Calculate hybrid substitution cost using multiple strategies
    pub fn substitution_cost(&self, a: char, b: char) -> f32 {
        if a == b {
            return 0.0;
        }

        let a_lowercase = a.to_lowercase().next().unwrap_or(a);
        let b_lowercase = b.to_lowercase().next().unwrap_or(b);

        // Try physical key distance if we have mappings
        let physical_cost =
            self.layout
                .distance_cost(a_lowercase, b_lowercase)
                .map_or(f32::INFINITY, |cost| {
                    cost * 0.3 // Physical proximity is a strong signal
                });

        // Use Unicode codepoint similarity
        let unicode_cost = self.unicode_similarity_cost(a, b);

        // Check phonetic similarity
        let phonetic_cost = self.phonetic_similarity_cost(a_lowercase, b_lowercase);

        // Return the minimum of all costs
        [physical_cost, unicode_cost, phonetic_cost]
            .iter()
            .cloned()
            .fold(f32::INFINITY, f32::min)
    }

    /// Calculate similarity based on Unicode codepoint proximity
    fn unicode_similarity_cost(&self, a: char, b: char) -> f32 {
        let diff = (a as u32).abs_diff(b as u32) as f32;

        // Characters close in Unicode are often similar
        // Scale: adjacent codepoints get ~0.1 cost, distant ones approach 1.0
        (diff / 100.0).clamp(0.1, 1.0)
    }

    /// Check phonetic similarity using small lookup groups
    fn phonetic_similarity_cost(&self, a: char, b: char) -> f32 {
        // Small groups of phonetically similar characters across scripts
        const PHONETIC_GROUPS: &[&[char]] = &[
            // "A" sounds
            &['a', 'а', 'α', 'ا', 'আ'],
            // "B" sounds
            &['b', 'б', 'β', 'ب', 'ব'],
            // "S" sounds
            &['s', 'с', 'σ', 'س', 'স'],
            // "T" sounds
            &['t', 'т', 'τ', 'ت', 'ত'],
            // "N" sounds
            &['n', 'н', 'ν', 'ن', 'ন'],
            // "R" sounds
            &['r', 'р', 'ρ', 'ر', 'র'],
            // "L" sounds
            &['l', 'л', 'λ', 'ل', 'ল'],
            // "M" sounds
            &['m', 'м', 'μ', 'م', 'ম'],
            // "K" sounds
            &['k', 'к', 'κ', 'ك', 'ক'],
            // "P" sounds
            &['p', 'п', 'π', 'پ', 'প'],
            // "F" sounds
            &['f', 'ф', 'φ', 'ف', 'ফ'],
            // "O" sounds
            &['o', 'о', 'ο', 'و', 'ও'],
            // "E" sounds
            &['e', 'е', 'ε', 'ه', 'এ'],
            // "I" sounds
            &['i', 'и', 'ι', 'ي', 'ই'],
            // "U" sounds
            &['u', 'у', 'υ', 'و', 'উ'],
            // "D" sounds
            &['d', 'д', 'δ', 'د', 'দ'],
            // "G" sounds
            &['g', 'г', 'γ', 'ج', 'গ'],
            // "H" sounds
            &['h', 'х', 'η', 'ه', 'হ'],
            // "V" sounds
            &['v', 'в', 'β', 'و', 'ভ'],
            // "Z" sounds
            &['z', 'з', 'ζ', 'ز', 'জ'],
            // "Y" sounds
            &['y', 'й', 'υ', 'ي'],
        ];

        for group in PHONETIC_GROUPS {
            if group.contains(&a) && group.contains(&b) {
                return 0.2; // Low cost for phonetically similar characters
            }
        }

        1.0 // Default high cost
    }
}

/// Supported keyboard layouts for optimized text matching
#[derive(Debug, Clone, Copy, Default, PartialEq)]

pub enum KeyboardLayout {
    Qwerty,
    ColemakDH,
    Colemak,
    Dvorak,
    Workman,
    Azerty,
    Qwertz,
    #[default]
    Unknown,
}

impl KeyboardLayout {
    const KNOWN_KEYBOARD_LAYOUTS: &'static [KeyboardLayout] = &[
        KeyboardLayout::Qwerty,
        KeyboardLayout::ColemakDH,
        KeyboardLayout::Colemak,
        KeyboardLayout::Dvorak,
        KeyboardLayout::Workman,
        KeyboardLayout::Azerty,
        KeyboardLayout::Qwertz,
    ];

    /// Guess the keyboard layout based on observed key positions
    pub fn guess(known_key_positions: &HashMap<String, char>) -> KeyboardLayout {
        Self::KNOWN_KEYBOARD_LAYOUTS
            .iter()
            .copied()
            .find(|layout| {
                known_key_positions.iter().all(|(from, to)| {
                    let Some(from_char) = code_to_char(from) else {
                        return false;
                    };
                    match (
                        Self::Qwerty.char_position(from_char),
                        layout.char_position(*to),
                    ) {
                        (Some(from_pos), Some(to_pos)) => from_pos == to_pos,
                        _ => false,
                    }
                })
            })
            .unwrap_or_default()
    }

    /// Calculate substitution cost between two characters based on keyboard distance
    pub fn distance_cost(&self, a: char, b: char) -> Option<f32> {
        let (a_pos, b_pos) = match (self.char_position(a), self.char_position(b)) {
            (Some(a_pos), Some(b_pos)) => (a_pos, b_pos),
            _ => return None,
        };

        let dx = (a_pos.0 as f32 - b_pos.0 as f32).abs();
        let dy = (a_pos.1 as f32 - b_pos.1 as f32).abs();
        let distance = (dx * dx + dy * dy).sqrt();

        // Scale the distance to be between 0.0 and 1.0
        Some((distance / 10.0).clamp(0.0, 1.0))
    }

    /// Get the position of a character on the keyboard layout
    fn char_position(&self, c: char) -> Option<(usize, usize)> {
        let layout = match self {
            KeyboardLayout::Qwerty => &QWERTY_KEYBOARD_LAYOUT,
            KeyboardLayout::ColemakDH => &COLEMACK_DH_KEYBOARD_LAYOUT,
            KeyboardLayout::Colemak => &COLEMAK_KEYBOARD_LAYOUT,
            KeyboardLayout::Dvorak => &DVORAK_KEYBOARD_LAYOUT,
            KeyboardLayout::Workman => &WORKMAN_KEYBOARD_LAYOUT,
            KeyboardLayout::Azerty => &AZERTY_KEYBOARD_LAYOUT,
            KeyboardLayout::Qwertz => &QWERTZ_KEYBOARD_LAYOUT,
            KeyboardLayout::Unknown => return None,
        };

        for (row_idx, row) in layout.iter().enumerate() {
            for (col_idx, &ch) in row.iter().enumerate() {
                if ch == c {
                    return Some((col_idx, row_idx));
                }
            }
        }
        None
    }
}

/// Convert a key code to a character
pub(super) fn code_to_char(code: &str) -> Option<char> {
    match code {
        "KeyA" => Some('a'),
        "KeyB" => Some('b'),
        "KeyC" => Some('c'),
        "KeyD" => Some('d'),
        "KeyE" => Some('e'),
        "KeyF" => Some('f'),
        "KeyG" => Some('g'),
        "KeyH" => Some('h'),
        "KeyI" => Some('i'),
        "KeyJ" => Some('j'),
        "KeyK" => Some('k'),
        "KeyL" => Some('l'),
        "KeyM" => Some('m'),
        "KeyN" => Some('n'),
        "KeyO" => Some('o'),
        "KeyP" => Some('p'),
        "KeyQ" => Some('q'),
        "KeyR" => Some('r'),
        "KeyS" => Some('s'),
        "KeyT" => Some('t'),
        "KeyU" => Some('u'),
        "KeyV" => Some('v'),
        "KeyW" => Some('w'),
        "KeyX" => Some('x'),
        "KeyY" => Some('y'),
        "KeyZ" => Some('z'),
        "Digit0" => Some('0'),
        "Digit1" => Some('1'),
        "Digit2" => Some('2'),
        "Digit3" => Some('3'),
        "Digit4" => Some('4'),
        "Digit5" => Some('5'),
        "Digit6" => Some('6'),
        "Digit7" => Some('7'),
        "Digit8" => Some('8'),
        "Digit9" => Some('9'),
        _ => None,
    }
}

// Keyboard layout definitions
static QWERTY_KEYBOARD_LAYOUT: [[char; 10]; 4] = [
    ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'],
    ['q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p'],
    ['a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', ';'],
    ['z', 'x', 'c', 'v', 'b', 'n', 'm', ',', '.', '/'],
];

static COLEMACK_DH_KEYBOARD_LAYOUT: [[char; 10]; 4] = [
    ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'],
    ['q', 'w', 'f', 'p', 'b', 'j', 'l', 'u', 'y', ';'],
    ['a', 'r', 's', 't', 'g', 'm', 'n', 'e', 'i', 'o'],
    ['x', 'c', 'd', 'v', 'z', 'k', 'h', ',', '.', '/'],
];

static COLEMAK_KEYBOARD_LAYOUT: [[char; 10]; 4] = [
    ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'],
    ['q', 'w', 'f', 'p', 'g', 'j', 'l', 'u', 'y', ';'],
    ['a', 'r', 's', 't', 'd', 'h', 'n', 'e', 'i', 'o'],
    ['z', 'x', 'c', 'v', 'b', 'k', 'm', ',', '.', '/'],
];

static DVORAK_KEYBOARD_LAYOUT: [[char; 10]; 4] = [
    ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'],
    ['\'', ',', '.', 'p', 'y', 'f', 'g', 'c', 'r', 'l'],
    ['a', 'o', 'e', 'u', 'i', 'd', 'h', 't', 'n', 's'],
    [';', 'q', 'j', 'k', 'x', 'b', 'm', 'w', 'v', 'z'],
];

static WORKMAN_KEYBOARD_LAYOUT: [[char; 10]; 4] = [
    ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'],
    ['q', 'd', 'r', 'w', 'b', 'j', 'f', 'u', 'p', ';'],
    ['a', 's', 'h', 't', 'g', 'y', 'n', 'e', 'o', 'i'],
    ['z', 'x', 'm', 'c', 'v', 'k', 'l', ',', '.', '/'],
];

static AZERTY_KEYBOARD_LAYOUT: [[char; 10]; 4] = [
    ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'],
    ['a', 'z', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p'],
    ['q', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm'],
    ['w', 'x', 'c', 'v', 'b', 'n', ',', ';', ':', '!'],
];

static QWERTZ_KEYBOARD_LAYOUT: [[char; 10]; 4] = [
    ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'],
    ['q', 'w', 'e', 'r', 't', 'z', 'u', 'i', 'o', 'p'],
    ['a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'ö'],
    ['y', 'x', 'c', 'v', 'b', 'n', 'm', ',', '.', '-'],
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::selectable::{OptionState, RcPartialEqValue};
    use std::collections::HashMap;

    #[test]
    fn test_levenshtein_distance() {
        let typeahead = ['a', 'b', 'c'];
        let value = ['a', 'b', 'c'];
        let distance = levenshtein_distance(&typeahead, &value, |_, _| 1.0);
        assert!(distance < 0.01); // Very small but not exactly 0 due to recency bias

        let typeahead = ['a', 'b', 'c'];
        let value = ['a', 'b', 'd'];
        let distance = levenshtein_distance(&typeahead, &value, |_, _| 1.0);
        assert!(distance > 0.0);

        let typeahead = ['a', 'b'];
        let value = ['a', 'b', 'c'];
        let distance = levenshtein_distance(&typeahead, &value, |_, _| 1.0);
        assert!(distance > 0.0);

        let typeahead = ['a', 'b', 'c'];
        let value = ['a', 'b'];
        let distance = levenshtein_distance(&typeahead, &value, |_, _| 1.0);
        assert!(distance > 0.0);

        // Test with keyboard-aware substitution costs
        let typeahead = ['q', 'w'];
        let value = ['q', 'e'];
        let qwerty_distance = levenshtein_distance(&typeahead, &value, |a, b| {
            KeyboardLayout::Qwerty.distance_cost(a, b).unwrap()
        });
        let uniform_distance = levenshtein_distance(&typeahead, &value, |_, _| 1.0);

        // 'w' and 'e' are adjacent on QWERTY, so should have lower cost than uniform
        assert!(qwerty_distance < uniform_distance);
    }

    #[test]
    fn test_normalized_distance() {
        let typeahead_chars = ['a', 'b', 'c'];
        let value_chars = ['a', 'b', 'c'];
        let keyboard = AdaptiveKeyboard::default();

        let distance = normalized_distance(&typeahead_chars, &value_chars, &keyboard);
        assert!(distance < 0.01); // Very small but not exactly 0 due to recency bias

        let typeahead_chars = ['a', 'b', 'c'];
        let value_chars = ['a', 'b', 'd'];
        let distance = normalized_distance(&typeahead_chars, &value_chars, &keyboard);
        assert!(distance > 0.0);

        // Test truncation behavior
        let typeahead_chars = ['a', 'b', 'c', 'd', 'e'];
        let value_chars = ['x', 'y', 'z'];
        let distance = normalized_distance(&typeahead_chars, &value_chars, &keyboard);
        assert!(distance > 0.0);

        // Test with keyboard-aware costs
        let typeahead_chars = ['q'];
        let value_chars = ['w'];
        let qwerty_distance = normalized_distance(&typeahead_chars, &value_chars, &keyboard);

        let typeahead_chars = ['q'];
        let value_chars = ['p'];
        let far_distance = normalized_distance(&typeahead_chars, &value_chars, &keyboard);

        // Adjacent keys should have lower distance than distant keys
        assert!(qwerty_distance < far_distance);
    }

    #[test]
    fn test_detect_keyboard_layout() {
        // Test QWERTY detection
        let mut known_positions = HashMap::new();
        known_positions.insert("KeyQ".to_string(), 'q');
        known_positions.insert("KeyW".to_string(), 'w');
        known_positions.insert("KeyE".to_string(), 'e');

        let detected = KeyboardLayout::guess(&known_positions);
        assert_eq!(detected, KeyboardLayout::Qwerty);

        // Test with colemak dh matches
        let mut known_positions = HashMap::new();
        known_positions.insert("KeyQ".to_string(), 'q');
        known_positions.insert("KeyW".to_string(), 'w');
        known_positions.insert("KeyE".to_string(), 'f');
        known_positions.insert("KeyR".to_string(), 'p');

        let detected = KeyboardLayout::guess(&known_positions);
        assert_eq!(detected, KeyboardLayout::ColemakDH);

        // Test empty input
        let known_positions = HashMap::new();
        let detected = KeyboardLayout::guess(&known_positions);
        assert_eq!(detected, KeyboardLayout::Qwerty);
    }

    #[test]
    fn test_keyboard_layout_substitution_cost() {
        let layout = KeyboardLayout::Qwerty;

        // Same character should have zero cost
        assert_eq!(layout.distance_cost('a', 'a'), Some(0.0));

        // Adjacent characters should have lower cost than distant ones
        let adjacent_cost = layout.distance_cost('q', 'w');
        let distant_cost = layout.distance_cost('q', 'p');
        assert!(adjacent_cost < distant_cost);

        // Unknown characters should return None
        let unknown_cost = layout.distance_cost('α', 'β');
        assert_eq!(unknown_cost, None);
    }

    #[test]
    fn test_best_match() {
        let options = vec![
            OptionState {
                tab_index: 0,
                value: RcPartialEqValue::new("apple"),
                text_value: "Apple".to_string(),
                id: "apple".to_string(),
                disabled: false,
            },
            OptionState {
                tab_index: 1,
                value: RcPartialEqValue::new("banana"),
                text_value: "Banana".to_string(),
                id: "banana".to_string(),
                disabled: false,
            },
            OptionState {
                tab_index: 2,
                value: RcPartialEqValue::new("cherry"),
                text_value: "Cherry".to_string(),
                id: "cherry".to_string(),
                disabled: false,
            },
        ];

        let layout = AdaptiveKeyboard::default();

        // Exact prefix match
        let result = best_match(&layout, "App", &options);
        assert_eq!(result, Some(0));

        // Partial match
        let result = best_match(&layout, "ban", &options);
        assert_eq!(result, Some(1));

        // Empty typeahead should return None
        let result = best_match(&layout, "", &options);
        assert_eq!(result, None);

        // No match should return closest option
        let result = best_match(&layout, "xyz", &options);
        assert!(result.is_some());
    }

    #[test]
    fn test_recency_bias() {
        // Later characters should have higher bias (recency bias favors recent chars)
        let early_bias = recency_bias(0, 10);
        let late_bias = recency_bias(9, 10);
        assert!(late_bias > early_bias);

        // Single character should have maximum bias
        let single_bias = recency_bias(0, 1);
        assert!(single_bias > 0.0);
        assert!(single_bias <= 1.0);
    }

    #[test]
    fn test_adaptive_keyboard_learning() {
        let mut adaptive = AdaptiveKeyboard::new();

        // Test learning from keyboard events
        adaptive.learn_from_event("KeyA", 'ф'); // Russian 'f' sound on A key
        adaptive.learn_from_event("KeyS", 'ы'); // Russian 'y' sound on S key
        assert_eq!(adaptive.layout, KeyboardLayout::Unknown);

        // Should have learned the mappings
        assert_eq!(adaptive.physical_mappings.get("KeyA"), Some(&'ф'));
        assert_eq!(adaptive.physical_mappings.get("KeyS"), Some(&'ы'));

        let options = vec![
            OptionState {
                tab_index: 0,
                value: RcPartialEqValue::new("ф"),
                text_value: "ф".to_string(),
                id: "ф".to_string(),
                disabled: false,
            },
            OptionState {
                tab_index: 1,
                value: RcPartialEqValue::new("banana"),
                text_value: "Banana".to_string(),
                id: "banana".to_string(),
                disabled: false,
            },
        ];

        // ы should be a closer match to ф than banana
        let result = best_match(&adaptive, "ф", &options);
        assert_eq!(result, Some(0));

        // b should still match banana
        let result = best_match(&adaptive, "b", &options);
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_unicode_similarity() {
        let adaptive = AdaptiveKeyboard::new();

        // Close Unicode codepoints should have low cost
        let close_cost = adaptive.unicode_similarity_cost('a', 'b'); // U+0061 vs U+0062
        let far_cost = adaptive.unicode_similarity_cost('a', '中'); // U+0061 vs U+4E2D

        assert!(close_cost < far_cost);
        assert!(close_cost >= 0.1);
        assert!(far_cost <= 1.0);

        // Same characters should have zero cost handled by main function
        assert_eq!(adaptive.substitution_cost('a', 'a'), 0.0);
    }

    #[test]
    fn test_phonetic_similarity() {
        let adaptive = AdaptiveKeyboard::new();

        // Test Latin and Cyrillic 'a' sounds
        let phonetic_cost = adaptive.phonetic_similarity_cost('a', 'а');
        assert_eq!(phonetic_cost, 0.2); // Should be low cost

        // Test Latin and Arabic 'b' sounds
        let phonetic_cost = adaptive.phonetic_similarity_cost('b', 'ب');
        assert_eq!(phonetic_cost, 0.2);

        // Test unrelated characters
        let unrelated_cost = adaptive.phonetic_similarity_cost('x', 'ж');
        assert_eq!(unrelated_cost, 1.0); // Should be high cost
    }

    #[test]
    fn test_hybrid_substitution_cost() {
        let mut adaptive = AdaptiveKeyboard::new();

        // Set up some learned mappings and corrections
        adaptive.learn_from_event("KeyA", 'ф');
        adaptive.learn_from_event("KeyS", 'ы');

        // Test physical key cost for mapped characters
        let physical_cost = adaptive.substitution_cost('ф', 'ы');
        assert!(physical_cost < 1.0); // Should use physical distance

        // Test phonetic similarity fallback
        let phonetic_cost = adaptive.substitution_cost('b', 'ب');
        assert_eq!(phonetic_cost, 0.2);

        // Test unicode similarity fallback
        let unicode_cost = adaptive.substitution_cost('x', 'y');
        assert!(unicode_cost < 1.0);
        assert!(unicode_cost >= 0.1);
    }

    #[test]
    fn test_multilingual_matching() {
        let mut adaptive = AdaptiveKeyboard::new();

        // Russian
        adaptive.learn_from_event("KeyA", 'ф');
        adaptive.learn_from_event("KeyB", 'и');

        // Arabic
        adaptive.learn_from_event("KeyS", 'س');
        adaptive.learn_from_event("KeyT", 'ت');

        // Chinese characters that are far apart in Unicode get high cost
        let distant_chinese_cost = adaptive.substitution_cost('中', '国');
        assert_eq!(distant_chinese_cost, 1.0); // Distant characters should have max cost

        // But closer Chinese characters should have lower cost
        let close_chinese_cost = adaptive.substitution_cost('中', '丰'); // U+4E2D vs U+4E30
        assert!(close_chinese_cost < 1.0); // Close Unicode characters should work

        // Mixed script matching - test f/ф which are phonetically similar
        let mixed_cost = adaptive.substitution_cost('f', 'ф');
        assert!(mixed_cost < 1.0); // Should work through phonetic similarity
    }
}
