//! Dioxus Kit, a shadcn-styled component library for Dioxus built on top
//! of `dioxus-kit-core`.

use dioxus::prelude::{asset, manganis, Asset};

/// The default shadcn-style theme stylesheet shipped with `dioxus-kit`.
pub const THEME_CSS: Asset = asset!("/assets/dx-components-theme.css");

pub mod accordion;
pub mod alert_dialog;
pub mod aspect_ratio;
pub mod avatar;
pub mod badge;
pub mod button;
pub mod calendar;
pub mod card;
pub mod checkbox;
pub mod collapsible;
pub mod color_picker;
pub mod combobox;
pub mod context_menu;
pub mod date_picker;
pub mod dialog;
pub mod drag_and_drop_list;
pub mod dropdown_menu;
pub mod hover_card;
pub mod input;
pub mod item;
pub mod label;
pub mod menubar;
pub mod navbar;
pub mod pagination;
pub mod popover;
pub mod progress;
pub mod radio_group;
pub mod scroll_area;
pub mod select;
pub mod separator;
pub mod sheet;
pub mod sidebar;
pub mod skeleton;
pub mod slider;
pub mod switch;
pub mod tabs;
pub mod textarea;
pub mod toast;
pub mod toggle;
pub mod toggle_group;
pub mod toolbar;
pub mod tooltip;
pub mod virtual_list;
