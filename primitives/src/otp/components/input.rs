use super::super::context::OtpCtx;
use crate::{use_controlled, use_id_or, use_unique_id};
use dioxus::prelude::*;
use std::ops::Range;

fn selection_range_for(anchor: usize, focus: usize, len: usize) -> Option<Range<usize>> {
    let start = anchor.min(focus).min(len);
    let end = anchor.max(focus).min(len);

    (start < end).then_some(start..end)
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct InputValueChange {
    value: String,
    cursor: usize,
    changed: bool,
}

impl InputValueChange {
    fn from_raw_input(
        current_value: &str,
        raw_value: &str,
        max: usize,
        cursor: usize,
        selected_range: Option<Range<usize>>,
    ) -> Self {
        if max == 0 {
            return Self {
                value: String::new(),
                cursor: 0,
                changed: false,
            };
        }

        let current_chars: Vec<char> = current_value.chars().collect();
        let raw_chars_full: Vec<char> = raw_value.chars().collect();
        let current_len = current_chars.len();
        let cursor = cursor.min(current_len);

        // Typing one character at the end of a full buffer replaces the last slot.
        // The native input has no maxlength attribute, so it lets the value grow by
        // one; we splice the overflow char into the last slot to match input-otp's
        // "type past the end to overwrite the last digit" behavior.
        if selected_range.is_none()
            && current_len == max
            && raw_chars_full.len() == current_len + 1
            && cursor == max
            && raw_chars_full[..max] == current_chars[..]
        {
            let mut next_chars = current_chars.clone();
            next_chars[max - 1] = raw_chars_full[max];
            let next: String = next_chars.into_iter().collect();
            return Self {
                value: next,
                cursor: max,
                changed: true,
            };
        }

        let raw_chars: Vec<char> = raw_chars_full.into_iter().take(max).collect();
        let range = edit_range_for_entry(cursor, current_len, max, selected_range.clone());
        let (next_chars, next_cursor) = if selected_range.is_none() && !range.is_empty() {
            if let Some(inserted_chars) =
                inserted_chars_at_cursor(&current_chars, &raw_chars, cursor)
            {
                let mut replaced_chars = current_chars.clone();
                replaced_chars.splice(range.start..range.end, inserted_chars.iter().copied());
                replaced_chars.truncate(max);
                let next_cursor = (range.start + inserted_chars.len()).min(replaced_chars.len());
                (replaced_chars, next_cursor)
            } else {
                let next_cursor = raw_value_cursor(&current_chars, &raw_chars, range, current_len);
                (raw_chars, next_cursor)
            }
        } else {
            let next_cursor = raw_value_cursor(&current_chars, &raw_chars, range, current_len);
            (raw_chars, next_cursor)
        };
        let next: String = next_chars.into_iter().collect();
        let changed = next != current_value;

        Self {
            value: next,
            cursor: next_cursor,
            changed,
        }
    }
}

fn edit_range_for_entry(
    cursor: usize,
    len: usize,
    max: usize,
    selected_range: Option<Range<usize>>,
) -> Range<usize> {
    if let Some(range) = selected_range {
        return range.start.min(len)..range.end.min(len);
    }

    let cursor = cursor.min(len);
    if len == 0 || cursor == len && len < max {
        cursor..cursor
    } else {
        let start = cursor.min(len.saturating_sub(1));
        start..(start + 1).min(len)
    }
}

fn inserted_chars_at_cursor(
    current_chars: &[char],
    raw_chars: &[char],
    cursor: usize,
) -> Option<Vec<char>> {
    if raw_chars.len() <= current_chars.len() || cursor > current_chars.len() {
        return None;
    }

    let inserted_len = raw_chars.len() - current_chars.len();
    if cursor + inserted_len > raw_chars.len()
        || raw_chars[..cursor] != current_chars[..cursor]
        || raw_chars[cursor + inserted_len..] != current_chars[cursor..]
    {
        return None;
    }

    Some(raw_chars[cursor..cursor + inserted_len].to_vec())
}

fn raw_value_cursor(
    current_chars: &[char],
    next_chars: &[char],
    range: Range<usize>,
    current_len: usize,
) -> usize {
    let replaced_len = range.len();
    let base_len = current_len.saturating_sub(replaced_len);
    if next_chars.len() >= base_len {
        (range.start + next_chars.len().saturating_sub(base_len)).min(next_chars.len())
    } else {
        current_chars
            .iter()
            .zip(next_chars)
            .take_while(|(a, b)| a == b)
            .count()
    }
}

fn utf16_offset_for_char(value: &str, index: usize) -> usize {
    value
        .chars()
        .take(index)
        .map(char::len_utf16)
        .sum::<usize>()
}

fn char_index_for_utf16(value: &str, utf16_offset: usize) -> usize {
    let mut acc = 0;
    let mut char_count = 0;
    for c in value.chars() {
        if acc >= utf16_offset {
            return char_count;
        }
        let next = acc + c.len_utf16();
        if next > utf16_offset {
            return char_count;
        }
        acc = next;
        char_count += 1;
    }
    char_count
}

fn active_slot_index(cursor: usize, len: usize, max: usize) -> Option<usize> {
    if max == 0 {
        return None;
    }

    Some(cursor.min(len).min(max - 1))
}

fn slot_selection_range(anchor_slot: usize, focus_slot: usize, len: usize) -> Option<Range<usize>> {
    if len == 0 {
        return None;
    }

    let anchor_slot = anchor_slot.min(len - 1);
    let focus_slot = focus_slot.min(len - 1);
    let start = anchor_slot.min(focus_slot);
    let end = anchor_slot.max(focus_slot) + 1;

    Some(start..end.min(len))
}

fn slot_selection_focus_cursor(anchor_slot: usize, focus_slot: usize, len: usize) -> usize {
    if len == 0 {
        return 0;
    }

    if focus_slot < anchor_slot {
        focus_slot.min(len)
    } else {
        (focus_slot + 1).min(len)
    }
}

fn apply_value_change(
    current_value: &str,
    next_value: String,
    max: usize,
    set_value: Callback<String>,
    on_complete: Callback<String>,
) {
    let old_len = current_value.chars().count();
    let len = next_value.chars().count();

    if next_value != current_value {
        set_value.call(next_value.clone());
        if old_len < max && len == max {
            on_complete.call(next_value);
        }
    }
}

fn focus_input(input_id: String) {
    spawn(async move {
        let eval = document::eval(
            r#"
            const id = await dioxus.recv();
            const input = document.getElementById(id);

            if (input && input.focus) {
                input.focus({ preventScroll: true });
            }
            "#,
        );
        let _ = eval.send(input_id);
    });
}

fn selection_direction_for_dom(direction: SelectionDirection) -> &'static str {
    match direction {
        SelectionDirection::Forward => "forward",
        SelectionDirection::Backward => "backward",
        SelectionDirection::None => "none",
    }
}

/// Read the native input's selection from Dioxus selection event data and push
/// it into `cursor` / `selection_range` / `selection_direction`.
///
/// Returned callback is meant to be wired to the input's selection-related
/// event handlers (`onselect`, `onselectionchange`) so we treat the native
/// input as the source of truth for keyboard selection inside it.
fn use_native_selection_sync(
    value: Memo<String>,
    mut cursor: Signal<usize>,
    mut selection_range: Signal<Option<Range<usize>>>,
    mut selection_direction: Signal<SelectionDirection>,
) -> Callback<SelectionEvent> {
    use_callback(move |event: SelectionEvent| {
        let Some(selection) = event.data.selection() else {
            return;
        };

        let start_char;
        let end_char;
        {
            let snapshot = value.peek();
            let range = selection.range();
            start_char = char_index_for_utf16(&snapshot, range.start);
            end_char = char_index_for_utf16(&snapshot, range.end);
        }
        let direction = selection.direction();

        if start_char == end_char {
            if *cursor.peek() != start_char {
                cursor.set(start_char);
            }
            if selection_range.peek().is_some() {
                selection_range.set(None);
            }
            if *selection_direction.peek() != SelectionDirection::None {
                selection_direction.set(SelectionDirection::None);
            }
        } else {
            let focus = match direction {
                SelectionDirection::Backward => start_char,
                _ => end_char,
            };
            if *cursor.peek() != focus {
                cursor.set(focus);
            }
            let new_range = Some(start_char..end_char);
            if *selection_range.peek() != new_range {
                selection_range.set(new_range);
            }
            if *selection_direction.peek() != direction {
                selection_direction.set(direction);
            }
        }
    })
}

async fn sync_input_selection(
    input_id: String,
    selection: Range<usize>,
    direction: SelectionDirection,
) {
    let eval = document::eval(
        r#"
        const id = await dioxus.recv();
        const start = await dioxus.recv();
        const end = await dioxus.recv();
        const direction = await dioxus.recv();
        const input = document.getElementById(id);

        if (input && document.activeElement === input && input.setSelectionRange) {
            input.setSelectionRange(start, end, direction);
        }
        "#,
    );
    let _ = eval.send(input_id);
    let _ = eval.send(selection.start);
    let _ = eval.send(selection.end);
    let _ = eval.send(selection_direction_for_dom(direction).to_string());
}

async fn sync_input_value_and_selection(
    input_id: String,
    value: String,
    selection: Range<usize>,
    direction: SelectionDirection,
) {
    let eval = document::eval(
        r#"
        const id = await dioxus.recv();
        const value = await dioxus.recv();
        const start = await dioxus.recv();
        const end = await dioxus.recv();
        const direction = await dioxus.recv();
        const input = document.getElementById(id);

        if (input) {
            if (input.value !== value) {
                input.value = value;
            }
            if (document.activeElement === input && input.setSelectionRange) {
                input.setSelectionRange(start, end, direction);
            }
        }
        "#,
    );
    let _ = eval.send(input_id);
    let _ = eval.send(value);
    let _ = eval.send(selection.start);
    let _ = eval.send(selection.end);
    let _ = eval.send(selection_direction_for_dom(direction).to_string());
}

#[cfg(test)]
mod tests {
    use super::{
        active_slot_index, char_index_for_utf16, selection_range_for, slot_selection_focus_cursor,
        slot_selection_range, InputValueChange,
    };

    fn input_change(value: &str, cursor: usize, changed: bool) -> InputValueChange {
        InputValueChange {
            value: value.to_string(),
            cursor,
            changed,
        }
    }

    #[test]
    fn input_change_uses_raw_value() {
        assert_eq!(
            InputValueChange::from_raw_input("12", "129", 6, 2, None),
            input_change("129", 3, true)
        );
    }

    #[test]
    fn input_change_replaces_at_visible_cursor() {
        assert_eq!(
            InputValueChange::from_raw_input("12", "192", 6, 1, None),
            input_change("19", 2, true)
        );
    }

    #[test]
    fn multi_character_input_change_replaces_active_slot() {
        assert_eq!(
            InputValueChange::from_raw_input("12", "1982", 6, 1, None),
            input_change("198", 3, true)
        );
    }

    #[test]
    fn unchanged_input_is_not_reported_as_changed() {
        assert_eq!(
            InputValueChange::from_raw_input("192", "192", 6, 3, None),
            input_change("192", 3, false)
        );
    }

    #[test]
    fn typing_past_end_of_full_buffer_replaces_last_slot() {
        // Native input grows by one char; we splice the overflow into the last slot.
        assert_eq!(
            InputValueChange::from_raw_input("123456", "1234569", 6, 6, None),
            input_change("123459", 6, true)
        );
    }

    #[test]
    fn paste_past_end_of_full_buffer_is_truncated() {
        // Multi-char overflow (paste) shouldn't replace; we keep the original buffer.
        assert_eq!(
            InputValueChange::from_raw_input("123456", "12345699", 6, 6, None),
            input_change("123456", 6, false)
        );
    }

    #[test]
    fn input_deletion_uses_raw_value() {
        assert_eq!(
            InputValueChange::from_raw_input("12", "1", 6, 2, None),
            input_change("1", 1, true)
        );
    }

    #[test]
    fn full_input_replacement_uses_raw_value() {
        assert_eq!(
            InputValueChange::from_raw_input("12", "987654", 6, 0, selection_range_for(0, 2, 2)),
            input_change("987654", 6, true)
        );
    }

    #[test]
    fn full_input_replacement_with_shared_prefix_uses_raw_value() {
        assert_eq!(
            InputValueChange::from_raw_input(
                "123456",
                "123999",
                6,
                0,
                selection_range_for(0, 6, 6),
            ),
            input_change("123999", 6, true)
        );
    }

    #[test]
    fn shorter_full_input_replacement_with_shared_prefix_uses_raw_value() {
        assert_eq!(
            InputValueChange::from_raw_input("123456", "1239", 6, 0, selection_range_for(0, 6, 6),),
            input_change("1239", 4, true)
        );
    }

    #[test]
    fn full_input_replacement_truncates_to_maxlength() {
        assert_eq!(
            InputValueChange::from_raw_input(
                "123456",
                "9876543",
                6,
                0,
                selection_range_for(0, 6, 6),
            ),
            input_change("987654", 6, true)
        );
    }

    #[test]
    fn selected_range_replacement_with_shared_digits_uses_raw_value() {
        assert_eq!(
            InputValueChange::from_raw_input(
                "123456",
                "123996",
                6,
                3,
                selection_range_for(3, 5, 6),
            ),
            input_change("123996", 5, true)
        );
    }

    #[test]
    fn char_index_for_utf16_handles_surrogate_pairs() {
        // 😀 is two UTF-16 code units, one char.
        assert_eq!(char_index_for_utf16("😀😃😄", 0), 0);
        assert_eq!(char_index_for_utf16("😀😃😄", 2), 1);
        assert_eq!(char_index_for_utf16("😀😃😄", 4), 2);
        // A utf16 offset that lands inside a surrogate rounds down to the boundary.
        assert_eq!(char_index_for_utf16("😀😃😄", 3), 1);
    }

    #[test]
    fn active_slot_stays_visible_at_end() {
        assert_eq!(active_slot_index(6, 6, 6), Some(5));
    }

    #[test]
    fn active_slot_tracks_next_empty_slot() {
        assert_eq!(active_slot_index(2, 2, 6), Some(2));
    }

    #[test]
    fn slot_selection_range_includes_starting_slot() {
        assert_eq!(slot_selection_range(1, 4, 6), Some(1..5));
        assert_eq!(slot_selection_range(4, 1, 6), Some(1..5));
        assert_eq!(slot_selection_range(5, 5, 6), Some(5..6));
        assert_eq!(slot_selection_range(5, 2, 2), Some(1..2));
    }

    #[test]
    fn slot_selection_cursors_track_drag_direction() {
        assert_eq!(slot_selection_focus_cursor(1, 4, 6), 5);
        assert_eq!(slot_selection_focus_cursor(4, 1, 6), 1);
    }
}

/// The props for the [`OneTimePasswordInput`] component.
#[derive(Props, Clone, PartialEq)]
pub struct OneTimePasswordInputProps {
    /// The controlled value of the OTP input.
    pub value: ReadSignal<Option<String>>,

    /// The default value when uncontrolled.
    #[props(default)]
    pub default_value: String,

    /// The maximum number of characters the input accepts (the total number of slots).
    pub maxlength: ReadSignal<usize>,

    /// Hint for the on-screen keyboard. Defaults to `"numeric"`.
    #[props(default = ReadSignal::new(Signal::new(String::from("numeric"))))]
    pub inputmode: ReadSignal<String>,

    /// Autocomplete hint applied to the underlying input. Defaults to `"one-time-code"`.
    #[props(default = ReadSignal::new(Signal::new(String::from("one-time-code"))))]
    pub autocomplete: ReadSignal<String>,

    /// Whether the input is disabled.
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// Whether the input is required in a form.
    #[props(default)]
    pub required: ReadSignal<bool>,

    /// The name attribute used for form submission.
    #[props(default)]
    pub name: ReadSignal<String>,

    /// Optional id for the inner `<input>`. When omitted, a stable id is generated.
    /// Use this to associate an external `<label for=...>`.
    #[props(default)]
    pub id: ReadSignal<Option<String>>,

    /// Accessible name for the input. Forwarded as `aria-label` on the underlying `<input>`.
    #[props(default)]
    pub aria_label: ReadSignal<Option<String>>,

    /// ID of an element labelling the input. Forwarded as `aria-labelledby` on the underlying `<input>`.
    #[props(default)]
    pub aria_labelledby: ReadSignal<Option<String>>,

    /// Optional validator. Called with the prospective new value when entering characters
    /// (keystrokes and paste); return `false` to reject the change. `Backspace` and `Delete`
    /// bypass the validator and always shrink the value.
    #[props(default)]
    pub validate: Option<Callback<String, bool>>,

    /// Callback fired whenever the value changes.
    #[props(default)]
    pub on_value_change: Callback<String>,

    /// Callback fired when the value reaches `maxlength`.
    #[props(default)]
    pub on_complete: Callback<String>,

    /// Additional attributes applied to the wrapper element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the input — typically [`super::OneTimePasswordGroup`],
    /// [`super::OneTimePasswordSlot`], and [`super::OneTimePasswordSeparator`] components.
    pub children: Element,
}

/// # OneTimePasswordInput
///
/// The `OneTimePasswordInput` is the root of an OTP entry. It renders a single, accessible
/// `<input>` element overlaid on top of its children so paste, autofill (`autocomplete="one-time-code"`),
/// IME composition, and screen readers continue to work, while child [`super::OneTimePasswordSlot`]s
/// render the visual representation of each character.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::otp::{
///     OneTimePasswordInput, OneTimePasswordGroup, OneTimePasswordSlot, OneTimePasswordSeparator,
/// };
///
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         OneTimePasswordInput { maxlength: 6usize,
///             OneTimePasswordGroup {
///                 OneTimePasswordSlot { index: 0usize }
///                 OneTimePasswordSlot { index: 1usize }
///                 OneTimePasswordSlot { index: 2usize }
///             }
///             OneTimePasswordSeparator {}
///             OneTimePasswordGroup {
///                 OneTimePasswordSlot { index: 3usize }
///                 OneTimePasswordSlot { index: 4usize }
///                 OneTimePasswordSlot { index: 5usize }
///             }
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The wrapper sets the following data attributes:
/// - `data-disabled`: `true` or `false` depending on the `disabled` prop.
#[component]
pub fn OneTimePasswordInput(props: OneTimePasswordInputProps) -> Element {
    let maxlength = props.maxlength;
    let on_complete = props.on_complete;
    let validate = props.validate;

    let (value, set_value) =
        use_controlled(props.value, props.default_value, props.on_value_change);

    let generated_id = use_unique_id();
    let input_id = use_id_or(generated_id, props.id);
    let mut is_focused = use_signal(|| false);
    let mut cursor = use_signal(|| 0usize);
    let mut selection_range = use_signal(|| None::<Range<usize>>);
    let mut selection_direction = use_signal(|| SelectionDirection::None);
    let mut selecting_with_pointer = use_signal(|| false);
    let mut pointer_selection_anchor_slot = use_signal(|| None::<usize>);
    let mut pointer_focus_pending = use_hook(|| CopyValue::new(false));
    let disabled = props.disabled;

    // The selection we want the native input to show, in UTF-16 offsets.
    let native_selection = use_memo(move || {
        let current_value = value.read();
        let len = current_value.chars().count();
        let (start, end) = match selection_range
            .cloned()
            .and_then(|r| selection_range_for(r.start, r.end, len))
        {
            Some(r) => (r.start, r.end),
            None => {
                let c = cursor().min(len);
                (c, c)
            }
        };
        utf16_offset_for_char(&current_value, start)..utf16_offset_for_char(&current_value, end)
    });

    let selected_range = use_memo(move || {
        if !is_focused() {
            return None;
        }

        let len = value.read().chars().count();
        selection_range
            .cloned()
            .and_then(|range| selection_range_for(range.start, range.end, len))
    });

    let active_index = use_memo(move || {
        if !is_focused() {
            return None;
        }
        if selected_range.read().is_some() {
            return None;
        }
        let len = value.read().chars().count();
        active_slot_index(cursor(), len, maxlength())
    });

    let begin_slot_selection = use_callback(move |index: usize| {
        if disabled() {
            return;
        }

        let len = value.read().chars().count();
        let next_cursor = (index + 1).min(len);

        is_focused.set(true);
        cursor.set(next_cursor);
        selection_range.set(None);
        selection_direction.set(SelectionDirection::None);
        pointer_selection_anchor_slot.set((index < len).then_some(index));
        selecting_with_pointer.set(true);
        pointer_focus_pending.set(true);
        focus_input(input_id());
    });

    let extend_slot_selection = use_callback(move |index: usize| {
        if !selecting_with_pointer() || disabled() {
            return;
        }

        let len = value.read().chars().count();
        let Some(anchor_slot) = pointer_selection_anchor_slot() else {
            return;
        };
        let next_cursor = slot_selection_focus_cursor(anchor_slot, index, len);
        let direction = if index < anchor_slot {
            SelectionDirection::Backward
        } else {
            SelectionDirection::Forward
        };

        cursor.set(next_cursor);
        selection_range.set(slot_selection_range(anchor_slot, index, len));
        selection_direction.set(direction);
    });

    let end_slot_selection = use_callback(move |index: Option<usize>| {
        if let Some(index) = index {
            if selecting_with_pointer() && selection_range.cloned().is_none() {
                let len = value.read().chars().count();
                cursor.set(index.min(len));
            }
        }
        selecting_with_pointer.set(false);
        pointer_selection_anchor_slot.set(None);
    });

    use_context_provider(|| OtpCtx {
        value,
        disabled: props.disabled,
        active_index,
        selected_range,
        begin_slot_selection,
        extend_slot_selection,
        end_slot_selection,
    });

    // Push our selection state down to the native input. The Dioxus event
    // handlers below push it back via `read_native_selection`; the idempotency
    // checks in that hook break the loop when the two agree.
    use_effect(move || {
        if !is_focused() {
            return;
        }

        let id = input_id();
        let selection = native_selection();
        let direction = selection_direction();

        spawn(async move {
            sync_input_selection(id, selection, direction).await;
        });
    });

    let read_native_selection =
        use_native_selection_sync(value, cursor, selection_range, selection_direction);

    rsx! {
        div {
            role: "group",
            position: "relative",
            "data-disabled": props.disabled,
            onpointerdown: move |event: Event<PointerData>| {
                if disabled() {
                    return;
                }

                event.prevent_default();
                is_focused.set(true);
                cursor.set(value.read().chars().count());
                selection_range.set(None);
                selection_direction.set(SelectionDirection::None);
                pointer_selection_anchor_slot.set(None);
                selecting_with_pointer.set(false);
                pointer_focus_pending.set(true);
                focus_input(input_id());
            },
            onpointerup: move |_| {
                selecting_with_pointer.set(false);
                pointer_selection_anchor_slot.set(None);
            },
            onpointercancel: move |_| {
                selecting_with_pointer.set(false);
                pointer_selection_anchor_slot.set(None);
                pointer_focus_pending.set(false);
            },
            onpointerleave: move |_| {
                selecting_with_pointer.set(false);
                pointer_selection_anchor_slot.set(None);
            },
            ..props.attributes,

            {props.children}

            input {
                id: input_id,
                r#type: "text",
                inputmode: props.inputmode,
                autocomplete: props.autocomplete,
                name: props.name,
                disabled: props.disabled,
                required: props.required,
                aria_label: props.aria_label,
                aria_labelledby: props.aria_labelledby,
                value,

                style: "position:absolute;z-index:20;top:0;left:0;right:0;bottom:0;width:100%;height:100%;opacity:0;color:transparent;background:transparent;caret-color:transparent;outline:none;border:none;padding:0;margin:0;text-align:center;font-family:inherit;font-size:inherit;cursor:text;user-select:none;pointer-events:none;",

                oninput: move |e: FormEvent| {
                    let raw = e.value();
                    let max = maxlength();
                    let current_value = value.read().clone();
                    let selected = selection_range.cloned();
                    let current_cursor = cursor().min(current_value.chars().count());
                    let change = InputValueChange::from_raw_input(
                        &current_value,
                        &raw,
                        max,
                        current_cursor,
                        selected.clone(),
                    );
                    if change.changed {
                        if let Some(validate) = validate {
                            if !validate.call(change.value.clone()) {
                                let id = input_id();
                                let revert_utf16 =
                                    utf16_offset_for_char(&current_value, current_cursor);
                                let revert_range = match selected {
                                    Some(r) => {
                                        utf16_offset_for_char(&current_value, r.start)
                                            ..utf16_offset_for_char(&current_value, r.end)
                                    }
                                    None => revert_utf16..revert_utf16,
                                };
                                let revert_value = current_value;
                                cursor.set(current_cursor);
                                spawn(async move {
                                    sync_input_value_and_selection(
                                        id,
                                        revert_value,
                                        revert_range,
                                        SelectionDirection::None,
                                    )
                                    .await;
                                });
                                return;
                            }
                        }
                    }
                    let next_cursor = change.cursor;
                    let next_value = change.value;
                    if raw != next_value {
                        let id = input_id();
                        let value_to_sync = next_value.clone();
                        let utf16 = utf16_offset_for_char(&value_to_sync, next_cursor);
                        spawn(async move {
                            sync_input_value_and_selection(
                                id,
                                value_to_sync,
                                utf16..utf16,
                                SelectionDirection::None,
                            )
                            .await;
                        });
                    }
                    apply_value_change(
                        &current_value,
                        next_value,
                        max,
                        set_value,
                        on_complete,
                    );
                    cursor.set(next_cursor);
                    selection_range.set(None);
                    selection_direction.set(SelectionDirection::None);
                    pointer_selection_anchor_slot.set(None);
                    selecting_with_pointer.set(false);
                },

                onselect: move |event| read_native_selection.call(event),
                onselectionchange: move |event| read_native_selection.call(event),

                onfocus: move |_| {
                    is_focused.set(true);
                    if pointer_focus_pending() {
                        pointer_focus_pending.set(false);
                        return;
                    }
                    if !selecting_with_pointer() && selection_range().is_none() {
                        cursor.set(value.read().chars().count());
                        selection_range.set(None);
                        selection_direction.set(SelectionDirection::None);
                    }
                },
                onblur: move |_| {
                    is_focused.set(false);
                    selecting_with_pointer.set(false);
                    pointer_selection_anchor_slot.set(None);
                    selection_range.set(None);
                    selection_direction.set(SelectionDirection::None);
                    pointer_focus_pending.set(false);
                },
            }
        }
    }
}
