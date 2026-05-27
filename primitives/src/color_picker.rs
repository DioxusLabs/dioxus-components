//! Defines the [`ColorPicker`] component and its sub-components.

use crate::dioxus_elements::geometry::ClientPoint;
use crate::move_interaction::{use_move_interaction, MoveEvent};
use dioxus::html::geometry::euclid::Size2D;
use dioxus::html::geometry::PixelsSize;
use dioxus::prelude::*;
use palette::{encoding, FromColor, Hsv, RgbHue, Srgb};

use std::rc::Rc;

mod color_naming;
use color_naming::color_name;

/// Represents an sRGB color.
pub type Color = Srgb<u8>;

const COLOR_AREA_MIN: f64 = 0.0;
const COLOR_AREA_MAX: f64 = 100.0;
const COLOR_AREA_RANGE: f64 = COLOR_AREA_MAX - COLOR_AREA_MIN;

fn color_hex(color: Color) -> String {
    format!("#{color:X}")
}

fn area_value_from_hsv(hsv: Hsv<encoding::Srgb, f64>) -> ClientPoint {
    ClientPoint::new(hsv.saturation, hsv.value) * COLOR_AREA_RANGE
}

fn set_area_value(ctx: ColorPickerContext, value: ClientPoint) {
    let scaled = value / COLOR_AREA_RANGE;
    ctx.set_sv(scaled.x, scaled.y);
}

fn snap_area_value(value: ClientPoint, step: f64) -> ClientPoint {
    value.map(|v| (v / step).round() * step)
}

fn clamp_area_value(value: ClientPoint, step: f64) -> ClientPoint {
    let clamped = value.map(|v| v.clamp(COLOR_AREA_MIN, COLOR_AREA_MAX));
    snap_area_value(clamped, step)
}

fn area_percent(value: ClientPoint) -> PixelsSize {
    let scaled = value.map(|v| ((v - COLOR_AREA_MIN) / COLOR_AREA_RANGE * 100.0).clamp(0.0, 100.0));
    PixelsSize::new(scaled.x, scaled.y)
}

/// Context provided by [`ColorPicker`] to its descendants.
///
/// The picker is controlled in HSV — [`Self::color`] echoes the controlled
/// prop, and the setter methods emit `on_color_change` after applying the
/// requested edit on top of the current value.
#[derive(Clone, Copy)]
pub struct ColorPickerContext {
    color: ReadSignal<Hsv<encoding::Srgb, f64>>,
    on_color_change: Callback<Hsv<encoding::Srgb, f64>>,
}

impl ColorPickerContext {
    /// Read the current HSV color.
    pub fn color(&self) -> Hsv<encoding::Srgb, f64> {
        (self.color)()
    }

    /// Replace the entire HSV color.
    pub fn set_color(&self, c: Hsv<encoding::Srgb, f64>) {
        self.on_color_change.call(c);
    }

    /// Set hue, keeping saturation and value.
    pub fn set_hue(&self, h: f64) {
        let current = (self.color)();
        self.on_color_change.call(Hsv::<encoding::Srgb, f64>::new(
            RgbHue::new(h),
            current.saturation,
            current.value,
        ));
    }

    /// Set saturation and value as a pair, keeping hue.
    pub fn set_sv(&self, s: f64, v: f64) {
        let current = (self.color)();
        self.on_color_change
            .call(Hsv::<encoding::Srgb, f64>::new(current.hue, s, v));
    }
}

/// The props for the [`ColorPicker`] component.
#[derive(Props, Clone, PartialEq)]
pub struct ColorPickerProps {
    /// The selected color
    #[props(default)]
    pub color: ReadSignal<Hsv<encoding::Srgb, f64>>,

    /// Callback when color changes
    #[props(default)]
    pub on_color_change: Callback<Hsv<encoding::Srgb, f64>>,

    /// Whether the color picker is disabled
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// Additional attributes to extend the color picker element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the color picker element
    pub children: Element,
}

/// # ColorPicker
///
/// The [`ColorPicker`] component provides the color picker context and
/// synchronizes a color value between multiple color components.
///
/// ## Example
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::color_picker::*;
/// #[component]
/// fn Demo() -> Element {
///    use palette::{IntoColor, encoding};
///    let mut color = use_signal(|| -> palette::Hsv<encoding::Srgb, f64> {
///        Color::new(155, 128, 255).into_format::<f64>().into_color()
///    });
///    rsx! {
///            ColorPicker {
///                color: color(),
///                on_color_change: move |c| {
///                    tracing::info!("Color changed: {:?}", c);
///                    color.set(c);
///                },
///                ColorArea {
///                    AreaTrack {
///                        AreaThumb {
///                            AreaThumbSaturationInput {}
///                            AreaThumbValueInput {}
///                        }
///                    }
///                }
///            }
///    }
///}
/// ```
///
/// # Styling
///
/// The [`ColorPicker`] component defines the following data attributes you can use to control styling:
/// - `data-disabled`: Indicates if the ColorPicker is disabled. Possible values are `true` or `false`.
#[component]
pub fn ColorPicker(props: ColorPickerProps) -> Element {
    use_context_provider(|| ColorPickerContext {
        color: props.color,
        on_color_change: props.on_color_change,
    });

    rsx! {
        div {
            role: "group",
            aria_label: "Color picker",
            "data-disabled": (props.disabled)(),
            ..props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`ColorArea`] component.
#[derive(Props, Clone, PartialEq)]
pub struct ColorAreaProps {
    /// The step value
    #[props(default = 1.0)]
    pub step: ReadSignal<f64>,

    /// Additional attributes to extend the color area element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the color area element
    pub children: Element,
}

/// # ColorArea
///
/// The [`ColorArea`] allows users to adjust two channels of color value against a two-dimensional gradient background.
/// Compose it with [`AreaTrack`] and [`AreaThumb`] inside a [`ColorPicker`].
///
/// ## Example
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::color_picker::*;
/// #[component]
/// fn Demo() -> Element {
///    use palette::{IntoColor, encoding};
///    let mut color = use_signal(|| -> palette::Hsv<encoding::Srgb, f64> {
///        Color::new(155, 128, 255).into_format::<f64>().into_color()
///    });
///    rsx! {
///            ColorPicker {
///                color: color(),
///                on_color_change: move |c| {
///                    tracing::info!("Color changed: {:?}", c);
///                    color.set(c);
///                },
///                ColorArea {
///                    AreaTrack {
///                        AreaThumb {
///                            AreaThumbSaturationInput {}
///                            AreaThumbValueInput {}
///                        }
///                    }
///                }
///            }
///    }
///}
/// ```
#[component]
pub fn ColorArea(props: ColorAreaProps) -> Element {
    let picker_ctx = use_context::<ColorPickerContext>();
    let mut dragging = use_signal(|| false);

    // Thumb position is read straight from HSV state so saturation is preserved
    // at brightness=0.
    let value = use_memo(move || area_value_from_hsv(picker_ctx.color()));

    let area_ctx = use_context_provider(|| ColorAreaContext {
        value,
        step: props.step,
        dragging: dragging.into(),
    });

    let mut movement = use_move_interaction(dragging);
    let mut granular_value = use_hook(|| CopyValue::new(value()));

    let size = movement.rect().map(|r| r.size);

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

        let d_s = move_event.delta_x / size.width * COLOR_AREA_RANGE;
        let d_h = move_event.delta_y / size.height * COLOR_AREA_RANGE;

        let new_value = granular_value() + Size2D::new(d_s, -d_h);
        granular_value.set(new_value);
        set_area_value(picker_ctx, clamp_area_value(new_value, (area_ctx.step)()));
    });

    rsx! {
        div {
            role: "group",
            onmounted: move |e| async move {
                let mut movement = movement;
                movement.set_mounted(e.data()).await;
            },
            onresize: move |_| async move {
                let mut movement = movement;
                movement.refresh_rect().await;
            },
            onpointerdown: move |e| {
                if !movement.start_pointer(&e) {
                    return;
                }

                // Handle pointer interaction
                spawn(async move {
                    let mut movement = movement;

                    // Update the bounding rect of the slider in case it moved
                    if let Some(r) = movement.refresh_rect().await {
                        let size = r.size;

                        // Get the mouse position relative to the slider
                        let top_left = r.origin;
                        let relative_pos = e.client_coordinates() - top_left.cast_unit();

                        let x = (relative_pos.x / size.width) * COLOR_AREA_RANGE;
                        let y = COLOR_AREA_MAX - ((relative_pos.y / size.height) * COLOR_AREA_RANGE);
                        let pt = ClientPoint::new(x, y);
                        granular_value.set(pt);
                        set_area_value(picker_ctx, clamp_area_value(pt, (area_ctx.step)()));
                    }

                    dragging.set(true);
                });
            },
            ..props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`AreaTrack`] component
#[derive(Props, Clone, PartialEq)]
pub struct AreaTrackProps {
    /// Additional attributes to apply to the track element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The children of the track which should include a [`AreaThumb`]
    pub children: Element,
}

/// # AreaTrack
///
/// The track component for [`ColorArea`]. It renders the color plane background
/// and should contain an [`AreaThumb`].
///
/// This must be used inside a [`ColorArea`] component.
#[component]
pub fn AreaTrack(props: AreaTrackProps) -> Element {
    let picker_ctx = use_context::<ColorPickerContext>();
    let area_color = color_hex(
        Srgb::<f64>::from_color(Hsv::<encoding::Srgb, f64>::new(
            picker_ctx.color().hue,
            1.0,
            1.0,
        ))
        .into_format(),
    );

    rsx! {
        div {
            style: "--area-color: {area_color}",
            ..props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`AreaThumb`] component
#[derive(Props, Clone, PartialEq)]
pub struct AreaThumbProps {
    /// Additional attributes to apply to the thumb element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The children of the thumb element
    pub children: Element,
}

/// The props for the [`AreaThumbSaturationInput`] component.
#[derive(Props, Clone, PartialEq)]
pub struct AreaThumbSaturationInputProps {
    /// Additional attributes to apply to the saturation input element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// The props for the [`AreaThumbValueInput`] component.
#[derive(Props, Clone, PartialEq)]
pub struct AreaThumbValueInputProps {
    /// Additional attributes to apply to the value input element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// # AreaThumb
///
/// The thumb component for [`ColorArea`]. It supports mouse/touch interaction
/// through [`ColorArea`] and keyboard navigation with arrow keys.
///
/// This must be used inside a [`ColorArea`] component.
#[component]
pub fn AreaThumb(props: AreaThumbProps) -> Element {
    let picker_ctx = use_context::<ColorPickerContext>();
    let area_ctx = use_context::<ColorAreaContext>();

    let mut button_ref: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    let saturation_input_ref: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    let value_input_ref: Signal<Option<Rc<MountedData>>> = use_signal(|| None);

    let thumb_ctx = use_context_provider(|| AreaThumbContext {
        saturation_input_ref,
        value_input_ref,
    });

    use_effect(move || {
        if let Some(button) = button_ref() {
            let dragging = area_ctx.dragging.cloned();
            if dragging {
                spawn(async move {
                    _ = button.set_focus(true).await;
                });
            }
        }
    });

    let percent = area_percent((area_ctx.value)());
    let style = format!(
        "left: {:.2}%; top: {:.2}%;",
        percent.width,
        100. - percent.height
    );
    let thumb_color = color_hex(Srgb::<f64>::from_color(picker_ctx.color()).into_format());

    rsx! {
        div {
            aria_label: "Color area",
            "data-dragging": area_ctx.dragging,
            style,
            background_color: thumb_color,
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
            // First arrow press from the wrapper applies the step and hands
            // focus to the matching axis input so AT announces the channel.
            onkeydown: move |evt: Event<KeyboardData>| async move {
                let Some(move_event) = MoveEvent::from_keyboard(&evt, (area_ctx.step)()) else {
                    return;
                };
                evt.prevent_default();

                let new_value =
                    (area_ctx.value)() + Size2D::new(move_event.delta_x, move_event.delta_y);
                set_area_value(picker_ctx, clamp_area_value(new_value, (area_ctx.step)()));

                let target = if move_event.delta_x != 0.0 {
                    (thumb_ctx.saturation_input_ref)()
                } else {
                    (thumb_ctx.value_input_ref)()
                };
                if let Some(target) = target {
                    _ = target.set_focus(true).await;
                }
            },
            ..props.attributes,
            {props.children}
        }
    }
}

/// The saturation axis input for [`AreaThumb`].
#[component]
pub fn AreaThumbSaturationInput(props: AreaThumbSaturationInputProps) -> Element {
    let picker_ctx = use_context::<ColorPickerContext>();
    let area_ctx = use_context::<ColorAreaContext>();
    let mut thumb_ctx = use_context::<AreaThumbContext>();

    let percent = area_percent((area_ctx.value)());
    let current = (area_ctx.value)();
    let min = COLOR_AREA_MIN;
    let max = COLOR_AREA_MAX;
    let step = (area_ctx.step)();
    let color_label = color_name(Srgb::<f64>::from_color(picker_ctx.color()).into_format());

    rsx! {
        input {
            r#type: "range",
            aria_label: "Saturation",
            aria_roledescription: "2D Slider",
            aria_valuetext: format!("Saturation {:.0}%, {color_label}", percent.width),
            aria_orientation: "horizontal",
            tabindex: "-1",
            min: "{min}",
            max: "{max}",
            step: "{step}",
            value: format!("{}", current.x),
            onmounted: move |evt| {
                thumb_ctx.saturation_input_ref.set(Some(evt.data()));
            },
            // Cross-axis arrows hand focus to the value input so AT
            // announces the new channel.
            onkeydown: move |evt: Event<KeyboardData>| async move {
                let Some(move_event) = MoveEvent::from_keyboard(&evt, (area_ctx.step)()) else {
                    return;
                };
                evt.prevent_default();

                let new_value =
                    (area_ctx.value)() + Size2D::new(move_event.delta_x, move_event.delta_y);
                set_area_value(picker_ctx, clamp_area_value(new_value, (area_ctx.step)()));

                if move_event.delta_y != 0.0 {
                    if let Some(target) = (thumb_ctx.value_input_ref)() {
                        _ = target.set_focus(true).await;
                    }
                }
            },
            // Voice-control / direct-manipulation: a programmatic value
            // change on the input feeds the new saturation through.
            oninput: move |evt: Event<FormData>| {
                if let Ok(s) = evt.value().parse::<f64>() {
                    let v = picker_ctx.color().value;
                    let scaled = s.clamp(COLOR_AREA_MIN, COLOR_AREA_MAX) / COLOR_AREA_RANGE;
                    picker_ctx.set_sv(scaled, v);
                }
            },
            ..props.attributes,
        }
    }
}

/// The value axis input for [`AreaThumb`].
#[component]
pub fn AreaThumbValueInput(props: AreaThumbValueInputProps) -> Element {
    let picker_ctx = use_context::<ColorPickerContext>();
    let area_ctx = use_context::<ColorAreaContext>();
    let mut thumb_ctx = use_context::<AreaThumbContext>();

    let percent = area_percent((area_ctx.value)());
    let current = (area_ctx.value)();
    let min = COLOR_AREA_MIN;
    let max = COLOR_AREA_MAX;
    let step = (area_ctx.step)();
    let color_label = color_name(Srgb::<f64>::from_color(picker_ctx.color()).into_format());

    rsx! {
        input {
            r#type: "range",
            aria_label: "Value",
            aria_roledescription: "2D Slider",
            aria_valuetext: format!("Value {:.0}%, {color_label}", percent.height),
            aria_orientation: "vertical",
            tabindex: "-1",
            min: "{min}",
            max: "{max}",
            step: "{step}",
            value: format!("{}", current.y),
            onmounted: move |evt| {
                thumb_ctx.value_input_ref.set(Some(evt.data()));
            },
            onkeydown: move |evt: Event<KeyboardData>| async move {
                let Some(move_event) = MoveEvent::from_keyboard(&evt, (area_ctx.step)()) else {
                    return;
                };
                evt.prevent_default();

                let new_value =
                    (area_ctx.value)() + Size2D::new(move_event.delta_x, move_event.delta_y);
                set_area_value(picker_ctx, clamp_area_value(new_value, (area_ctx.step)()));

                if move_event.delta_x != 0.0 {
                    if let Some(target) = (thumb_ctx.saturation_input_ref)() {
                        _ = target.set_focus(true).await;
                    }
                }
            },
            oninput: move |evt: Event<FormData>| {
                if let Ok(v) = evt.value().parse::<f64>() {
                    let s = picker_ctx.color().saturation;
                    let scaled = v.clamp(COLOR_AREA_MIN, COLOR_AREA_MAX) / COLOR_AREA_RANGE;
                    picker_ctx.set_sv(s, scaled);
                }
            },
            ..props.attributes,
        }
    }
}

#[derive(Copy, Clone)]
struct ColorAreaContext {
    value: Memo<ClientPoint>,
    step: ReadSignal<f64>,
    dragging: ReadSignal<bool>,
}

#[derive(Clone, Copy)]
struct AreaThumbContext {
    saturation_input_ref: Signal<Option<Rc<MountedData>>>,
    value_input_ref: Signal<Option<Rc<MountedData>>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[component]
    fn ColorAreaWithZeroChildThumb() -> Element {
        rsx! {
            ColorPicker {
                color: Hsv::<encoding::Srgb, f64>::new(RgbHue::new(155.0), 0.5, 0.75),
                ColorArea {
                    AreaTrack {
                        AreaThumb {}
                    }
                }
            }
        }
    }

    #[component]
    fn ColorAreaWithAccessibleThumbInputs() -> Element {
        rsx! {
            ColorPicker {
                color: Hsv::<encoding::Srgb, f64>::new(RgbHue::new(155.0), 0.5, 0.75),
                ColorArea {
                    AreaTrack {
                        AreaThumb {
                            AreaThumbSaturationInput {
                                class: "custom-saturation",
                            }
                            AreaThumbValueInput {
                                class: "custom-value",
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn area_thumb_allows_zero_children() {
        let mut dom = VirtualDom::new(ColorAreaWithZeroChildThumb);
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);

        assert_eq!(html.matches("type=\"range\"").count(), 0);
    }

    #[test]
    fn area_thumb_preserves_explicit_axis_input_slots() {
        let mut dom = VirtualDom::new(ColorAreaWithAccessibleThumbInputs);
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);

        assert_eq!(html.matches("type=\"range\"").count(), 2);
        assert!(html.contains("custom-saturation"));
        assert!(html.contains("custom-value"));
    }
}
