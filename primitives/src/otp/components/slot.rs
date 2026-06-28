use super::super::context::OtpCtx;
use dioxus::prelude::*;

/// The props for the [`OneTimePasswordSlot`] component.
#[derive(Props, Clone, PartialEq)]
pub struct OneTimePasswordSlotProps {
    /// The position of this slot in the value (zero-based).
    pub index: ReadSignal<usize>,

    /// Additional attributes applied to the slot element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// Optional children rendered after the character (for example, a custom caret element).
    pub children: Element,
}

/// # OneTimePasswordSlot
///
/// A single slot within a [`super::OneTimePasswordInput`]. Renders the character at `index`
/// from the shared value. Must be used inside a [`super::OneTimePasswordInput`].
///
/// ## Styling
///
/// The slot element exposes:
/// - `data-active`: `true` when this slot is the next one to receive input.
/// - `data-selected`: `true` when this slot's character is selected.
/// - `data-selection-start`: `true` when this slot is the first selected slot.
/// - `data-selection-end`: `true` when this slot is the last selected slot.
/// - `data-empty`: `true` when no character has been entered at this position.
/// - `data-disabled`: mirrors the parent's disabled state.
#[component]
pub fn OneTimePasswordSlot(props: OneTimePasswordSlotProps) -> Element {
    let ctx: OtpCtx = use_context();
    let index = props.index;

    let char_at = use_memo(move || {
        ctx.value
            .read()
            .chars()
            .nth(index())
            .map(|c| c.to_string())
            .unwrap_or_default()
    });
    let is_active = use_memo(move || ctx.active_index.cloned() == Some(index()));
    let is_selected = use_memo(move || {
        ctx.selected_range
            .cloned()
            .is_some_and(|range| range.contains(&index()))
    });
    let is_selection_start = use_memo(move || {
        ctx.selected_range
            .cloned()
            .is_some_and(|range| range.start == index())
    });
    let is_selection_end = use_memo(move || {
        ctx.selected_range
            .cloned()
            .is_some_and(|range| range.end == index() + 1)
    });
    let is_empty = use_memo(move || char_at.read().is_empty());

    rsx! {
        div {
            role: "presentation",
            aria_hidden: "true",
            "data-active": is_active,
            "data-selected": is_selected,
            "data-selection-start": is_selection_start,
            "data-selection-end": is_selection_end,
            "data-empty": is_empty,
            "data-disabled": ctx.disabled,
            onpointerdown: move |event: Event<PointerData>| {
                if (ctx.disabled)() {
                    return;
                }

                event.prevent_default();
                event.stop_propagation();
                ctx.begin_slot_selection.call(index());
            },
            onpointermove: move |event: Event<PointerData>| {
                if (ctx.disabled)() {
                    return;
                }

                event.prevent_default();
                ctx.extend_slot_selection.call(index());
            },
            onpointerenter: move |_| {
                if !(ctx.disabled)() {
                    ctx.extend_slot_selection.call(index());
                }
            },
            onpointerup: move |_| ctx.end_slot_selection.call(Some(index())),
            onpointercancel: move |_| ctx.end_slot_selection.call(None),
            ..props.attributes,

            {char_at}
            {props.children}
        }
    }
}
