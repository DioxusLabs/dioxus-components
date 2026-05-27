use dioxus::prelude::*;
use dioxus_icons::lucide::{Moon, Sun};

const COOKIE_NAME: &str = "dx_theme";
const CHANNEL_NAME: &str = "dx-theme";

pub fn theme_seed() {
    _ = document::eval(&format!(
        r#"
        (function () {{
          if (window.__dx_theme_seeded) return;
          window.__dx_theme_seeded = true;

          const COOKIE_NAME = '{COOKIE_NAME}';
          const CHANNEL_NAME = '{CHANNEL_NAME}';

          function getCookie(name) {{
            const prefix = name + '=';
            const parts = document.cookie.split(';');
            for (let p of parts) {{
              p = p.trim();
              if (p.startsWith(prefix)) return decodeURIComponent(p.slice(prefix.length));
            }}
            return null;
          }}

          function apply(theme) {{
            if (theme === 'dark' || theme === 'light') {{
              document.documentElement.setAttribute('data-theme', theme);
            }} else {{
              document.documentElement.removeAttribute('data-theme');
            }}
          }}

          apply(getCookie(COOKIE_NAME));

          try {{
            const ch = new BroadcastChannel(CHANNEL_NAME);
            ch.addEventListener('message', (event) => {{
              const data = event.data;
              apply(data && data.theme);
            }});
            window.__dx_theme_channel = ch;
          }} catch (_) {{}}
        }})();
        "#,
    ));
}

pub fn set_theme(dark_mode: bool) {
    let theme = if dark_mode { "dark" } else { "light" };

    _ = document::eval(&format!(
        r#"
        (function () {{
          const COOKIE_NAME = '{COOKIE_NAME}';
          const CHANNEL_NAME = '{CHANNEL_NAME}';

          function getCookie(name) {{
            const prefix = name + '=';
            const parts = document.cookie.split(';');
            for (let p of parts) {{
              p = p.trim();
              if (p.startsWith(prefix)) return decodeURIComponent(p.slice(prefix.length));
            }}
            return null;
          }}

          document.documentElement.setAttribute('data-theme', '{theme}');
          if (getCookie(COOKIE_NAME) === '{theme}') return;

          document.cookie = '{COOKIE_NAME}={theme}; path=/; max-age=31536000; samesite=lax';

          try {{
            const ch = window.__dx_theme_channel;
            if (ch && typeof ch.postMessage === 'function') {{
              ch.postMessage({{ theme: '{theme}' }});
            }} else {{
              const tmp = new BroadcastChannel(CHANNEL_NAME);
              tmp.postMessage({{ theme: '{theme}' }});
              tmp.close();
            }}
          }} catch (_) {{}}
        }})();
        "#
    ));
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
