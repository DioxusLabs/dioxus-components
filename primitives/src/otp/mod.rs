//! Defines the [`OneTimePasswordInput`] component and its sub-components for building
//! accessible, composable one-time-password (OTP) inputs.
//!
//! `OneTimePasswordInput` renders a single hidden `<input>` so paste, browser autofill
//! (`autocomplete="one-time-code"`), IME composition, and screen readers all keep working.
//! `OneTimePasswordSlot` children render the visual representation of each character;
//! `OneTimePasswordGroup` and `OneTimePasswordSeparator` are presentational layout helpers.

mod components;
mod context;

pub use components::{
    OneTimePasswordGroup, OneTimePasswordGroupProps, OneTimePasswordInput,
    OneTimePasswordInputProps, OneTimePasswordSeparator, OneTimePasswordSeparatorProps,
    OneTimePasswordSlot, OneTimePasswordSlotProps,
};
