use std::{collections::BTreeMap, rc::Rc};

use dioxus::prelude::*;

use crate::use_effect_with_cleanup;

pub(crate) fn use_focus_provider(roving_loop: ReadSignal<bool>) -> FocusState {
    use_context_provider(|| FocusState::new(roving_loop))
}

pub(crate) fn use_focus_entry_disabled(
    mut ctx: FocusState,
    index: impl Readable<Target = usize> + Copy + 'static,
    disabled: impl Fn() -> bool + Copy + 'static,
) {
    use_effect_with_cleanup(move || {
        let idx = index.cloned();
        ctx.add_update_item(idx, disabled());
        move || {
            ctx.remove_item(idx);
        }
    });
}

pub(crate) fn use_focus_control(
    ctx: FocusState,
    index: impl Readable<Target = usize> + Copy + 'static,
) -> impl FnMut(MountedEvent) {
    let disabled = || false;
    use_focus_control_disabled(ctx, index, disabled)
}

pub(crate) fn use_focus_control_disabled(
    ctx: FocusState,
    index: impl Readable<Target = usize> + Copy + 'static,
    disabled: impl Fn() -> bool + Copy + 'static,
) -> impl FnMut(MountedEvent) {
    let mut controlled_ref: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    use_effect(move || {
        if disabled() {
            return;
        }
        ctx.control_mount_focus(index.cloned(), controlled_ref);
    });

    move |data: Event<MountedData>| controlled_ref.set(Some(data.data()))
}

pub(crate) fn use_focus_controlled_item_disabled(
    index: impl Readable<Target = usize> + Copy + 'static,
    disabled: impl Fn() -> bool + Copy + 'static,
) -> impl FnMut(MountedEvent) {
    let ctx: FocusState = use_context();
    use_focus_entry_disabled(ctx, index, disabled);
    use_focus_control_disabled(ctx, index, disabled)
}

#[derive(Clone, Copy)]
pub(crate) enum FocusPlacement {
    First,
    Last,
}

pub(crate) fn use_deferred_focus(
    mut ctx: FocusState,
    mut placement: Signal<Option<FocusPlacement>>,
    active: impl Fn() -> bool + Copy + 'static,
) {
    use_effect(move || {
        if !active() {
            placement.set(None);
            return;
        }
        let Some(placement_value) = placement() else {
            return;
        };
        if ctx.try_focus_placement(placement_value) {
            placement.set(None);
        }
    });
}

fn first_enabled<'a>(iter: impl IntoIterator<Item = (&'a usize, &'a bool)>) -> Option<usize> {
    iter.into_iter()
        .find_map(|(&idx, &disabled)| (!disabled).then_some(idx))
}

fn next_index(indices: &[usize], current: Option<usize>, roving_loop: bool) -> Option<usize> {
    match current {
        Some(current) => {
            let next_position = indices.partition_point(|&index| index <= current);
            indices
                .get(next_position)
                .copied()
                .or_else(|| roving_loop.then(|| indices.first().copied()).flatten())
        }
        None => indices.first().copied(),
    }
}

fn prev_index(indices: &[usize], current: Option<usize>, roving_loop: bool) -> Option<usize> {
    match current {
        Some(current) => {
            let prev_position = indices.partition_point(|&index| index < current);
            prev_position
                .checked_sub(1)
                .and_then(|position| indices.get(position).copied())
                .or_else(|| roving_loop.then(|| indices.last().copied()).flatten())
        }
        None if roving_loop => indices.last().copied(),
        None => indices.first().copied(),
    }
}

#[derive(Clone, Copy)]
pub(crate) struct FocusState {
    pub(crate) roving_loop: ReadSignal<bool>,
    pub(crate) recent_focus: Signal<Option<usize>>,
    pub(crate) current_focus: Signal<Option<usize>>,
    items: Signal<BTreeMap<usize, bool>>,
}

impl FocusState {
    pub(crate) fn new(roving_loop: ReadSignal<bool>) -> Self {
        Self {
            roving_loop,
            recent_focus: Signal::new(None),
            current_focus: Signal::new(None),
            items: Signal::new(BTreeMap::new()),
        }
    }

    pub(crate) fn set_focus(&mut self, index: Option<usize>) {
        // Resolve the effective target: an index pointing at a disabled item
        // collapses to None.
        let target = match index {
            Some(idx) if self.items.peek().get(&idx) == Some(&true) => None,
            other => other,
        };
        if let Some(idx) = target {
            self.recent_focus.set(Some(idx));
        }
        // Only notify subscribers when the value actually changes. Otherwise
        // a redundant `blur()` call (with nothing focused) still wakes any
        // effect reading `any_focused()`, which on Brave/Android caused the
        // context menu to auto-close: the menu container's onblur fires
        // shortly after open, calls blur(), and the focus-sync effect then
        // sees `open && !any_focused` and closes the menu.
        if *self.current_focus.peek() != target {
            self.current_focus.set(target);
        }
    }

    pub(crate) fn first_enabled_index(&self) -> Option<usize> {
        first_enabled(self.items.read().iter())
    }

    pub(crate) fn last_enabled_index(&self) -> Option<usize> {
        first_enabled(self.items.read().iter().rev())
    }

    fn enabled_indices(&self) -> Vec<usize> {
        self.items
            .read()
            .iter()
            .filter_map(|(&index, &disabled)| (!disabled).then_some(index))
            .collect()
    }

    fn focus_next_from(&mut self, current: Option<usize>, indices: &[usize]) {
        self.set_focus(next_index(indices, current, (self.roving_loop)()));
    }

    fn focus_prev_from(&mut self, current: Option<usize>, indices: &[usize]) {
        self.set_focus(prev_index(indices, current, (self.roving_loop)()));
    }

    pub(crate) fn focus_next(&mut self) {
        let indices = self.enabled_indices();
        self.focus_next_from(self.recent_focus(), &indices);
    }

    pub(crate) fn focus_prev(&mut self) {
        let indices = self.enabled_indices();
        self.focus_prev_from(self.recent_focus(), &indices);
    }

    pub(crate) fn focus_first(&mut self) {
        self.set_focus(self.first_enabled_index());
    }

    pub(crate) fn focus_last(&mut self) {
        self.set_focus(self.last_enabled_index());
    }

    pub(crate) fn focus_next_from_current(&mut self, indices: &[usize]) {
        self.focus_next_from(self.current_focus(), indices);
    }

    pub(crate) fn focus_prev_from_current(&mut self, indices: &[usize]) {
        self.focus_prev_from(self.current_focus(), indices);
    }

    pub(crate) fn try_focus_placement(&mut self, placement: FocusPlacement) -> bool {
        let Some(index) = (match placement {
            FocusPlacement::First => self.first_enabled_index(),
            FocusPlacement::Last => self.last_enabled_index(),
        }) else {
            return false;
        };
        self.set_focus(Some(index));
        true
    }

    pub(crate) fn blur(&mut self) {
        self.set_focus(None);
    }

    pub(crate) fn any_focused(&self) -> bool {
        self.current_focus.read().is_some()
    }

    pub(crate) fn is_focused(&self, id: usize) -> bool {
        (self.current_focus)().map(|x| x == id).unwrap_or(false)
    }

    pub(crate) fn current_focus(&self) -> Option<usize> {
        (self.current_focus)()
    }

    pub(crate) fn recent_focus(&self) -> Option<usize> {
        (self.recent_focus)()
    }

    pub(crate) fn recent_focus_or_default(&self) -> usize {
        self.recent_focus()
            .filter(|&index| self.is_enabled(index))
            .or_else(|| self.first_enabled_index())
            .unwrap_or_default()
    }

    pub(crate) fn is_enabled(&self, index: usize) -> bool {
        self.items.peek().get(&index) == Some(&false)
    }

    /// Pick the next enabled item after `from`, wrapping when roving_loop is on.
    /// Used to redirect focus that's parked on a known-disabled item.
    fn next_focus_skipping(&self, from: usize) -> Option<usize> {
        let items = self.items.peek();
        first_enabled(items.range(from.saturating_add(1)..)).or_else(|| {
            self.roving_loop
                .peek()
                .then(|| first_enabled(items.iter()))
                .flatten()
        })
    }

    pub(crate) fn add_update_item(&mut self, index: usize, disabled: bool) {
        if self.items.peek().get(&index) == Some(&disabled) {
            return;
        }
        self.items.write().insert(index, disabled);

        let Some(focused) = *self.current_focus.peek() else {
            return;
        };
        if disabled && focused == index {
            // Focus cannot remain on a disabled item.
            self.blur();
        } else if !disabled && self.items.peek().get(&focused) == Some(&true) {
            // Focus is parked on a known-disabled item; advance to the nearest enabled one.
            if let Some(next) = self.next_focus_skipping(focused) {
                self.set_focus(Some(next));
            }
        }
    }

    pub(crate) fn remove_item(&mut self, index: usize) {
        let removed = self.items.write().remove(&index).is_some();
        if removed && (self.current_focus)() == Some(index) {
            self.set_focus(None);
        }
    }

    pub(crate) fn control_mount_focus(
        &self,
        index: usize,
        controlled_ref: Signal<Option<Rc<MountedData>>>,
    ) {
        if self.is_focused(index) && self.is_enabled(index) {
            if let Some(md) = controlled_ref() {
                spawn(async move {
                    let _ = md.set_focus(true).await;
                });
            }
        }
    }
}
