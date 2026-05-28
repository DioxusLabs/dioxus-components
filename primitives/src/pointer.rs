use crate::dioxus_core::queue_effect;
use crate::EventListener;
use dioxus::html::geometry::ClientPoint;
use dioxus::prelude::*;
use wasm_bindgen::JsCast;

#[derive(Debug)]
struct Pointer {
    id: i32,
    position: ClientPoint,
}

static POINTERS: GlobalSignal<Vec<Pointer>> = Global::new(|| {
    queue_effect(move || {
        let Some(window) = web_sys::window() else {
            return;
        };

        let pointerdown = EventListener::new(&window, "pointerdown", move |event| {
            if let Some((pointer_id, position)) = pointer_event_position(event) {
                add_pointer(pointer_id, position);
            }
        });
        let pointermove = EventListener::new(&window, "pointermove", move |event| {
            if let Some((pointer_id, position)) = pointer_event_position(event) {
                update_pointer(pointer_id, position);
            }
        });
        let pointerup = EventListener::new(&window, "pointerup", move |event| {
            if let Some((pointer_id, _)) = pointer_event_position(event) {
                remove_pointer(pointer_id);
            }
        });
        let pointercancel = EventListener::new(&window, "pointercancel", move |event| {
            if let Some((pointer_id, _)) = pointer_event_position(event) {
                remove_pointer(pointer_id);
            }
        });

        std::mem::forget((pointerdown, pointermove, pointerup, pointercancel));
    });

    Vec::new()
});

fn pointer_event_position(event: &web_sys::Event) -> Option<(i32, ClientPoint)> {
    let event = event.dyn_ref::<web_sys::PointerEvent>()?;
    Some((
        event.pointer_id(),
        ClientPoint::new(event.client_x() as f64, event.client_y() as f64),
    ))
}

pub(crate) fn track_pointer_down(pointer_id: i32, position: ClientPoint) {
    add_pointer(pointer_id, position);
}

pub(crate) fn pointer_position(pointer_id: i32) -> Option<ClientPoint> {
    POINTERS
        .read()
        .iter()
        .find(|pointer| pointer.id == pointer_id)
        .map(|pointer| pointer.position)
}

fn add_pointer(pointer_id: i32, position: ClientPoint) {
    let mut pointers = POINTERS.write();
    upsert_pointer(&mut pointers, pointer_id, position);
}

fn upsert_pointer(pointers: &mut Vec<Pointer>, pointer_id: i32, position: ClientPoint) {
    if let Some(pointer) = pointers.iter_mut().find(|pointer| pointer.id == pointer_id) {
        pointer.position = position;
    } else {
        pointers.push(Pointer {
            id: pointer_id,
            position,
        });
    }
}

fn update_pointer(pointer_id: i32, position: ClientPoint) {
    if let Some(pointer) = POINTERS
        .write()
        .iter_mut()
        .find(|pointer| pointer.id == pointer_id)
    {
        pointer.position = position;
    }
}

fn remove_pointer(pointer_id: i32) {
    POINTERS.write().retain(|pointer| pointer.id != pointer_id);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn upsert_pointer_updates_existing_pointer() {
        let mut pointers = vec![Pointer {
            id: 1,
            position: ClientPoint::new(10.0, 20.0),
        }];

        upsert_pointer(&mut pointers, 1, ClientPoint::new(30.0, 40.0));

        assert_eq!(pointers.len(), 1);
        assert_eq!(pointers[0].position, ClientPoint::new(30.0, 40.0));
    }
}
