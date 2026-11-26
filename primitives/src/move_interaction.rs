use dioxus::html::geometry::euclid::Rect;
use dioxus::html::geometry::euclid::Vector2D;
use dioxus::html::geometry::ClientPoint;
use dioxus::html::geometry::Pixels;
use dioxus::html::input_data::MouseButton;
use dioxus::prelude::*;
use std::future::Future;
use std::rc::Rc;

#[cfg(all(feature = "native", not(target_arch = "wasm32")))]
mod native;

/// Keyboard modifier state attached to a move event.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct MoveModifiers {
    pub(crate) alt_key: bool,
    pub(crate) ctrl_key: bool,
    pub(crate) meta_key: bool,
    pub(crate) shift_key: bool,
}

/// A normalized movement delta.
///
/// Pointer deltas are reported in CSS pixels. Keyboard deltas use the caller's
/// provided step value.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct MoveEvent {
    pub(crate) delta_x: f64,
    pub(crate) delta_y: f64,
    pub(crate) modifiers: MoveModifiers,
}

impl MoveEvent {
    pub(crate) fn from_keyboard(event: &Event<KeyboardData>, step: f64) -> Option<Self> {
        let modifiers = event.data().modifiers();
        let modifiers = MoveModifiers {
            alt_key: modifiers.alt(),
            ctrl_key: modifiers.ctrl(),
            meta_key: modifiers.meta(),
            shift_key: modifiers.shift(),
        };
        let delta = if modifiers.shift_key {
            step * 10.0
        } else {
            step
        };

        let (delta_x, delta_y) = match event.data().key() {
            Key::Named(NamedKey::ArrowUp) => (0.0, delta),
            Key::Named(NamedKey::ArrowDown) => (0.0, -delta),
            Key::Named(NamedKey::ArrowRight) => (delta, 0.0),
            Key::Named(NamedKey::ArrowLeft) => (-delta, 0.0),
            _ => return None,
        };

        Some(Self {
            delta_x,
            delta_y,
            modifiers,
        })
    }
}

/// Shared movement state for controls that support pointer dragging and arrow keys.
#[derive(Clone, Copy)]
pub(crate) struct MoveInteraction {
    rect: Signal<Option<Rect<f64, Pixels>>>,
    element: Signal<Option<Rc<MountedData>>>,
    active_pointer_id: Signal<Option<i32>>,
    captured_position: Signal<Option<ClientPoint>>,
    last_pointer_position: CopyValue<Option<ClientPoint>>,
    dragging: Signal<bool>,
    #[cfg(all(feature = "native", not(target_arch = "wasm32")))]
    native_pointer: Option<Signal<Option<native::NativePointer>>>,
}

pub(crate) fn use_move_interaction(dragging: Signal<bool>) -> MoveInteraction {
    let rect = use_signal(|| None);
    let element = use_signal(|| None);
    let active_pointer_id = use_signal(|| None);
    let captured_position = use_signal(|| None);
    let last_pointer_position = use_hook(|| CopyValue::new(None::<ClientPoint>));

    let movement = MoveInteraction {
        rect,
        element,
        active_pointer_id,
        captured_position,
        last_pointer_position,
        dragging,
        #[cfg(all(feature = "native", not(target_arch = "wasm32")))]
        native_pointer: None,
    };
    #[cfg(all(feature = "native", not(target_arch = "wasm32")))]
    let movement = native::with_window_pointer(movement);
    movement
}

impl MoveInteraction {
    pub(crate) fn rect(&self) -> Option<Rect<f64, Pixels>> {
        self.rect.cloned()
    }

    pub(crate) async fn set_mounted(&mut self, mounted: Rc<MountedData>) {
        self.element.set(Some(mounted.clone()));
        if let Ok(rect) = mounted.get_client_rect().await {
            self.rect.set(Some(rect));
        }
    }

    pub(crate) async fn refresh_rect(&mut self) -> Option<Rect<f64, Pixels>> {
        let element = (self.element)()?;

        if let Ok(rect) = element.get_client_rect().await {
            self.rect.set(Some(rect));
            Some(rect)
        } else {
            None
        }
    }

    pub(crate) fn start_pointer(
        &mut self,
        event: &Event<PointerData>,
    ) -> Option<impl Future<Output = bool> + 'static> {
        event.prevent_default();
        event.stop_propagation();

        if self.active_pointer_id.read().is_some()
            || event.trigger_button() != Some(MouseButton::Primary)
        {
            return None;
        }

        let element = self.element.peek().clone()?;
        let pointer_id = event.data().pointer_id();
        self.active_pointer_id.set(Some(pointer_id));
        let position = event.client_coordinates();
        self.last_pointer_position.set(Some(position));
        self.captured_position.set(Some(position));
        let capture = element.set_pointer_capture(pointer_id);
        #[cfg(all(feature = "native", not(target_arch = "wasm32")))]
        let native_start = self
            .native_pointer
            .map(|_| native::NativePointer::new(event));

        let mut movement = *self;
        Some(async move {
            let result = capture.await;
            if *movement.active_pointer_id.peek() != Some(pointer_id) {
                return false;
            }
            match result {
                Ok(()) => true,
                #[cfg(all(feature = "native", not(target_arch = "wasm32")))]
                Err(dioxus::html::MountedError::NotSupported) if native_start.is_some() => {
                    if let Some(mut pointer) = movement.native_pointer {
                        pointer.set(native_start);
                    }
                    true
                }
                Err(_) => {
                    movement.end_pointer();
                    false
                }
            }
        })
    }

    pub(crate) fn begin_drag(&mut self, event: &Event<PointerData>) {
        if self.is_active(event) {
            self.dragging.set(true);
        }
    }

    pub(crate) fn move_capture(&mut self, event: &Event<PointerData>) {
        if !self.uses_window_events() && self.is_active(event) {
            self.captured_position.set(Some(event.client_coordinates()));
        }
    }

    pub(crate) fn finish_capture(&mut self, event: &Event<PointerData>) {
        if !self.uses_window_events() && self.is_active(event) {
            self.end_pointer();
        }
    }

    pub(crate) fn release_capture(&mut self, event: &Event<PointerData>) {
        if !self.is_active(event) {
            return;
        }
        let uses_window_events = self.uses_window_events();
        self.end_pointer();
        if !uses_window_events {
            if let Some(element) = self.element.peek().as_ref() {
                let release = element.release_pointer_capture(event.pointer_id());
                spawn(async move {
                    let _ = release.await;
                });
            }
        }
    }

    fn is_active(&self, event: &Event<PointerData>) -> bool {
        *self.active_pointer_id.peek() == Some(event.pointer_id())
    }

    fn uses_window_events(&self) -> bool {
        #[cfg(all(feature = "native", not(target_arch = "wasm32")))]
        {
            self.native_pointer
                .is_some_and(|pointer| pointer.peek().is_some())
        }
        #[cfg(not(all(feature = "native", not(target_arch = "wasm32"))))]
        false
    }

    pub(crate) fn pointer_move(&mut self) -> Option<MoveEvent> {
        if !(self.dragging)() {
            return None;
        }

        let pointer_position = self.pointer_position()?;

        let delta = if let Some(last_position) =
            self.last_pointer_position.replace(Some(pointer_position))
        {
            pointer_position - last_position
        } else {
            Vector2D::zero()
        };

        Some(MoveEvent {
            delta_x: delta.x,
            delta_y: delta.y,
            modifiers: MoveModifiers::default(),
        })
    }

    fn pointer_position(&self) -> Option<ClientPoint> {
        #[cfg(all(feature = "native", not(target_arch = "wasm32")))]
        if let Some(pointer) = self.native_pointer {
            if let Some(pointer) = *pointer.read() {
                return Some(pointer.position);
            }
        }
        self.captured_position.cloned()
    }

    fn end_pointer(&mut self) {
        #[cfg(all(feature = "native", not(target_arch = "wasm32")))]
        if let Some(mut pointer) = self.native_pointer {
            pointer.set(None);
        }
        self.captured_position.set(None);
        self.active_pointer_id.take();
        self.last_pointer_position.set(None);
        self.dragging.set(false);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dioxus::html::{Code, HasKeyboardData, Location, Modifiers, ModifiersInteraction};
    use std::rc::Rc;

    struct TestKeyboardData {
        key: Key,
        modifiers: Modifiers,
    }

    impl ModifiersInteraction for TestKeyboardData {
        fn modifiers(&self) -> Modifiers {
            self.modifiers
        }
    }

    impl HasKeyboardData for TestKeyboardData {
        fn key(&self) -> Key {
            self.key.clone()
        }

        fn code(&self) -> Code {
            Code::Unidentified
        }

        fn location(&self) -> Location {
            Location::Standard
        }

        fn is_auto_repeating(&self) -> bool {
            false
        }

        fn is_composing(&self) -> bool {
            false
        }

        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
    }

    fn keyboard_event(key: Key, modifiers: Modifiers) -> Event<KeyboardData> {
        Event::new(
            Rc::new(KeyboardData::new(TestKeyboardData { key, modifiers })),
            true,
        )
    }

    #[test]
    fn keyboard_move_maps_arrow_keys() {
        assert_eq!(
            MoveEvent::from_keyboard(
                &keyboard_event(Key::Named(NamedKey::ArrowUp), Modifiers::empty()),
                2.0
            ),
            Some(MoveEvent {
                delta_x: 0.0,
                delta_y: 2.0,
                modifiers: MoveModifiers::default(),
            })
        );
        assert_eq!(
            MoveEvent::from_keyboard(
                &keyboard_event(Key::Named(NamedKey::ArrowDown), Modifiers::empty()),
                2.0
            )
            .map(|event| (event.delta_x, event.delta_y)),
            Some((0.0, -2.0))
        );
        assert_eq!(
            MoveEvent::from_keyboard(
                &keyboard_event(Key::Named(NamedKey::ArrowRight), Modifiers::empty()),
                2.0
            )
            .map(|event| (event.delta_x, event.delta_y)),
            Some((2.0, 0.0))
        );
        assert_eq!(
            MoveEvent::from_keyboard(
                &keyboard_event(Key::Named(NamedKey::ArrowLeft), Modifiers::empty()),
                2.0
            )
            .map(|event| (event.delta_x, event.delta_y)),
            Some((-2.0, 0.0))
        );
    }

    #[test]
    fn keyboard_move_applies_shift_multiplier() {
        let expected_modifiers = MoveModifiers {
            shift_key: true,
            ..MoveModifiers::default()
        };

        assert_eq!(
            MoveEvent::from_keyboard(
                &keyboard_event(Key::Named(NamedKey::ArrowRight), Modifiers::SHIFT),
                2.0
            )
            .map(|event| (event.delta_x, event.delta_y, event.modifiers)),
            Some((20.0, 0.0, expected_modifiers))
        );
    }

    #[test]
    fn keyboard_move_ignores_non_arrow_keys() {
        assert_eq!(
            MoveEvent::from_keyboard(
                &keyboard_event(Key::Character("a".to_string()), Modifiers::empty()),
                2.0
            ),
            None
        );
    }
}
