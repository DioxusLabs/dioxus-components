use dioxus::prelude::*;
use std::rc::Rc;

pub(crate) type ElementRef = Signal<Option<Rc<MountedData>>>;

#[cfg(all(feature = "native", not(target_arch = "wasm32")))]
mod native {
    use super::*;
    use blitz_dom::{BaseDocument, NodeId};
    use dioxus::html::PlatformEventData;
    use dioxus_core::{AttributeValue, ListenerCallback};
    use dioxus_native::winit::{
        dpi::LogicalPosition,
        event::{ElementState, WindowEvent},
        keyboard::{Key, NamedKey},
        window::Window,
    };
    use dioxus_native_dom::NodeHandle;
    use std::cell::Cell;
    use std::sync::Arc;

    pub(super) fn is_native() -> bool {
        try_consume_context::<Arc<dyn Window>>().is_some()
    }

    pub(super) fn use_keydown_listener(key: &str, mut on_keydown: impl FnMut() + 'static) {
        let key = match key {
            "Escape" => NamedKey::Escape,
            "F6" => NamedKey::F6,
            _ => unreachable!("unsupported global key: {key}"),
        };
        dioxus_native::use_window_event(move |event, _| {
            if let WindowEvent::KeyboardInput { event, .. } = event {
                if event.state == ElementState::Pressed && event.logical_key == Key::Named(key) {
                    on_keydown();
                }
            }
        });
    }

    fn contains(doc: &BaseDocument, root: NodeId, target: Option<NodeId>) -> bool {
        let mut target = target;
        while let Some(id) = target {
            if id == root {
                return true;
            }
            target = doc.get_node(id).and_then(|node| node.parent);
        }
        false
    }

    pub(super) fn use_outside_dismiss(
        id: impl Readable<Target = String> + Copy + 'static,
        open: impl Readable<Target = bool> + Copy + 'static,
        reference: ElementRef,
        on_dismiss: impl FnMut() + Clone + 'static,
    ) {
        let window = dioxus_native::use_window();
        let last_focus = use_hook(|| Rc::new(Cell::new(None)));
        use_effect({
            let last_focus = Rc::clone(&last_focus);
            move || {
                let mounted = reference.cloned().filter(|_| open.cloned());
                let focus = mounted.as_ref().and_then(|mounted| {
                    let node = mounted.downcast::<NodeHandle>()?;
                    Some(node.try_doc()?.get_focussed_node_id())
                });
                last_focus.set(focus);
            }
        });

        dioxus_native::use_window_event(move |event, _| {
            if matches!(event, WindowEvent::KeyboardInput { .. }) && open.cloned() {
                window.request_redraw();
                return;
            }
            if !matches!(
                event,
                WindowEvent::PointerButton {
                    state: ElementState::Pressed,
                    ..
                } | WindowEvent::RedrawRequested
            ) || !open.cloned()
            {
                return;
            }
            let Some(mounted) = reference.peek().clone() else {
                return;
            };
            let Some(node) = mounted.downcast::<NodeHandle>() else {
                return;
            };
            let Some(doc) = node.try_doc() else {
                return;
            };
            let Some(root) = doc.get_element_by_id(&id.cloned()) else {
                return;
            };

            let outside = match event {
                WindowEvent::PointerButton {
                    state: ElementState::Pressed,
                    position,
                    ..
                } => {
                    let scale = doc.viewport().scale_f64();
                    let LogicalPosition { x, y } = position.to_logical::<f32>(scale);
                    #[cfg(not(target_os = "macos"))]
                    let (x, y) = {
                        let insets = window.safe_area();
                        (
                            x - (insets.left as f64 / scale) as f32,
                            y - (insets.top as f64 / scale) as f32,
                        )
                    };
                    let hit = doc.element_from_point(x, y);
                    !contains(&doc, root, hit)
                }
                WindowEvent::RedrawRequested => {
                    let focus = doc.get_focussed_node_id();
                    last_focus
                        .replace(Some(focus))
                        .is_some_and(|last| last != focus)
                        && !contains(&doc, root, focus)
                }
                _ => false,
            };
            if outside {
                let mut on_dismiss = on_dismiss.clone();
                spawn(async move {
                    if open.cloned() {
                        on_dismiss();
                    }
                });
            }
        });
    }

    fn move_focus(node: &NodeHandle, from: Option<NodeId>, backwards: bool) {
        if node.try_doc().is_none() {
            return;
        }
        let mut doc = node.doc_mut();
        let root = node.node_id();
        let mut candidates = Vec::new();
        if let Some(node) = doc.get_node(root) {
            let mut todo: Vec<_> = node.children.iter().rev().copied().collect();
            while let Some(id) = todo.pop() {
                if let Some(child) = doc.get_node(id) {
                    if child.is_focussable() && child.attr("type".into()) != Some("hidden") {
                        candidates.push(id);
                    }
                    todo.extend(child.children.iter().rev().copied());
                }
            }
        }
        let target = next_focus(&candidates, from, backwards).unwrap_or(root);
        if doc.set_focus_to(target) {
            doc.shell_provider.request_redraw();
        }
    }

    fn next_focus<T: Copy + Eq>(items: &[T], from: Option<T>, backwards: bool) -> Option<T> {
        let index = from.and_then(|previous| items.iter().position(|&item| item == previous));
        if backwards {
            index
                .and_then(|i| i.checked_sub(1))
                .and_then(|i| items.get(i))
                .or(items.last())
        } else {
            index.and_then(|i| items.get(i + 1)).or(items.first())
        }
        .copied()
    }

    pub(super) fn use_focus_trap(reference: ElementRef, enabled: impl Fn() -> bool + 'static) {
        crate::use_effect_with_cleanup(move || {
            let mounted = reference.cloned().filter(|_| enabled());
            let previous = mounted.as_ref().and_then(|mounted| {
                let node = mounted.downcast::<NodeHandle>()?;
                let previous = node.try_doc()?.get_focussed_node_id();
                move_focus(node, None, false);
                previous
            });
            move || {
                if let (Some(mounted), Some(previous)) = (mounted, previous) {
                    let node = mounted.downcast::<NodeHandle>().unwrap();
                    if node.try_doc().is_some() {
                        let mut doc = node.doc_mut();
                        if doc.set_focus_to(previous) {
                            doc.shell_provider.request_redraw();
                        }
                    }
                }
            }
        });
    }

    fn add_listener(
        attributes: &mut Vec<Attribute>,
        name: &'static str,
        mut listener: impl FnMut(Event<PlatformEventData>) + 'static,
    ) {
        let existing = attributes
            .iter()
            .position(|attribute| attribute.name == name);
        let previous = existing.map(|index| attributes.remove(index));
        let callback = ListenerCallback::new(move |event: Event<PlatformEventData>| {
            if let Some(Attribute {
                value: AttributeValue::Listener(previous),
                ..
            }) = &previous
            {
                previous.call(event.clone().into_any());
            }
            listener(event);
        });
        attributes.push(Attribute::new(
            name,
            AttributeValue::Listener(callback.erase()),
            None,
            false,
        ));
    }

    pub(super) fn add_element_listeners(
        attributes: &mut Vec<Attribute>,
        mut reference: ElementRef,
        trap_active: impl Fn() -> bool + 'static,
    ) {
        add_listener(attributes, "onmounted", move |event| {
            reference.set(Some(event.map(|data| MountedData::from(data)).data()));
        });
        add_listener(attributes, "onkeydown", move |event| {
            let event = event.map(|data| KeyboardData::from(data));
            if event.key() != dioxus::prelude::Key::Named(dioxus::prelude::NamedKey::Tab)
                || !trap_active()
            {
                return;
            }
            event.prevent_default();
            event.stop_propagation();
            let backwards = event.modifiers().shift();
            spawn(async move {
                if let Some(mounted) = reference.peek().clone() {
                    let Some(node) = mounted.downcast::<NodeHandle>() else {
                        return;
                    };
                    let current = node.try_doc().and_then(|doc| doc.get_focussed_node_id());
                    move_focus(node, current, backwards);
                }
            });
        });
    }

    #[cfg(test)]
    mod tests {
        use super::next_focus;

        #[test]
        fn tab_stays_in_the_overlay_in_both_directions() {
            let nodes = [5, 8, 9];
            assert_eq!(next_focus(&nodes, Some(5), false), Some(8));
            assert_eq!(next_focus(&nodes, Some(9), false), Some(5));
            assert_eq!(next_focus(&nodes, Some(5), true), Some(9));
            assert_eq!(next_focus(&nodes, None, true), Some(9));
            assert_eq!(next_focus::<usize>(&[], None, false), None);
        }
    }
}

pub(crate) fn is_native() -> bool {
    #[cfg(all(feature = "native", not(target_arch = "wasm32")))]
    {
        native::is_native()
    }
    #[cfg(not(all(feature = "native", not(target_arch = "wasm32"))))]
    false
}

pub(crate) fn use_keydown_listener(key: &'static str, on_keydown: impl FnMut() + 'static) {
    #[cfg(all(feature = "native", not(target_arch = "wasm32")))]
    native::use_keydown_listener(key, on_keydown);
    #[cfg(not(all(feature = "native", not(target_arch = "wasm32"))))]
    let _ = (key, on_keydown);
}

pub(crate) fn use_outside_dismiss(
    id: impl Readable<Target = String> + Copy + 'static,
    open: impl Readable<Target = bool> + Copy + 'static,
    reference: ElementRef,
    on_dismiss: impl FnMut() + Clone + 'static,
) {
    #[cfg(all(feature = "native", not(target_arch = "wasm32")))]
    native::use_outside_dismiss(id, open, reference, on_dismiss);
    #[cfg(not(all(feature = "native", not(target_arch = "wasm32"))))]
    let _ = (id, open, reference, on_dismiss);
}

pub(crate) fn use_focus_trap(reference: ElementRef, enabled: impl Fn() -> bool + 'static) {
    #[cfg(all(feature = "native", not(target_arch = "wasm32")))]
    if is_native() {
        native::use_focus_trap(reference, enabled);
    }
    #[cfg(not(all(feature = "native", not(target_arch = "wasm32"))))]
    let _ = (reference, enabled);
}

pub(crate) fn add_element_listeners(
    attributes: Vec<Attribute>,
    reference: ElementRef,
    trap_active: impl Fn() -> bool + 'static,
) -> Vec<Attribute> {
    #[cfg(all(feature = "native", not(target_arch = "wasm32")))]
    if is_native() {
        let mut attributes = attributes;
        native::add_element_listeners(&mut attributes, reference, trap_active);
        return attributes;
    }
    #[cfg(not(all(feature = "native", not(target_arch = "wasm32"))))]
    let _ = (reference, trap_active);
    attributes
}
