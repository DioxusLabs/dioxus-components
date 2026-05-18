use crate::dioxus_core::{queue_effect, Runtime};
use dioxus::html::geometry::ClientPoint;
use dioxus::prelude::*;

#[derive(Debug)]
struct Pointer {
    id: i32,
    position: ClientPoint,
}

static POINTERS: GlobalSignal<Vec<Pointer>> = Global::new(|| {
    let runtime = Runtime::current();
    queue_effect(move || {
        runtime.spawn(ScopeId::ROOT, async move {
            let mut pointer_updates = dioxus::document::eval(
                // clientX/clientY (not pageX/pageY) must match element handlers
                // that store `evt.client_coordinates()` and viewport-relative
                // rects from getBoundingClientRect.
                "window.addEventListener('pointerdown', (e) => {
                    dioxus.send(['down', [e.pointerId, e.clientX, e.clientY]]);
                });
                window.addEventListener('pointermove', (e) => {
                    dioxus.send(['move', [e.pointerId, e.clientX, e.clientY]]);
                });
                window.addEventListener('pointerup', (e) => {
                    dioxus.send(['up', [e.pointerId, e.clientX, e.clientY]]);
                });
                window.addEventListener('pointercancel', (e) => {
                    dioxus.send(['up', [e.pointerId, e.clientX, e.clientY]]);
                });",
            );

            while let Ok((event_type, (pointer_id, x, y))) =
                pointer_updates.recv::<(String, (i32, f64, f64))>().await
            {
                let position = ClientPoint::new(x, y);

                match event_type.as_str() {
                    "down" => add_pointer(pointer_id, position),
                    "move" => update_pointer(pointer_id, position),
                    "up" => remove_pointer(pointer_id),
                    _ => {}
                }
            }
        });
    });

    Vec::new()
});

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
