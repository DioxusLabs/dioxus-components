//! Controlled or uncontrolled open/closed state.

use dioxus::prelude::*;

use crate::use_controlled;

/// Options for [`use_disclosure`].
#[derive(Clone, Copy)]
pub struct UseDisclosureOptions {
    /// Controlled open state. When set, the returned state follows this value.
    pub opened: ReadSignal<Option<bool>>,
    /// Initial open state when uncontrolled.
    pub default_opened: ReadSignal<bool>,
    /// Callback fired when the open state changes.
    pub on_opened_change: Callback<bool>,
}

impl Default for UseDisclosureOptions {
    fn default() -> Self {
        Self {
            opened: ReadSignal::new(Signal::new(None)),
            default_opened: ReadSignal::new(Signal::new(false)),
            on_opened_change: Callback::default(),
        }
    }
}

/// A cloneable handle for disclosure state.
#[derive(Clone, Copy)]
pub struct Disclosure {
    opened: Memo<bool>,
    set_opened: Callback<bool>,
}

impl Disclosure {
    /// Returns whether the disclosure is currently open.
    pub fn opened(&self) -> bool {
        (self.opened)()
    }

    /// Opens the disclosure.
    ///
    /// Returns `true` when this call requested an actual closed-to-open
    /// transition.
    pub fn open(&self) -> bool {
        self.set_opened_if_changed(true)
    }

    /// Closes the disclosure.
    ///
    /// Returns `true` when this call requested an actual open-to-closed
    /// transition.
    pub fn close(&self) -> bool {
        self.set_opened_if_changed(false)
    }

    /// Toggles the disclosure.
    ///
    /// Returns the next open state when this call requested a transition.
    pub fn toggle(&self) -> Option<bool> {
        let next = !self.opened();
        self.set_opened_if_changed(next).then_some(next)
    }

    /// Sets the disclosure open state.
    ///
    /// Returns `true` when the requested value differs from the current value.
    pub fn set_opened(&self, opened: bool) -> bool {
        self.set_opened_if_changed(opened)
    }

    fn set_opened_if_changed(&self, opened: bool) -> bool {
        if self.opened() == opened {
            return false;
        }
        self.set_opened.call(opened);
        true
    }
}

/// Create controlled or uncontrolled disclosure state with transition helpers.
pub fn use_disclosure(options: UseDisclosureOptions) -> Disclosure {
    let (opened, set_opened) = use_controlled(
        options.opened,
        options.default_opened.cloned(),
        options.on_opened_change,
    );

    Disclosure { opened, set_opened }
}
