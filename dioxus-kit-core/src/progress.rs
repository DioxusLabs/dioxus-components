//! Defines the [`Progress`] component and its sub-components.

use dioxus::prelude::*;

/// The props for the [`Progress`] component.
#[derive(Props, Clone, PartialEq)]
pub struct ProgressProps {
    /// The current progress value, between 0 and max.
    pub value: ReadSignal<Option<f64>>,

    /// The maximum value. Defaults to 100.
    #[props(default = ReadSignal::new(Signal::new(100.0)))]
    pub max: ReadSignal<f64>,

    /// Additional attributes to apply to the progress element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the progress component.
    pub children: Element,
}

/// # Progress
///
/// The `Progress` component shows the progress of an operation.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_kit_core::progress::{Progress, ProgressIndicator};
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         Progress {
///             aria_label: "Progressbar Demo",
///             value: 50.0,
///             ProgressIndicator {}
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`Progress`] component defines the following data attributes you can use to control styling:
/// - `data-state`: Indicates the state of the progress. Values are `loading` or `indeterminate`.
/// - `data-value`: The current progress value between 0 and max.
/// - `data-max`: The maximum progress value.
///
/// The [`Progress`] component defines the following css variables you can use to control styling:
/// - `--progress-value`: A value between 0 and 100 representing the current progress percentage.
#[component]
pub fn Progress(props: ProgressProps) -> Element {
    // Calculate percentage for styling and "data-state"
    let percentage = use_memo(move || {
        props.value.cloned().map(|v| {
            let max = (props.max)();
            (v / max) * 100.0
        })
    });

    let state = use_memo(move || match percentage() {
        Some(_) => "loading",
        None => "indeterminate",
    });

    rsx! {
        div {
            role: "progressbar",
            "aria-valuemin": 0,
            "aria-valuemax": props.max,
            "aria-valuenow": props.value.cloned(),
            "data-state": state,
            "data-value": props.value.cloned().map(|v| v.to_string()),
            "data-max": props.max,
            style: percentage().map(|p| format!("--progress-value: {p}%")),
            ..props.attributes,

            {props.children}
        }
    }
}

/// The props for the [`ProgressIndicator`] component.
#[derive(Props, Clone, PartialEq)]
pub struct ProgressIndicatorProps {
    /// Additional attributes to apply to the indicator element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The children of the indicator component.
    pub children: Element,
}

/// # ProgressIndicator
///
/// The `ProgressIndicator` component represents the visual indicator that shows the progress completion.
///
/// This must be used inside a [`Progress`] component.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_kit_core::progress::{Progress, ProgressIndicator};
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         Progress {
///             aria_label: "Progressbar Demo",
///             value: 50.0,
///             ProgressIndicator {}
///         }
///     }
/// }
/// ```
#[component]
pub fn ProgressIndicator(props: ProgressIndicatorProps) -> Element {
    rsx! {
        div { ..props.attributes, {props.children} }
    }
}
