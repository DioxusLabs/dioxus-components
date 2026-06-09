use dioxus::prelude::*;
use dioxus_icons::lucide::{Moon, Sun};
use std::cell::RefCell;
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use web_sys::{HtmlDocument, MessageEvent};

const COOKIE_NAME: &str = "dx_theme";
const CHANNEL_NAME: &str = "dx-theme";

thread_local! {
    static THEME_CHANNEL: RefCell<Option<ThemeChannel>> = const { RefCell::new(None) };
}

struct ThemeChannel {
    channel: JsValue,
    _on_message: Closure<dyn FnMut(MessageEvent)>,
}

pub fn theme_seed() {
    THEME_CHANNEL.with(|theme_channel| {
        if theme_channel.borrow().is_some() {
            return;
        }

        apply_theme(get_cookie(COOKIE_NAME).as_deref());

        let Some(channel) = broadcast_channel() else {
            return;
        };

        let on_message = Closure::wrap(Box::new(move |event: MessageEvent| {
            if let Some(theme) = theme_from_message(event.data()) {
                apply_theme(Some(&theme));
            }
        }) as Box<dyn FnMut(MessageEvent)>);

        let _ = js_sys::Reflect::set(
            &channel,
            &JsValue::from_str("onmessage"),
            on_message.as_ref().unchecked_ref(),
        );
        theme_channel.replace(Some(ThemeChannel {
            channel,
            _on_message: on_message,
        }));
    });
}

pub fn set_theme(dark_mode: bool) {
    let theme = if dark_mode { "dark" } else { "light" };

    apply_theme(Some(theme));
    if get_cookie(COOKIE_NAME).as_deref() != Some(theme) {
        set_cookie(COOKIE_NAME, theme);
    }
    post_theme(theme);
}

fn html_document() -> Option<HtmlDocument> {
    web_sys::window()?
        .document()?
        .dyn_into::<HtmlDocument>()
        .ok()
}

fn get_cookie(name: &str) -> Option<String> {
    let prefix = format!("{name}=");
    html_document()?
        .cookie()
        .ok()?
        .split(';')
        .map(str::trim)
        .find_map(|cookie| cookie.strip_prefix(&prefix).map(str::to_string))
}

fn set_cookie(name: &str, value: &str) {
    if let Some(document) = html_document() {
        let _ = document.set_cookie(&format!(
            "{name}={value}; path=/; max-age=31536000; samesite=lax"
        ));
    }
}

fn apply_theme(theme: Option<&str>) {
    let Some(root) = web_sys::window()
        .and_then(|window| window.document())
        .and_then(|document| document.document_element())
    else {
        return;
    };

    match theme {
        Some(theme @ ("dark" | "light")) => {
            let _ = root.set_attribute("data-theme", theme);
        }
        _ => {
            let _ = root.remove_attribute("data-theme");
        }
    }
}

fn theme_from_message(data: JsValue) -> Option<String> {
    data.as_string().or_else(|| {
        js_sys::Reflect::get(&data, &JsValue::from_str("theme"))
            .ok()
            .and_then(|theme| theme.as_string())
    })
}

fn post_theme(theme: &str) {
    THEME_CHANNEL.with(|theme_channel| {
        if let Some(theme_channel) = theme_channel.borrow().as_ref() {
            let _ = call_channel_method(
                &theme_channel.channel,
                "postMessage",
                &JsValue::from_str(theme),
            );
            return;
        }

        if let Some(channel) = broadcast_channel() {
            let _ = call_channel_method(&channel, "postMessage", &JsValue::from_str(theme));
            let _ = call_channel_method(&channel, "close", &JsValue::UNDEFINED);
        }
    });
}

fn broadcast_channel() -> Option<JsValue> {
    let constructor =
        js_sys::Reflect::get(&js_sys::global(), &JsValue::from_str("BroadcastChannel"))
            .ok()?
            .dyn_into::<js_sys::Function>()
            .ok()?;
    js_sys::Reflect::construct(
        &constructor,
        &js_sys::Array::of1(&JsValue::from_str(CHANNEL_NAME)),
    )
    .ok()
}

fn call_channel_method(
    channel: &JsValue,
    method: &str,
    argument: &JsValue,
) -> Result<JsValue, JsValue> {
    let method = js_sys::Reflect::get(channel, &JsValue::from_str(method))?
        .dyn_into::<js_sys::Function>()?;
    if argument.is_undefined() {
        method.call0(channel)
    } else {
        method.call1(channel, argument)
    }
}

#[component]
pub fn DarkModeToggle() -> Element {
    rsx! {
        button {
            class: "dx-dark-mode-toggle dx-dark-mode-only",
            onclick: move |_| set_theme(false),
            r#type: "button",
            aria_label: "Enable light mode",
            DarkModeIcon {}
        }
        button {
            class: "dx-dark-mode-toggle dx-light-mode-only",
            onclick: move |_| set_theme(true),
            r#type: "button",
            aria_label: "Enable dark mode",
            LightModeIcon {}
        }
    }
}

#[component]
fn DarkModeIcon() -> Element {
    rsx! {
        Moon { size: "24px" }
    }
}

#[component]
fn LightModeIcon() -> Element {
    rsx! {
        Sun { size: "24px" }
    }
}
