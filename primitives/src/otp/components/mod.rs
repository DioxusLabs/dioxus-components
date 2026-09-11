//! Component definitions for the one-time-password primitive.

mod group;
mod input;
mod separator;
mod slot;

pub use group::{OneTimePasswordGroup, OneTimePasswordGroupProps};
pub use input::{OneTimePasswordInput, OneTimePasswordInputProps};
pub use separator::{OneTimePasswordSeparator, OneTimePasswordSeparatorProps};
pub use slot::{OneTimePasswordSlot, OneTimePasswordSlotProps};
