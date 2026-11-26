use super::MoveInteraction;
use dioxus::html::geometry::{euclid::Vector2D, ClientPoint, ClientSpace};
use dioxus::prelude::*;
use dioxus_native::winit::{
    dpi::{LogicalPosition, PhysicalPosition},
    event::{ButtonSource, ElementState, MouseButton, PointerKind, WindowEvent},
    window::Window,
};
use std::sync::Arc;

#[derive(Clone, Copy, Debug, PartialEq)]
enum PointerId {
    Mouse,
    Pen,
    Touch(i32),
}

impl From<PointerKind> for PointerId {
    fn from(kind: PointerKind) -> Self {
        match kind {
            PointerKind::Touch(finger) => Self::Touch(finger.into_raw() as i32),
            PointerKind::TabletTool(_) => Self::Pen,
            _ => Self::Mouse,
        }
    }
}

#[derive(Clone, Copy)]
pub(super) struct NativePointer {
    id: PointerId,
    pub(super) position: ClientPoint,
    client_offset: Vector2D<f64, ClientSpace>,
}

impl NativePointer {
    pub(super) fn new(event: &Event<PointerData>) -> Self {
        let id = match event.pointer_type().as_str() {
            "touch" => PointerId::Touch(event.pointer_id()),
            "pen" => PointerId::Pen,
            _ => PointerId::Mouse,
        };
        let position = event.client_coordinates();
        // Blitz reports window coordinates as screen coordinates, before subtracting safe insets.
        let client_offset = position - event.screen_coordinates().cast_unit();
        Self {
            id,
            position,
            client_offset,
        }
    }

    fn update_position(&mut self, position: PhysicalPosition<f64>, scale: f64) {
        let LogicalPosition { x, y } = position.to_logical::<f64>(scale);
        self.position = ClientPoint::new(x, y) + self.client_offset;
    }

    fn ends_on(&self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::Focused(false) => true,
            WindowEvent::PointerButton {
                state: ElementState::Released,
                button,
                ..
            } => {
                let id = match button {
                    ButtonSource::Mouse(MouseButton::Left) | ButtonSource::Unknown(_) => {
                        PointerId::Mouse
                    }
                    ButtonSource::Touch { finger_id, .. } => {
                        PointerId::Touch(finger_id.into_raw() as i32)
                    }
                    ButtonSource::TabletTool { .. } => PointerId::Pen,
                    _ => return false,
                };
                self.id == id
            }
            _ => false,
        }
    }
}

pub(super) fn with_window_pointer(mut movement: MoveInteraction) -> MoveInteraction {
    if try_consume_context::<Arc<dyn Window>>().is_none() {
        return movement;
    }

    let window = dioxus_native::use_window();
    let mut pointer = use_signal(|| None::<NativePointer>);
    movement.native_pointer = Some(pointer);
    dioxus_native::use_window_event(move |event, _| {
        let Some(mut active) = *pointer.peek() else {
            return;
        };
        match event {
            WindowEvent::PointerMoved {
                position, source, ..
            } if active.id == PointerKind::from(source.clone()).into() => {
                active.update_position(*position, window.scale_factor());
                pointer.set(Some(active));
            }
            _ if active.ends_on(event) => movement.end_pointer(),
            _ => {}
        }
    });
    movement
}

#[cfg(test)]
mod tests {
    use super::*;
    use dioxus_native::winit::event::FingerId;

    #[test]
    fn window_motion_uses_logical_client_coordinates() {
        let mut pointer = NativePointer {
            id: PointerId::Mouse,
            position: ClientPoint::origin(),
            client_offset: Vector2D::new(-5.0, -10.0),
        };
        pointer.update_position(PhysicalPosition::new(90.0, 120.0), 2.0);
        assert_eq!(pointer.position, ClientPoint::new(40.0, 50.0));
    }

    #[test]
    fn release_and_cancellation_only_end_the_matching_drag() {
        let finger = FingerId::from_raw(7);
        let pointer = NativePointer {
            id: PointerKind::Touch(finger).into(),
            position: ClientPoint::origin(),
            client_offset: Vector2D::zero(),
        };
        let release = |button| WindowEvent::PointerButton {
            button,
            device_id: None,
            position: PhysicalPosition::new(0.0, 0.0),
            primary: true,
            state: ElementState::Released,
            is_macos_activation_click: false,
        };
        assert!(!pointer.ends_on(&release(ButtonSource::Mouse(MouseButton::Left))));
        assert!(pointer.ends_on(&release(ButtonSource::Touch {
            finger_id: finger,
            force: None,
        })));
        let leave = |kind| WindowEvent::PointerLeft {
            kind,
            device_id: None,
            position: None,
            primary: true,
        };
        assert!(!pointer.ends_on(&leave(PointerKind::Touch(FingerId::from_raw(8)))));
        assert!(!pointer.ends_on(&leave(PointerKind::Touch(finger))));
        assert!(pointer.ends_on(&WindowEvent::Focused(false)));
    }
}
