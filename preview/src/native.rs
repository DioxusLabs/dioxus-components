use dioxus::prelude::*;
use std::cell::RefCell;
use std::path::{Path, PathBuf};

thread_local! {
    static CLIPBOARD: RefCell<Option<arboard::Clipboard>> = const { RefCell::new(None) };
}

#[derive(Clone, Copy)]
pub(crate) struct NativeTheme(Signal<bool>);

#[component]
pub fn NativeApp() -> Element {
    let dark = use_context_provider(|| {
        NativeTheme(Signal::new(
            config_path()
                .as_deref()
                .and_then(read_theme)
                .unwrap_or_default(),
        ))
    });
    let theme = theme_name((dark.0)());

    rsx! {
        div { class: "dx-native-preview", "data-theme": theme, super::App {} }
    }
}

impl NativeTheme {
    pub(crate) fn set(mut self, dark: bool) {
        if *self.0.peek() == dark {
            return;
        }
        self.0.set(dark);
        if let Some(path) = config_path() {
            if let Err(error) = write_theme(&path, dark) {
                tracing::warn!(%error, "could not save preview theme");
            }
        }
    }
}

pub(crate) fn copy(text: &str) -> bool {
    let result = CLIPBOARD.with_borrow_mut(|slot| {
        if slot.is_none() {
            *slot = Some(arboard::Clipboard::new()?);
        }
        slot.as_mut().unwrap().set_text(text)
    });
    if let Err(error) = result {
        tracing::warn!(%error, "could not copy preview text");
        return false;
    }
    true
}

fn config_path() -> Option<PathBuf> {
    Some(dirs::config_dir()?.join("dioxus-components").join("theme"))
}

fn read_theme(path: &Path) -> Option<bool> {
    match std::fs::read_to_string(path).ok()?.trim() {
        "dark" => Some(true),
        "light" => Some(false),
        _ => None,
    }
}

fn write_theme(path: &Path, dark: bool) -> std::io::Result<()> {
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }
    std::fs::write(path, theme_name(dark))
}

fn theme_name(dark: bool) -> &'static str {
    if dark {
        "dark"
    } else {
        "light"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn theme_preference_round_trips() -> std::io::Result<()> {
        let base = std::env::temp_dir().join(format!("dioxus-preview-{}", fastrand::u64(..)));
        let path = base.join("settings").join("theme");
        assert_eq!(read_theme(&path), None);
        write_theme(&path, true)?;
        assert_eq!(read_theme(&path), Some(true));
        write_theme(&path, false)?;
        assert_eq!(read_theme(&path), Some(false));
        std::fs::write(&path, "invalid")?;
        assert_eq!(read_theme(&path), None);
        std::fs::remove_dir_all(base)
    }
}
