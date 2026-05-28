# Dioxus spawn from raw wasm-bindgen-x callback MRE

This is a minimal Dioxus desktop reproduction for a panic when a raw `wasm-bindgen-x` DOM callback calls `dioxus::spawn`.

The callback is created inside a Dioxus component hook, but when the browser invokes it later, Dioxus has no current scope on the stack.

## Run

```sh
RUST_BACKTRACE=1 cargo run
```

The app schedules `button.focus()` from JavaScript. That dispatches `focusin`, which invokes the raw Rust callback and calls `dioxus::spawn`.

## Expected

The task is spawned, or the API reports that spawning is unavailable from a raw external callback.

## Actual

The app panics:

```text
thread 'dioxus-desktop-dom' panicked at .../dioxus/packages/core/src/runtime.rs:223:51:
called `Option::unwrap()` on a `None` value
```

Relevant frames:

```text
dioxus_core::runtime::Runtime::current_scope_id
dioxus_core::global_context::spawn
wry_bindgen_event_reentry_mre::App::{{closure}}::{{closure}}
wry_bindgen::encode::<impl ... FnMut<(&A1,)>>::into_js_closure::{{closure}}
wry_bindgen::runtime::handle_rust_callback
```

This is the same failure shape as the component preview crash where a raw DOM listener callback entered `dioxus_primitives::timers::Timeout::new`, which called `dioxus::spawn`.

## Versions

- `dioxus`: git `https://github.com/DioxusLabs/dioxus`, rev `be7bc4683a77b2e8a7c1945fd17f6f7f91af28d0`
- `wasm-bindgen-x`: `0.2.106-alpha.1`
- `web-sys-x`: `0.3.83-alpha.1`
- `wry-bindgen`: pulled by Dioxus desktop / wasm-bindgen-x stack
