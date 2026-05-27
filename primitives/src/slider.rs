//! Defines the [`Slider`] and [`RangeSlider`] components and their sub-components, which provide
//! a range input control for selecting a single value or a value range within a specified range.

use crate::move_interaction::{use_move_interaction, MoveEvent};
use crate::use_controlled;
use dioxus::prelude::*;
use std::ops::Range;
use std::rc::Rc;

fn ordered_range(start: f64, end: f64) -> Range<f64> {
    if start <= end {
        start..end
    } else {
        end..start
    }
}

fn normalize_range(range: Range<f64>) -> Range<f64> {
    ordered_range(range.start, range.end)
}

fn snap_value(value: f64, step: f64) -> f64 {
    (value / step).round() * step
}

fn closest_thumb_for(raw: f64, thumbs: &[f64]) -> usize {
    if thumbs.len() < 2 {
        return 0;
    }

    let d0 = (raw - thumbs[0]).abs();
    let d1 = (raw - thumbs[1]).abs();
    if d0 < d1 {
        0
    } else if d1 < d0 {
        1
    } else if raw < thumbs[0] {
        0
    } else {
        1
    }
}

fn clamp_to_step_bounds(raw: f64, lo: f64, hi: f64, step: f64) -> f64 {
    let snapped = snap_value(raw.clamp(lo, hi), step);
    if snapped > hi {
        ((hi / step).floor() * step).clamp(lo, hi)
    } else if snapped < lo {
        ((lo / step).ceil() * step).clamp(lo, hi)
    } else {
        snapped
    }
}

/// The props for the [`Slider`] component
#[derive(Props, Clone, PartialEq)]
pub struct SliderProps {
    /// The controlled value of the slider
    pub value: ReadSignal<Option<f64>>,

    /// The default value when uncontrolled
    #[props(default = 0.0)]
    pub default_value: f64,

    /// The minimum value
    #[props(default = 0.0)]
    pub min: ReadSignal<f64>,

    /// The maximum value
    #[props(default = 100.0)]
    pub max: ReadSignal<f64>,

    /// The step value
    #[props(default = 1.0)]
    pub step: ReadSignal<f64>,

    /// Whether the slider is disabled
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// Orientation of the slider
    #[props(default = true)]
    pub horizontal: bool,

    /// Inverts the order of the values
    #[props(default)]
    pub inverted: bool,

    /// Callback when value changes
    #[props(default)]
    pub on_value_change: Callback<f64>,

    /// The label for the slider (for accessibility)
    pub label: ReadSignal<Option<String>>,

    /// Additional attributes for the slider
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the slider
    pub children: Element,
}

/// # Slider
///
/// The `Slider` component is a range input control that allows users to select a value along a
/// [`SliderTrack`] by dragging a [`SliderThumb`] with the pointer or using the arrow keys. For a
/// two-thumb range selector, see [`RangeSlider`].
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::slider::{Slider, SliderRange, SliderThumb, SliderTrack};
///
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         Slider {
///             label: "Demo Slider",
///             horizontal: true,
///             default_value: 50.0,
///             SliderTrack {
///                 SliderRange {}
///                 SliderThumb {}
///             }
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`Slider`] component defines the following data attributes you can use to control styling:
/// - `data-disabled`: Indicates if the slider is disabled. Values are `true` or `false`.
/// - `data-orientation`: Indicates the orientation of the slider. Values are `horizontal` or `vertical`.
#[component]
pub fn Slider(props: SliderProps) -> Element {
    let (value, set_value) =
        use_controlled(props.value, props.default_value, props.on_value_change);

    let thumbs = use_memo(move || vec![value()]);
    let set_thumb = use_callback(move |(_idx, v): (usize, f64)| set_value.call(v));

    rsx! {
        SliderImpl {
            thumbs,
            set_thumb,
            min: props.min,
            max: props.max,
            step: props.step,
            disabled: props.disabled,
            horizontal: props.horizontal,
            inverted: props.inverted,
            label: props.label,
            attributes: props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`RangeSlider`] component
#[derive(Props, Clone, PartialEq)]
pub struct RangeSliderProps {
    /// The controlled value of the range slider
    pub value: ReadSignal<Option<Range<f64>>>,

    /// The default value when uncontrolled
    #[props(default = ordered_range(0.0, 100.0))]
    pub default_value: Range<f64>,

    /// The minimum value
    #[props(default = 0.0)]
    pub min: ReadSignal<f64>,

    /// The maximum value
    #[props(default = 100.0)]
    pub max: ReadSignal<f64>,

    /// The step value
    #[props(default = 1.0)]
    pub step: ReadSignal<f64>,

    /// Whether the range slider is disabled
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// Orientation of the range slider
    #[props(default = true)]
    pub horizontal: bool,

    /// Inverts the order of the values
    #[props(default)]
    pub inverted: bool,

    /// Callback when value changes
    #[props(default)]
    pub on_value_change: Callback<Range<f64>>,

    /// The label for the range slider (for accessibility)
    pub label: ReadSignal<Option<String>>,

    /// Additional attributes for the range slider
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the range slider
    pub children: Element,
}

/// # RangeSlider
///
/// The `RangeSlider` component is a two-thumb range input control. Users select a `(start, end)`
/// pair by dragging two [`SliderThumb`]s along a [`SliderTrack`]; a [`SliderRange`] fills the
/// area between them. The active thumb cannot move past its neighbor.
///
/// Render two [`SliderThumb`]s with `index: 0` (start) and `index: 1` (end). For a single-value
/// slider, see [`Slider`].
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::slider::{RangeSlider, SliderRange, SliderThumb, SliderTrack};
///
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         RangeSlider {
///             label: "Range Slider",
///             horizontal: true,
///             default_value: 20.0..80.0,
///             SliderTrack {
///                 SliderRange {}
///                 SliderThumb { index: 0 }
///                 SliderThumb { index: 1 }
///             }
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`RangeSlider`] component defines the following data attributes you can use to control styling:
/// - `data-disabled`: Indicates if the slider is disabled. Values are `true` or `false`.
/// - `data-orientation`: Indicates the orientation of the slider. Values are `horizontal` or `vertical`.
#[component]
pub fn RangeSlider(props: RangeSliderProps) -> Element {
    let (raw_value, set_value) = use_controlled(
        props.value,
        normalize_range(props.default_value),
        props.on_value_change,
    );
    let value = use_memo(move || normalize_range(raw_value()));

    let thumbs = use_memo(move || {
        let v = value();
        vec![v.start, v.end]
    });
    let set_thumb = use_callback(move |(idx, v): (usize, f64)| {
        let cur = value();
        let next = match idx {
            0 => ordered_range(v, cur.end),
            _ => ordered_range(cur.start, v),
        };
        set_value.call(next);
    });

    rsx! {
        SliderImpl {
            thumbs,
            set_thumb,
            min: props.min,
            max: props.max,
            step: props.step,
            disabled: props.disabled,
            horizontal: props.horizontal,
            inverted: props.inverted,
            label: props.label,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct SliderImplProps {
    thumbs: Memo<Vec<f64>>,
    set_thumb: Callback<(usize, f64)>,
    min: ReadSignal<f64>,
    max: ReadSignal<f64>,
    step: ReadSignal<f64>,
    disabled: ReadSignal<bool>,
    horizontal: bool,
    inverted: bool,
    label: ReadSignal<Option<String>>,
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    children: Element,
}

#[component]
fn SliderImpl(props: SliderImplProps) -> Element {
    let orientation = if props.horizontal {
        "horizontal"
    } else {
        "vertical"
    };

    let mut dragging = use_signal(|| false);
    // Index of the thumb currently being interacted with via pointer. Only meaningful while
    // `dragging` is true; reads outside a drag may return a stale value from the prior interaction.
    let active_thumb = use_signal(|| 0usize);

    let ctx = use_context_provider(|| SliderContext {
        thumbs: props.thumbs,
        set_thumb: props.set_thumb,
        min: props.min,
        max: props.max,
        step: props.step,
        disabled: props.disabled,
        horizontal: props.horizontal,
        inverted: props.inverted,
        dragging: dragging.into(),
        active_thumb,
        label: props.label,
    });

    let mut movement = use_move_interaction(dragging);
    let mut granular_thumbs = use_hook(move || CopyValue::new(props.thumbs.peek().clone()));

    let size = movement.rect().map(|r| {
        if props.horizontal {
            r.width()
        } else {
            r.height()
        }
    });

    use_effect(move || {
        if !dragging() {
            return;
        }

        let Some(size) = size else {
            return;
        };

        let Some(move_event) = movement.pointer_move() else {
            return;
        };

        let delta_pos = if ctx.horizontal {
            move_event.delta_x
        } else {
            move_event.delta_y
        };

        let value_delta = delta_pos / size * ctx.range_size();

        let idx = ctx.active_thumb.cloned();
        let mut current = granular_thumbs.cloned();
        let cur_v = current.get(idx).copied().unwrap_or(0.0);
        let raw = cur_v + value_delta;
        if let Some(slot) = current.get_mut(idx) {
            *slot = raw;
        }
        granular_thumbs.set(current);
        ctx.set_thumb.call((idx, ctx.clamp_for(idx, raw)));
    });

    rsx! {
        div {
            role: "group",
            "data-disabled": props.disabled,
            "data-orientation": orientation,

            onmounted: move |evt| async move {
                let mut movement = movement;
                movement.set_mounted(evt.data()).await;
            },
            onresize: move |_| async move {
                let mut movement = movement;
                movement.refresh_rect().await;
            },
            onpointerdown: move |evt| {
                if (ctx.disabled)() {
                    return;
                }

                if !movement.start_pointer(&evt) {
                    return;
                }

                // Handle pointer interaction
                spawn(async move {
                    let mut movement = movement;

                    // Update the bounding rect of the slider in case it moved
                    if let Some(r) = movement.refresh_rect().await {
                        let size = if props.horizontal {
                            r.width()
                        } else {
                            r.height()
                        };

                        // Get the mouse position relative to the slider
                        let top_left = r.origin;
                        let relative_pos = evt.client_coordinates() - top_left.cast_unit();

                        let offset = if ctx.horizontal {
                            relative_pos.x
                        } else {
                            relative_pos.y
                        };
                        let raw = (offset / size) * ctx.range_size() + (ctx.min)();

                        let idx = ctx.closest_thumb(raw);
                        let mut active = ctx.active_thumb;
                        active.set(idx);

                        // Seed granular state from the live thumb values, then write the
                        // unclamped raw position into the active slot for fluid drag.
                        let mut current = (ctx.thumbs)();
                        if let Some(slot) = current.get_mut(idx) {
                            *slot = raw;
                        }
                        granular_thumbs.set(current);

                        ctx.set_thumb.call((idx, ctx.clamp_for(idx, raw)));
                    }

                    dragging.set(true);
                });
            },

            ..props.attributes,

            {props.children}
        }
    }
}

/// The props for the [`SliderTrack`] component
#[derive(Props, Clone, PartialEq)]
pub struct SliderTrackProps {
    /// Additional attributes to apply to the track element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The children of the track which should include a [`SliderThumb`]
    pub children: Element,
}

/// # SliderTrack
///
/// The track component for the [`Slider`] or [`RangeSlider`] that represents the full range of
/// the slider. It serves as the container for the [`SliderRange`] and provides the background
/// track. Clicking along the track will update the value of the slider and move the closest
/// [`SliderThumb`] to the new position.
///
/// This must be used inside a [`Slider`] or [`RangeSlider`] component.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::slider::{Slider, SliderRange, SliderThumb, SliderTrack};
///
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         Slider {
///             label: "Demo Slider",
///             horizontal: true,
///             SliderTrack {
///                 SliderRange {}
///                 SliderThumb {}
///             }
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`SliderTrack`] component defines the following data attributes you can use to control styling:
/// - `data-disabled`: Indicates if the slider is disabled. Values are `true` or `false`.
/// - `data-orientation`: Indicates the orientation of the slider. Values are `horizontal` or `vertical`.
#[component]
pub fn SliderTrack(props: SliderTrackProps) -> Element {
    let ctx = use_context::<SliderContext>();
    let orientation = if ctx.horizontal {
        "horizontal"
    } else {
        "vertical"
    };

    rsx! {
        div {
            "data-disabled": ctx.disabled,
            "data-orientation": orientation,
            ..props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`SliderRange`] component
#[derive(Props, Clone, PartialEq)]
pub struct SliderRangeProps {
    /// Additional attributes to apply to the range element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The children of the range element
    pub children: Element,
}

/// # SliderRange
///
/// The range component for the [`Slider`] or [`RangeSlider`] that visually represents the
/// selected portion of the slider track. For a [`Slider`] it spans from the minimum to the
/// current value; for a [`RangeSlider`] it spans between the two thumbs.
///
/// This must be used inside a [`SliderTrack`] component.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::slider::{Slider, SliderRange, SliderThumb, SliderTrack};
///
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         Slider {
///             label: "Demo Slider",
///             horizontal: true,
///             SliderTrack {
///                 SliderRange {}
///                 SliderThumb {}
///             }
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`SliderRange`] component defines the following data attributes you can use to control styling:
/// - `data-disabled`: Indicates if the slider is disabled. Values are `true` or `false`.
/// - `data-orientation`: Indicates the orientation of the slider. Values are `horizontal` or `vertical`.
///
/// It automatically has the percentage based size and position styles applied based on the current slider value.
#[component]
pub fn SliderRange(props: SliderRangeProps) -> Element {
    let ctx = use_context::<SliderContext>();
    let orientation = if ctx.horizontal {
        "horizontal"
    } else {
        "vertical"
    };

    let style = use_memo(move || {
        let t = (ctx.thumbs)();
        let (start, end) = match t.as_slice() {
            [s, e, ..] => (*s, *e),
            [v] => ((ctx.min)(), *v),
            // Defensive: thumbs is built by Slider/RangeSlider with len 1 or 2, never empty.
            [] => ((ctx.min)(), (ctx.min)()),
        };

        let start_percent = ctx.as_percent(start);
        let end_percent = ctx.as_percent(end);

        if ctx.horizontal {
            format!("left: {}%; right: {}%", start_percent, 100.0 - end_percent)
        } else {
            format!("bottom: {}%; top: {}%", start_percent, 100.0 - end_percent)
        }
    });

    rsx! {
        div {
            "data-disabled": ctx.disabled,
            "data-orientation": orientation,
            style,
            ..props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`SliderThumb`] component
#[derive(Props, Clone, PartialEq)]
pub struct SliderThumbProps {
    /// Which thumb this is in a [`RangeSlider`]. Use `0` for the start thumb and `1` for the
    /// end thumb. Defaults to `0`.
    #[props(default)]
    pub index: Option<usize>,

    /// Additional attributes to apply to the thumb element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The children of the thumb element
    pub children: Element,
}

/// # SliderThumb
///
/// The thumb component for the [`Slider`] or [`RangeSlider`] that users can drag to change the
/// slider value. It supports both mouse/touch interaction and keyboard navigation with arrow
/// keys. Arrow keys move the thumb by the step value by default, or by 10x the step value if
/// the shift key is held down.
///
/// In a [`RangeSlider`] each thumb is constrained by its neighbor: the start thumb cannot move
/// past the end thumb's value, and vice versa.
///
/// This must be used inside a [`SliderTrack`] component.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::slider::{Slider, SliderRange, SliderThumb, SliderTrack};
///
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         Slider {
///             label: "Demo Slider",
///             horizontal: true,
///             SliderTrack {
///                 SliderRange {}
///                 SliderThumb {}
///             }
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`SliderThumb`] component defines the following data attributes you can use to control styling:
/// - `data-disabled`: Indicates if the slider is disabled. Values are `true` or `false`.
/// - `data-orientation`: Indicates the orientation of the slider. Values are `horizontal` or `vertical`.
/// - `data-dragging`: Indicates if the thumb is currently being dragged. Values are `true` or `false`.
/// - `data-index`: The thumb's index (`0` or `1`). Useful for differentiating start/end thumbs in a [`RangeSlider`].
///
/// It automatically has the percentage based position styles applied based on the current slider value.
#[component]
pub fn SliderThumb(props: SliderThumbProps) -> Element {
    let ctx = use_context::<SliderContext>();
    let orientation = if ctx.horizontal {
        "horizontal"
    } else {
        "vertical"
    };

    let index = props.index.unwrap_or(0);

    // Thumb's own value plus the bounds it can reach: in range mode each thumb is capped by
    // its neighbor; otherwise it's [min, max].
    let bounds = use_memo(move || {
        let t = (ctx.thumbs)();
        let value = t.get(index).copied().unwrap_or(0.0);
        let (vmin, vmax) = match (t.len(), index) {
            (2, 0) => ((ctx.min)(), t[1]),
            (2, _) => (t[0], (ctx.max)()),
            _ => ((ctx.min)(), (ctx.max)()),
        };
        (value, vmin, vmax)
    });
    let value = use_memo(move || bounds().0);

    let percent = ctx.as_percent(value());
    let style = if ctx.horizontal {
        format!("left: {percent}%")
    } else {
        format!("bottom: {percent}%")
    };

    let mut button_ref: Signal<Option<Rc<MountedData>>> = use_signal(|| None);

    use_effect(move || {
        let button_ref = button_ref();
        if let Some(button) = button_ref {
            let disabled = ctx.disabled.cloned();
            let dragging = ctx.dragging.cloned();
            let active = ctx.active_thumb.cloned();
            if !disabled && dragging && active == index {
                spawn(async move {
                    _ = button.set_focus(true).await;
                });
            }
        }
    });

    let aria_label = ctx.label;
    let (_, vmin, vmax) = bounds();

    rsx! {
        button {
            type: "button",
            role: "slider",
            aria_valuemin: vmin,
            aria_valuemax: vmax,
            aria_valuenow: value,
            aria_orientation: orientation,
            aria_label,
            "data-disabled": ctx.disabled,
            "data-orientation": orientation,
            "data-dragging": ctx.dragging,
            "data-index": index as i64,
            style,
            tabindex: 0,
            onmounted: move |evt| {
                // Store the mounted data for focus management
                button_ref.set(Some(evt.data()));
            },
            onmousedown: move |evt| {
                // Don't focus the button. The dragging state will handle focus
                evt.prevent_default();
            },
            ontouchstart: move |evt| {
                // Don't focus the button. The dragging state will handle focus
                evt.prevent_default();
            },
            onkeydown: move |evt| async move {
                if (ctx.disabled)() {
                    return;
                }

                let Some(move_event) = MoveEvent::from_keyboard(&evt, (ctx.step)()) else {
                    return;
                };

                evt.prevent_default();
                let new_value = value() + move_event.delta_x + move_event.delta_y;

                // Clamp (against neighbor in range mode) and snap, then commit.
                ctx.set_thumb.call((index, ctx.clamp_for(index, new_value)));
            },
            ..props.attributes,
            {props.children}
        }
    }
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
struct SliderContext {
    thumbs: Memo<Vec<f64>>,
    set_thumb: Callback<(usize, f64)>,
    min: ReadSignal<f64>,
    max: ReadSignal<f64>,
    step: ReadSignal<f64>,
    disabled: ReadSignal<bool>,
    horizontal: bool,
    inverted: bool,
    dragging: ReadSignal<bool>,
    active_thumb: Signal<usize>,
    label: ReadSignal<Option<String>>,
}

impl SliderContext {
    fn range(&self) -> [f64; 2] {
        if !self.inverted {
            [(self.min)(), (self.max)()]
        } else {
            [(self.max)(), (self.min)()]
        }
    }

    fn range_size(&self) -> f64 {
        let [range_min, range_max] = self.range();
        range_max - range_min
    }

    /// Pick the thumb index whose current value is closest to `raw`. On a tie (most commonly
    /// when the two thumbs have collided at the same value) pick by direction so neither thumb
    /// gets stranded: clicks at or to the right of the tied position activate thumb 1, clicks
    /// to the left activate thumb 0. For single-thumb sliders this is always `0`.
    fn closest_thumb(&self, raw: f64) -> usize {
        let t = (self.thumbs)();
        closest_thumb_for(raw, &t)
    }

    /// Clamp `raw` to the bounds the given thumb is allowed to occupy (against the global
    /// min/max and, in range mode, against its neighbor), then snap to a step boundary that
    /// stays inside those bounds. If the snapped value would exceed a non-step-aligned bound
    /// (e.g. a neighbor thumb was set to a fractional value), the result is rounded toward
    /// the bound instead of past it.
    fn clamp_for(&self, index: usize, raw: f64) -> f64 {
        let t = (self.thumbs)();
        let (lo, hi) = ((self.min)(), (self.max)());
        let (lo, hi) = match (t.len(), index) {
            (2, 0) => (lo, t[1]),
            (2, _) => (t[0], hi),
            _ => (lo, hi),
        };
        clamp_to_step_bounds(raw, lo, hi, (self.step)())
    }

    fn as_percent(&self, value: f64) -> f64 {
        let min = (self.min)();
        let max = (self.max)();
        ((value - min) / (max - min) * 100.0).clamp(0.0, 100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn closest_thumb_uses_raw_collision_position() {
        let collided = [80.0, 80.0];

        assert_eq!(closest_thumb_for(79.6, &collided), 0);
        assert_eq!(closest_thumb_for(80.0, &collided), 1);
        assert_eq!(closest_thumb_for(80.4, &collided), 1);
    }

    #[test]
    fn clamp_to_step_bounds_keeps_fallbacks_in_range() {
        assert_eq!(clamp_to_step_bounds(8.0, 5.0, 8.0, 10.0), 5.0);
        assert_eq!(clamp_to_step_bounds(5.0, 5.0, 8.0, 10.0), 5.0);
        assert_eq!(clamp_to_step_bounds(93.0, 93.0, 95.0, 10.0), 95.0);
    }

    #[test]
    fn clamp_to_step_bounds_preserves_available_step_ticks() {
        assert_eq!(clamp_to_step_bounds(85.0, 72.0, 85.0, 10.0), 80.0);
        assert_eq!(clamp_to_step_bounds(84.0, 78.0, 89.0, 10.0), 80.0);
    }
}
