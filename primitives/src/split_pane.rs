//! Unstyled split-pane primitives for resizable pane layouts.

use crate::move_interaction::{use_move_interaction, MoveEvent};
use dioxus::prelude::*;

const DEFAULT_STEP: f64 = 10.0;
const DEFAULT_SNAP_TOLERANCE: f64 = 0.0;
const DEFAULT_DIVIDER_SIZE: f64 = 0.0;
const SNAP_RELEASE_DISTANCE: f64 = 25.0;
const UNKNOWN_CONTAINER_SIZE: f64 = 1000.0;

/// The orientation of a [`SplitPane`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SplitPaneDirection {
    /// Panes are side-by-side and dividers move left and right.
    #[default]
    Horizontal,
    /// Panes are stacked and dividers move up and down.
    Vertical,
}

impl SplitPaneDirection {
    fn orientation(self) -> &'static str {
        match self {
            Self::Horizontal => "vertical",
            Self::Vertical => "horizontal",
        }
    }

    fn data_orientation(self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }

    fn flex_direction(self) -> &'static str {
        match self {
            Self::Horizontal => "row",
            Self::Vertical => "column",
        }
    }

    fn delta(self, movement: MoveEvent) -> f64 {
        match self {
            Self::Horizontal => movement.delta_x,
            Self::Vertical => movement.delta_y,
        }
    }

    fn keyboard_delta(self, movement: MoveEvent) -> f64 {
        match self {
            Self::Horizontal => movement.delta_x,
            Self::Vertical => -movement.delta_y,
        }
    }
}

/// A pane size or constraint value.
#[derive(Clone, Debug, PartialEq)]
pub enum SplitPaneSize {
    /// A CSS-pixel value.
    Px(f64),
    /// A percentage of the available pane space.
    Percent(f64),
}

impl SplitPaneSize {
    /// Create a CSS-pixel split pane size.
    pub fn px(value: f64) -> Self {
        Self::Px(value)
    }

    /// Create a percentage split pane size.
    pub fn percent(value: f64) -> Self {
        Self::Percent(value)
    }

    fn to_px(&self, available: f64) -> f64 {
        match *self {
            Self::Px(px) => px.max(0.0),
            Self::Percent(percent) => (available * percent / 100.0).max(0.0),
        }
    }

    fn as_css(&self) -> String {
        match *self {
            Self::Px(px) => format!("{px}px"),
            Self::Percent(percent) => format!("{percent}%"),
        }
    }
}

impl From<f64> for SplitPaneSize {
    fn from(value: f64) -> Self {
        Self::Px(value)
    }
}

impl From<i32> for SplitPaneSize {
    fn from(value: i32) -> Self {
        Self::Px(value as f64)
    }
}

impl From<&str> for SplitPaneSize {
    fn from(value: &str) -> Self {
        parse_size(value).unwrap_or(Self::Px(0.0))
    }
}

impl From<String> for SplitPaneSize {
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}

/// Browser storage location used by [`use_split_pane_persistence`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SplitPaneStorage {
    /// Persist sizes to `window.localStorage`.
    #[default]
    Local,
    /// Persist sizes to `window.sessionStorage`.
    Session,
}

impl SplitPaneStorage {
    fn js_name(self) -> &'static str {
        match self {
            Self::Local => "localStorage",
            Self::Session => "sessionStorage",
        }
    }
}

/// The source of a split pane resize.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SplitPaneResizeSource {
    /// Resize originated from a pointer drag.
    Pointer,
    /// Resize originated from keyboard input.
    Keyboard,
}

/// Resize callback payload for [`SplitPane`].
#[derive(Clone, Debug, PartialEq)]
pub struct SplitPaneResizeEvent {
    /// Pane sizes after constraints and snapping have been applied.
    ///
    /// During `on_resize`, these are the proposed next sizes. Controlled panes continue to render
    /// from their `size` prop until the owner commits those values.
    pub sizes: Vec<SplitPaneSize>,
    /// Divider index that initiated the resize.
    pub divider_index: usize,
    /// Interaction source that initiated the resize.
    pub source: SplitPaneResizeSource,
    /// Pointer type for pointer-originated resizes, such as `mouse`, `touch`, or `pen`.
    pub pointer_type: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
struct PaneConfig {
    size: Option<SplitPaneSize>,
    default_size: Option<SplitPaneSize>,
    min_size: Option<SplitPaneSize>,
    max_size: Option<SplitPaneSize>,
}

#[derive(Clone, Copy)]
struct SplitPaneContext {
    direction: SplitPaneDirection,
    resizable: ReadSignal<bool>,
    step: ReadSignal<f64>,
    snap_points: ReadSignal<Vec<SplitPaneSize>>,
    snap_tolerance: ReadSignal<f64>,
    divider_size: ReadSignal<f64>,
    divider_class: ReadSignal<Option<String>>,
    divider_style: ReadSignal<Option<String>>,
    panes: Signal<Vec<PaneConfig>>,
    sizes: Signal<Vec<f64>>,
    container_size: Signal<Option<f64>>,
    pane_cursor: Signal<usize>,
    divider_cursor: Signal<usize>,
    dragging_divider: Signal<Option<usize>>,
    pointer_type: Signal<Option<String>>,
    snapped_before: Signal<Option<f64>>,
    keyboard_start: CopyValue<Option<KeyboardResizeState>>,
    on_resize_start: Callback<SplitPaneResizeEvent>,
    on_resize: Callback<SplitPaneResizeEvent>,
    on_resize_end: Callback<SplitPaneResizeEvent>,
}

#[derive(Clone, Debug, PartialEq)]
struct KeyboardResizeState {
    stored_sizes: Option<Vec<f64>>,
    snapped_before: Option<f64>,
}

#[derive(Clone, Debug, PartialEq)]
struct KeyboardEscapeRestore {
    stored_sizes: Option<Vec<f64>>,
    callback_sizes: Vec<f64>,
    snapped_before: Option<f64>,
}

#[derive(Clone, Copy)]
struct ResizePairRequest<'a> {
    sizes: &'a [f64],
    panes: &'a [PaneConfig],
    divider_index: usize,
    delta: f64,
    available: f64,
    snap_points: &'a [SplitPaneSize],
    snap_tolerance: f64,
    snapped_before: Option<f64>,
}

impl SplitPaneContext {
    fn available_size(&self) -> f64 {
        let container = (self.container_size)().unwrap_or(UNKNOWN_CONTAINER_SIZE);
        available_pane_space(container, self.panes.read().len(), (self.divider_size)())
    }

    fn effective_sizes(&self) -> Vec<f64> {
        let available = self.available_size();
        let panes = self.panes.read().clone();
        let current = self.sizes.read().clone();
        effective_sizes_from_state(&panes, &current, available)
    }

    fn event(
        &self,
        divider_index: usize,
        source: SplitPaneResizeSource,
        sizes: &[f64],
        pointer_type: Option<String>,
    ) -> SplitPaneResizeEvent {
        SplitPaneResizeEvent {
            sizes: resize_event_sizes(sizes),
            divider_index,
            source,
            pointer_type,
        }
    }

    fn resize_pair(&mut self, divider_index: usize, delta: f64, source: SplitPaneResizeSource) {
        if !(self.resizable)() {
            return;
        }

        let available = self.available_size();
        let panes = self.panes.read().clone();
        let current = self.ensure_sizes();
        let current_snap = (self.snapped_before)();
        let (next, next_snap) = resize_pair(ResizePairRequest {
            sizes: &current,
            panes: &panes,
            divider_index,
            delta,
            available,
            snap_points: &(self.snap_points)(),
            snap_tolerance: (self.snap_tolerance)(),
            snapped_before: current_snap,
        });

        if next != current {
            self.sizes.set(next.clone());
            self.snapped_before.set(next_snap);
            self.on_resize.call(self.event(
                divider_index,
                source,
                &next,
                self.pointer_type.read().clone(),
            ));
        } else if next_snap != current_snap {
            self.snapped_before.set(next_snap);
        }
    }

    fn set_pair_edge(
        &mut self,
        divider_index: usize,
        edge: PairEdge,
        source: SplitPaneResizeSource,
    ) {
        let available = self.available_size();
        let panes = self.panes.read().clone();
        let current = self.ensure_sizes();
        let next = resize_pair_to_edge(&current, &panes, divider_index, edge, available);

        if next != current {
            self.sizes.set(next.clone());
            self.snapped_before.set(None);
            self.on_resize.call(self.event(
                divider_index,
                source,
                &next,
                self.pointer_type.read().clone(),
            ));
        }
    }

    fn start_keyboard_resize(&mut self, divider_index: usize) {
        if self.keyboard_start.read().is_none() {
            let start = self.ensure_sizes();
            let stored_sizes =
                committed_sizes(self.sizes.read().as_slice()).map(|sizes| sizes.to_vec());
            self.keyboard_start.set(Some(KeyboardResizeState {
                stored_sizes,
                snapped_before: (self.snapped_before)(),
            }));
            self.on_resize_start.call(self.event(
                divider_index,
                SplitPaneResizeSource::Keyboard,
                &start,
                None,
            ));
        }
    }

    fn ensure_sizes(&mut self) -> Vec<f64> {
        self.effective_sizes()
    }
}

fn resize_event_sizes(sizes: &[f64]) -> Vec<SplitPaneSize> {
    sizes.iter().copied().map(SplitPaneSize::Px).collect()
}

fn data_bool(value: bool) -> &'static str {
    if value {
        "true"
    } else {
        "false"
    }
}

fn accessibility_number(value: f64) -> String {
    if value.fract().abs() < 0.001 {
        format!("{value:.0}")
    } else {
        value.to_string()
    }
}

fn data_index(value: usize) -> String {
    value.to_string()
}

/// Props for [`SplitPane`].
#[derive(Props, Clone, PartialEq)]
pub struct SplitPaneProps {
    /// Layout direction for the panes.
    #[props(default)]
    pub direction: SplitPaneDirection,
    /// Whether divider resizing is enabled.
    #[props(default = true)]
    pub resizable: ReadSignal<bool>,
    /// Snap points applied during pointer and keyboard resize.
    #[props(default)]
    pub snap_points: ReadSignal<Vec<SplitPaneSize>>,
    /// Distance in CSS pixels from a snap point before the divider snaps.
    ///
    /// After snapping, the divider stays pinned until the active drag or keyboard interaction
    /// moves about 25px past the snap point.
    #[props(default = DEFAULT_SNAP_TOLERANCE)]
    pub snap_tolerance: ReadSignal<f64>,
    /// Keyboard arrow-key resize step in CSS pixels.
    #[props(default = DEFAULT_STEP)]
    pub step: ReadSignal<f64>,
    /// Divider size in CSS pixels. This is used when calculating available pane space.
    #[props(default = DEFAULT_DIVIDER_SIZE)]
    pub divider_size: ReadSignal<f64>,
    /// Class applied to every [`SplitPaneDivider`] when that divider does not provide one.
    #[props(default)]
    pub divider_class: ReadSignal<Option<String>>,
    /// Style applied to every [`SplitPaneDivider`] before divider-local styles.
    #[props(default)]
    pub divider_style: ReadSignal<Option<String>>,
    /// Called when a pointer or keyboard resize interaction starts.
    #[props(default)]
    pub on_resize_start: Callback<SplitPaneResizeEvent>,
    /// Called with proposed pane sizes whenever a resize interaction changes the layout.
    ///
    /// Controlled panes remain sourced from their `size` props; commit this event's sizes from the
    /// owner to accept the proposed resize.
    #[props(default)]
    pub on_resize: Callback<SplitPaneResizeEvent>,
    /// Called when a pointer or keyboard resize interaction ends.
    #[props(default)]
    pub on_resize_end: Callback<SplitPaneResizeEvent>,
    /// Additional attributes to apply to the root element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// Panes and dividers rendered inside the split pane.
    pub children: Element,
}

/// A split-pane root that coordinates pane layout, resizing, constraints, and accessibility.
///
/// Render two or more [`Pane`] children and place a [`SplitPaneDivider`] between each adjacent
/// pair. The root fills its parent (`width: 100%; height: 100%`) and expects the parent to have
/// an explicit size.
#[component]
pub fn SplitPane(props: SplitPaneProps) -> Element {
    let panes = use_signal(Vec::<PaneConfig>::new);
    let sizes = use_signal(Vec::<f64>::new);
    let mut container_size = use_signal(|| None::<f64>);
    let mut pane_cursor = use_signal(|| 0usize);
    let mut divider_cursor = use_signal(|| 0usize);
    let mut root_ref: Signal<Option<std::rc::Rc<MountedData>>> = use_signal(|| None);
    let dragging_divider = use_signal(|| None::<usize>);
    let pointer_type = use_signal(|| None::<String>);
    let snapped_before = use_signal(|| None::<f64>);
    let keyboard_start = use_hook(|| CopyValue::new(None::<KeyboardResizeState>));

    pane_cursor.set(0);
    divider_cursor.set(0);

    let ctx = SplitPaneContext {
        direction: props.direction,
        resizable: props.resizable,
        step: props.step,
        snap_points: props.snap_points,
        snap_tolerance: props.snap_tolerance,
        divider_size: props.divider_size,
        divider_class: props.divider_class,
        divider_style: props.divider_style,
        panes,
        sizes,
        container_size,
        pane_cursor,
        divider_cursor,
        dragging_divider,
        pointer_type,
        snapped_before,
        keyboard_start,
        on_resize_start: props.on_resize_start,
        on_resize: props.on_resize,
        on_resize_end: props.on_resize_end,
    };

    use_context_provider(|| ctx);

    use_effect(move || {
        let mut ctx = ctx;
        ctx.ensure_sizes();
    });

    let base_style = format!(
        "display:flex;flex-direction:{};width:100%;height:100%;",
        props.direction.flex_direction()
    );

    rsx! {
        div {
            role: "group",
            "data-orientation": props.direction.data_orientation(),
            "data-resizable": data_bool((props.resizable)()),
            style: base_style,
            onmounted: move |evt| async move {
                let mounted = evt.data();
                root_ref.set(Some(mounted.clone()));
                let Ok(rect) = mounted.get_client_rect().await else {
                    return;
                };
                let size = match props.direction {
                    SplitPaneDirection::Horizontal => rect.width(),
                    SplitPaneDirection::Vertical => rect.height(),
                };
                container_size.set(Some(size));
            },
            onresize: move |_| async move {
                let Some(mounted) = root_ref() else {
                    return;
                };
                let Ok(rect) = mounted.get_client_rect().await else {
                    return;
                };
                let size = match props.direction {
                    SplitPaneDirection::Horizontal => rect.width(),
                    SplitPaneDirection::Vertical => rect.height(),
                };
                container_size.set(Some(size));
            },
            ..props.attributes,
            {props.children}
        }
    }
}

/// Props for [`Pane`].
#[derive(Props, Clone, PartialEq)]
pub struct PaneProps {
    /// Controlled pane size. When present, this value is the source of truth for the pane.
    #[props(default)]
    pub size: ReadSignal<Option<SplitPaneSize>>,
    /// Initial pane size used when uncontrolled.
    #[props(default)]
    pub default_size: ReadSignal<Option<SplitPaneSize>>,
    /// Minimum pane size.
    #[props(default)]
    pub min_size: ReadSignal<Option<SplitPaneSize>>,
    /// Maximum pane size.
    #[props(default)]
    pub max_size: ReadSignal<Option<SplitPaneSize>>,
    /// Additional attributes to apply to the pane element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// Pane contents.
    pub children: Element,
}

/// A split-pane child pane.
#[component]
pub fn Pane(props: PaneProps) -> Element {
    let mut ctx = use_context::<SplitPaneContext>();
    let index = use_hook({
        let mut cursor = ctx.pane_cursor;
        move || {
            let index = cursor();
            cursor.set(index + 1);
            index
        }
    });

    use_effect(move || {
        let config = PaneConfig {
            size: (props.size)(),
            default_size: (props.default_size)(),
            min_size: (props.min_size)(),
            max_size: (props.max_size)(),
        };
        let mut panes = ctx.panes.write();
        if panes.len() <= index {
            panes.resize(index + 1, config.clone());
        }
        panes[index] = config;
    });

    let style = use_memo(move || {
        let available = ctx.available_size();
        let size = ctx
            .effective_sizes()
            .get(index)
            .copied()
            .unwrap_or_else(|| {
                (props.default_size)()
                    .map(|s| s.to_px(available))
                    .unwrap_or(0.0)
            });
        format!("flex:0 0 {size}px;min-width:0;min-height:0;overflow:auto;")
    });

    rsx! {
        div {
            "data-pane-index": data_index(index),
            style,
            ..props.attributes,
            {props.children}
        }
    }
}

/// Props for [`SplitPaneDivider`].
#[derive(Props, Clone, PartialEq)]
pub struct SplitPaneDividerProps {
    /// Divider index. Defaults to render order among dividers.
    #[props(default)]
    pub index: Option<usize>,
    /// Optional class for the divider.
    #[props(default)]
    pub class: ReadSignal<Option<String>>,
    /// Optional inline style for the divider.
    #[props(default)]
    pub style: ReadSignal<Option<String>>,
    /// Custom divider contents.
    #[props(default)]
    pub divider: Option<Element>,
    /// Additional attributes to apply to the divider element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// Children rendered inside the divider when `divider` is not supplied.
    pub children: Element,
}

/// A focusable separator that resizes the panes on either side.
#[component]
pub fn SplitPaneDivider(props: SplitPaneDividerProps) -> Element {
    let mut ctx = use_context::<SplitPaneContext>();
    let auto_index = use_hook({
        let mut cursor = ctx.divider_cursor;
        move || {
            let index = cursor();
            cursor.set(index + 1);
            index
        }
    });
    let index = props.index.unwrap_or(auto_index);
    let mut dragging = use_signal(|| false);
    let mut movement = use_move_interaction(dragging);

    use_effect(move || {
        if !dragging() {
            return;
        }

        let Some(move_event) = movement.pointer_move() else {
            if (ctx.dragging_divider)() == Some(index) {
                ctx.dragging_divider.set(None);
                ctx.snapped_before.set(None);
                let callback_sizes = ctx.effective_sizes();
                let pointer_type = ctx.pointer_type.take();
                ctx.on_resize_end.call(ctx.event(
                    index,
                    SplitPaneResizeSource::Pointer,
                    &callback_sizes,
                    pointer_type,
                ));
            }
            return;
        };

        ctx.resize_pair(
            index,
            ctx.direction.delta(move_event),
            SplitPaneResizeSource::Pointer,
        );
    });

    let bounds = use_memo(move || {
        let sizes = ctx.ensure_sizes();
        let panes = ctx.panes.read().clone();
        let available = ctx.available_size();
        divider_bounds(&sizes, &panes, index, available)
    });

    let (value, min, max) = bounds();
    let root_style = divider_style(
        ctx.direction,
        (ctx.divider_size)(),
        (ctx.divider_style)(),
        (props.style)(),
    );
    let class = (props.class)().or_else(|| (ctx.divider_class)());
    let child = props
        .divider
        .clone()
        .unwrap_or_else(|| props.children.clone());

    rsx! {
        div {
            role: "separator",
            tabindex: if (ctx.resizable)() { 0 } else { -1 },
            aria_orientation: ctx.direction.orientation(),
            aria_valuemin: accessibility_number(min),
            aria_valuemax: accessibility_number(max),
            aria_valuenow: accessibility_number(value),
            "data-orientation": ctx.direction.data_orientation(),
            "data-resizable": data_bool((ctx.resizable)()),
            "data-dragging": data_bool(dragging()),
            "data-divider-index": data_index(index),
            class,
            style: root_style,
            onmounted: move |evt| async move {
                movement.set_mounted(evt.data()).await;
            },
            onresize: move |_| async move {
                movement.refresh_rect().await;
            },
            onpointerdown: move |evt| {
                if !(ctx.resizable)() || !movement.start_pointer(&evt) {
                    return;
                }
                let mut ctx = ctx;
                let pointer_type = evt.pointer_type();
                dragging.set(true);
                ctx.dragging_divider.set(Some(index));
                ctx.pointer_type.set(Some(pointer_type.clone()));
                ctx.snapped_before.set(None);
                spawn(async move {
                    movement.refresh_rect().await;
                    let callback_sizes = ctx.effective_sizes();
                    ctx.on_resize_start
                        .call(ctx.event(
                            index,
                            SplitPaneResizeSource::Pointer,
                            &callback_sizes,
                            Some(pointer_type),
                        ));
                });
            },
            onkeydown: move |evt| {
                if !(ctx.resizable)() {
                    return;
                }

                match evt.data().key() {
                    Key::Home => {
                        evt.prevent_default();
                        ctx.start_keyboard_resize(index);
                        ctx.set_pair_edge(index, PairEdge::MinBefore, SplitPaneResizeSource::Keyboard);
                    }
                    Key::End => {
                        evt.prevent_default();
                        ctx.start_keyboard_resize(index);
                        ctx.set_pair_edge(index, PairEdge::MaxBefore, SplitPaneResizeSource::Keyboard);
                    }
                    Key::Escape => {
                        evt.prevent_default();
                        if let Some(start) = ctx.keyboard_start.take() {
                            let restored = restore_keyboard_resize(
                                start,
                                &ctx.panes.read(),
                                ctx.available_size(),
                            );
                            match restored.stored_sizes {
                                Some(stored_sizes) => ctx.sizes.set(stored_sizes),
                                None => ctx.sizes.set(Vec::new()),
                            }
                            ctx.snapped_before.set(restored.snapped_before);
                            ctx.on_resize.call(ctx.event(
                                index,
                                SplitPaneResizeSource::Keyboard,
                                &restored.callback_sizes,
                                None,
                            ));
                            ctx.on_resize_end.call(ctx.event(
                                index,
                                SplitPaneResizeSource::Keyboard,
                                &restored.callback_sizes,
                                None,
                            ));
                        }
                    }
                    _ => {
                        let Some(move_event) = MoveEvent::from_keyboard(&evt, (ctx.step)()) else {
                            return;
                        };
                        evt.prevent_default();
                        ctx.start_keyboard_resize(index);
                        ctx.resize_pair(
                            index,
                            ctx.direction.keyboard_delta(move_event),
                            SplitPaneResizeSource::Keyboard,
                        );
                    }
                }
            },
            onblur: move |_| {
                if ctx.keyboard_start.read().is_some() {
                    ctx.keyboard_start.take();
                    ctx.snapped_before.set(None);
                    let callback_sizes = ctx.effective_sizes();
                    ctx.on_resize_end
                        .call(ctx.event(
                            index,
                            SplitPaneResizeSource::Keyboard,
                            &callback_sizes,
                            None,
                        ));
                }
            },
            ..props.attributes,
            {child}
        }
    }
}

/// Persist split-pane sizes in browser storage.
///
/// The returned signal can be passed to pane `size` or `default_size` values after converting the
/// vector entries for the panes you want to restore. The callback is suitable for
/// [`SplitPaneProps::on_resize_end`].
pub fn use_split_pane_persistence(
    key: impl Into<String>,
    storage: SplitPaneStorage,
) -> (
    Signal<Option<Vec<SplitPaneSize>>>,
    Callback<SplitPaneResizeEvent>,
) {
    let key = use_hook(|| key.into());
    let mut sizes = use_signal(|| None::<Vec<SplitPaneSize>>);

    use_effect({
        let key = key.clone();
        move || {
            let storage_key = key.clone();
            spawn(async move {
                let mut eval = document::eval(&format!(
                    "const value = window.{}?.getItem(await dioxus.recv()) ?? null; dioxus.send(value);",
                    storage.js_name()
                ));
                let _ = eval.send(storage_key);
                if let Ok(Some(value)) = eval.recv::<Option<String>>().await {
                    sizes.set(Some(decode_sizes(&value)));
                }
            });
        }
    });

    let on_resize_end = use_callback(move |event: SplitPaneResizeEvent| {
        let encoded = encode_sizes(&event.sizes);
        let storage_key = key.clone();
        spawn(async move {
            let eval = document::eval(&format!(
                "const [key, value] = await dioxus.recv(); window.{}?.setItem(key, value);",
                storage.js_name()
            ));
            let _ = eval.send((storage_key, encoded));
        });
    });

    (sizes, on_resize_end)
}

#[derive(Clone, Copy)]
enum PairEdge {
    MinBefore,
    MaxBefore,
}

fn parse_size(value: &str) -> Option<SplitPaneSize> {
    let trimmed = value.trim();
    if let Some(value) = trimmed.strip_suffix('%') {
        value.trim().parse::<f64>().ok().map(SplitPaneSize::Percent)
    } else if let Some(value) = trimmed.strip_suffix("px") {
        value.trim().parse::<f64>().ok().map(SplitPaneSize::Px)
    } else {
        trimmed.parse::<f64>().ok().map(SplitPaneSize::Px)
    }
}

fn available_pane_space(container: f64, pane_count: usize, divider_size: f64) -> f64 {
    let dividers = pane_count.saturating_sub(1) as f64;
    (container - dividers * divider_size.max(0.0)).max(0.0)
}

fn normalize_sizes(panes: &[PaneConfig], current: Option<&[f64]>, available: f64) -> Vec<f64> {
    if panes.is_empty() {
        return Vec::new();
    }

    let mut sizes = vec![0.0; panes.len()];
    let mut flexible = Vec::new();
    let mut fixed_total = 0.0;

    for (index, pane) in panes.iter().enumerate() {
        let current_size = current
            .and_then(|sizes| sizes.get(index))
            .copied()
            .map(SplitPaneSize::Px);
        let source = if let Some(size) = pane.size.as_ref() {
            Some(size)
        } else {
            current_size.as_ref().or(pane.default_size.as_ref())
        };

        if let Some(source) = source {
            let px = clamp_pane(source.to_px(available), pane, available);
            sizes[index] = px;
            fixed_total += px;
        } else {
            flexible.push(index);
        }
    }

    let remaining = (available - fixed_total).max(0.0);
    let fallback = if flexible.is_empty() {
        0.0
    } else {
        remaining / flexible.len() as f64
    };

    for index in flexible {
        sizes[index] = clamp_pane(fallback, &panes[index], available);
    }

    rebalance_to_available(&mut sizes, panes, available);
    sizes
}

fn committed_sizes(current: &[f64]) -> Option<&[f64]> {
    (!current.is_empty()).then_some(current)
}

fn effective_sizes_from_state(panes: &[PaneConfig], current: &[f64], available: f64) -> Vec<f64> {
    normalize_sizes(panes, committed_sizes(current), available)
}

fn clamp_pane(value: f64, pane: &PaneConfig, available: f64) -> f64 {
    let min = pane
        .min_size
        .as_ref()
        .map(|size| size.to_px(available))
        .unwrap_or(0.0);
    let max = pane
        .max_size
        .as_ref()
        .map(|size| size.to_px(available))
        .unwrap_or(available)
        .max(min);
    value.clamp(min, max)
}

fn rebalance_to_available(sizes: &mut [f64], panes: &[PaneConfig], available: f64) {
    let mut delta = available - sizes.iter().sum::<f64>();
    if delta.abs() < 0.001 {
        return;
    }

    for (index, pane) in panes.iter().enumerate().rev() {
        if delta.abs() < 0.001 {
            break;
        }
        let current = sizes[index];
        let target = clamp_pane(current + delta, pane, available);
        sizes[index] = target;
        delta -= target - current;
    }
}

fn resize_pair(request: ResizePairRequest<'_>) -> (Vec<f64>, Option<f64>) {
    let ResizePairRequest {
        sizes,
        panes,
        divider_index,
        delta,
        available,
        snap_points,
        snap_tolerance,
        snapped_before,
    } = request;

    if divider_index + 1 >= sizes.len() || divider_index + 1 >= panes.len() {
        return (sizes.to_vec(), None);
    }

    let before = sizes[divider_index];
    let after = sizes[divider_index + 1];
    let pair_total = before + after;
    let (raw_before, snapped_before) = apply_snap(
        before + delta,
        available,
        snap_points,
        snap_tolerance,
        snapped_before,
    );
    let min_after = pane_min(&panes[divider_index + 1], available);
    let max_after = pane_max(&panes[divider_index + 1], available);
    let min_before = pane_min(&panes[divider_index], available).max(pair_total - max_after);
    let max_before = pane_max(&panes[divider_index], available).min(pair_total - min_after);
    let before = raw_before.clamp(min_before, max_before.max(min_before));
    let after = pair_total - before;

    let mut next = sizes.to_vec();
    next[divider_index] = before;
    next[divider_index + 1] = clamp_pane(after, &panes[divider_index + 1], available);
    (
        next,
        snapped_before.filter(|snap| (*snap - before).abs() < 0.001),
    )
}

fn resize_pair_to_edge(
    sizes: &[f64],
    panes: &[PaneConfig],
    divider_index: usize,
    edge: PairEdge,
    available: f64,
) -> Vec<f64> {
    if divider_index + 1 >= sizes.len() || divider_index + 1 >= panes.len() {
        return sizes.to_vec();
    }

    let pair_total = sizes[divider_index] + sizes[divider_index + 1];
    let min_after = pane_min(&panes[divider_index + 1], available);
    let max_after = pane_max(&panes[divider_index + 1], available);
    let min_before = pane_min(&panes[divider_index], available).max(pair_total - max_after);
    let max_before = pane_max(&panes[divider_index], available).min(pair_total - min_after);
    let before = match edge {
        PairEdge::MinBefore => min_before,
        PairEdge::MaxBefore => max_before.max(min_before),
    };
    let mut next = sizes.to_vec();
    next[divider_index] = before;
    next[divider_index + 1] = pair_total - before;
    next
}

fn divider_bounds(
    sizes: &[f64],
    panes: &[PaneConfig],
    divider_index: usize,
    available: f64,
) -> (f64, f64, f64) {
    if divider_index + 1 >= sizes.len() || divider_index + 1 >= panes.len() {
        return (0.0, 0.0, 0.0);
    }

    let pair_total = sizes[divider_index] + sizes[divider_index + 1];
    let min_after = pane_min(&panes[divider_index + 1], available);
    let max_after = pane_max(&panes[divider_index + 1], available);
    let min = pane_min(&panes[divider_index], available).max(pair_total - max_after);
    let max = pane_max(&panes[divider_index], available).min(pair_total - min_after);
    (sizes[divider_index], min, max.max(min))
}

fn pane_min(pane: &PaneConfig, available: f64) -> f64 {
    pane.min_size
        .as_ref()
        .map(|size| size.to_px(available))
        .unwrap_or(0.0)
}

fn pane_max(pane: &PaneConfig, available: f64) -> f64 {
    pane.max_size
        .as_ref()
        .map(|size| size.to_px(available))
        .unwrap_or(available)
}

fn apply_snap(
    value: f64,
    available: f64,
    snap_points: &[SplitPaneSize],
    snap_tolerance: f64,
    snapped_before: Option<f64>,
) -> (f64, Option<f64>) {
    if let Some(snap) = snapped_before {
        if (value - snap).abs() <= SNAP_RELEASE_DISTANCE {
            return (snap, Some(snap));
        }
    }

    for point in snap_points {
        let snap = point.to_px(available);
        if snapped_before.is_some_and(|previous| (previous - snap).abs() < 0.001) {
            continue;
        }
        if (value - snap).abs() <= snap_tolerance {
            return (snap, Some(snap));
        }
    }
    (value, None)
}

fn restore_keyboard_resize(
    start: KeyboardResizeState,
    panes: &[PaneConfig],
    available: f64,
) -> KeyboardEscapeRestore {
    let callback_sizes = match start.stored_sizes.as_deref() {
        Some(stored_sizes) => effective_sizes_from_state(panes, stored_sizes, available),
        None => effective_sizes_from_state(panes, &[], available),
    };

    KeyboardEscapeRestore {
        stored_sizes: start.stored_sizes,
        callback_sizes,
        snapped_before: start.snapped_before,
    }
}

fn divider_style(
    direction: SplitPaneDirection,
    divider_size: f64,
    root_style: Option<String>,
    extra_style: Option<String>,
) -> String {
    let basis = divider_size.max(0.0);
    let cursor = match direction {
        SplitPaneDirection::Horizontal => "col-resize",
        SplitPaneDirection::Vertical => "row-resize",
    };
    let mut style = format!("flex:0 0 {basis}px;cursor:{cursor};touch-action:none;");
    if let Some(extra) = root_style {
        style.push_str(&extra);
    }
    if let Some(extra) = extra_style {
        style.push_str(&extra);
    }
    style
}

fn encode_sizes(sizes: &[SplitPaneSize]) -> String {
    sizes
        .iter()
        .map(SplitPaneSize::as_css)
        .collect::<Vec<_>>()
        .join("|")
}

fn decode_sizes(value: &str) -> Vec<SplitPaneSize> {
    value.split('|').filter_map(parse_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pane(
        default_size: Option<SplitPaneSize>,
        min_size: Option<SplitPaneSize>,
        max_size: Option<SplitPaneSize>,
    ) -> PaneConfig {
        PaneConfig {
            size: None,
            default_size,
            min_size,
            max_size,
        }
    }

    fn controlled_pane(
        size: Option<SplitPaneSize>,
        default_size: Option<SplitPaneSize>,
        min_size: Option<SplitPaneSize>,
        max_size: Option<SplitPaneSize>,
    ) -> PaneConfig {
        PaneConfig {
            size,
            default_size,
            min_size,
            max_size,
        }
    }

    #[test]
    fn available_pane_space_subtracts_divider_width_once_per_gap() {
        assert_eq!(available_pane_space(800.0, 3, 12.0), 776.0);
    }

    #[test]
    fn available_pane_space_clamps_negative_inputs_to_zero() {
        assert_eq!(available_pane_space(200.0, 2, -24.0), 200.0);
        assert_eq!(available_pane_space(-50.0, 2, 10.0), 0.0);
    }

    #[test]
    fn parses_numeric_percent_and_px_sizes() {
        assert_eq!(parse_size("25%"), Some(SplitPaneSize::Percent(25.0)));
        assert_eq!(parse_size("240px"), Some(SplitPaneSize::Px(240.0)));
        assert_eq!(parse_size("120"), Some(SplitPaneSize::Px(120.0)));
    }

    #[test]
    fn normalizes_unspecified_panes_equally() {
        let panes = vec![
            pane(None, None, None),
            pane(None, None, None),
            pane(None, None, None),
        ];

        assert_eq!(
            normalize_sizes(&panes, None, 900.0),
            vec![300.0, 300.0, 300.0]
        );
    }

    #[test]
    fn applies_default_size_and_constraints() {
        let panes = vec![
            pane(
                Some(SplitPaneSize::Percent(20.0)),
                Some(SplitPaneSize::Px(250.0)),
                None,
            ),
            pane(None, None, None),
        ];

        assert_eq!(normalize_sizes(&panes, None, 1000.0), vec![250.0, 750.0]);
    }

    #[test]
    fn normalize_sizes_keeps_controlled_panes_on_prop_values() {
        let panes = vec![
            controlled_pane(
                Some(SplitPaneSize::Percent(25.0)),
                Some(SplitPaneSize::Px(400.0)),
                None,
                None,
            ),
            pane(None, None, None),
        ];

        assert_eq!(
            normalize_sizes(&panes, Some(&[600.0, 400.0]), 1000.0),
            vec![250.0, 750.0]
        );
    }

    #[test]
    fn normalize_sizes_preserves_uncontrolled_current_sizes() {
        let panes = vec![
            pane(Some(SplitPaneSize::Px(100.0)), None, None),
            pane(None, None, None),
        ];

        assert_eq!(
            normalize_sizes(&panes, Some(&[420.0, 580.0]), 1000.0),
            vec![420.0, 580.0]
        );
    }

    #[test]
    fn effective_sizes_reapply_late_defaults_when_no_sizes_are_committed() {
        let initial = vec![pane(None, None, None), pane(None, None, None)];
        let restored = vec![
            pane(Some(SplitPaneSize::Percent(25.0)), None, None),
            pane(None, None, None),
        ];

        assert_eq!(
            effective_sizes_from_state(&initial, &[], 1000.0),
            vec![500.0, 500.0]
        );
        assert_eq!(
            effective_sizes_from_state(&restored, &[], 1000.0),
            vec![250.0, 750.0]
        );
    }

    #[test]
    fn effective_sizes_keep_percent_defaults_responsive_until_resize_is_committed() {
        let panes = vec![
            pane(Some(SplitPaneSize::Percent(25.0)), None, None),
            pane(None, None, None),
        ];

        assert_eq!(
            effective_sizes_from_state(&panes, &[], 1000.0),
            vec![250.0, 750.0]
        );
        assert_eq!(
            effective_sizes_from_state(&panes, &[], 1600.0),
            vec![400.0, 1200.0]
        );
        assert_eq!(
            effective_sizes_from_state(&panes, &[350.0, 650.0], 1600.0),
            vec![350.0, 1250.0]
        );
    }

    #[test]
    fn resize_pair_preserves_pair_total_and_clamps_neighbor_minimum() {
        let panes = vec![
            pane(None, Some(SplitPaneSize::Px(100.0)), None),
            pane(None, Some(SplitPaneSize::Px(250.0)), None),
        ];

        let (sizes, _) = resize_pair(ResizePairRequest {
            sizes: &[400.0, 600.0],
            panes: &panes,
            divider_index: 0,
            delta: 500.0,
            available: 1000.0,
            snap_points: &[],
            snap_tolerance: 0.0,
            snapped_before: None,
        });

        assert_eq!(sizes, vec![750.0, 250.0]);
    }

    #[test]
    fn resize_pair_returns_proposed_sizes_for_controlled_panes() {
        let panes = vec![
            controlled_pane(Some(SplitPaneSize::Percent(25.0)), None, None, None),
            pane(None, None, None),
        ];
        let current = normalize_sizes(&panes, Some(&[600.0, 400.0]), 1000.0);

        let (proposed, _) = resize_pair(ResizePairRequest {
            sizes: &current,
            panes: &panes,
            divider_index: 0,
            delta: 100.0,
            available: 1000.0,
            snap_points: &[],
            snap_tolerance: 0.0,
            snapped_before: None,
        });

        assert_eq!(current, vec![250.0, 750.0]);
        assert_eq!(proposed, vec![350.0, 650.0]);
        assert_eq!(
            normalize_sizes(&panes, Some(&proposed), 1000.0),
            vec![250.0, 750.0]
        );
    }

    #[test]
    fn resize_event_sizes_preserve_proposed_px_values() {
        assert_eq!(
            resize_event_sizes(&[350.0, 650.0]),
            vec![SplitPaneSize::Px(350.0), SplitPaneSize::Px(650.0)]
        );
    }

    #[test]
    fn snap_points_apply_within_tolerance() {
        let panes = vec![pane(None, None, None), pane(None, None, None)];

        let sizes = resize_pair(ResizePairRequest {
            sizes: &[400.0, 600.0],
            panes: &panes,
            divider_index: 0,
            delta: 94.0,
            available: 1000.0,
            snap_points: &[SplitPaneSize::Percent(50.0)],
            snap_tolerance: 10.0,
            snapped_before: None,
        });

        assert_eq!(sizes.0, vec![500.0, 500.0]);
        assert_eq!(sizes.1, Some(500.0));
    }

    #[test]
    fn snapped_divider_releases_after_moving_past_release_distance() {
        let panes = vec![pane(None, None, None), pane(None, None, None)];
        let snap_points = [SplitPaneSize::Percent(50.0)];

        let (snapped, snap_state) = resize_pair(ResizePairRequest {
            sizes: &[400.0, 600.0],
            panes: &panes,
            divider_index: 0,
            delta: 94.0,
            available: 1000.0,
            snap_points: &snap_points,
            snap_tolerance: 30.0,
            snapped_before: None,
        });
        let (still_snapped, snap_state) = resize_pair(ResizePairRequest {
            sizes: &snapped,
            panes: &panes,
            divider_index: 0,
            delta: 20.0,
            available: 1000.0,
            snap_points: &snap_points,
            snap_tolerance: 30.0,
            snapped_before: snap_state,
        });
        let (released, snap_state) = resize_pair(ResizePairRequest {
            sizes: &snapped,
            panes: &panes,
            divider_index: 0,
            delta: 26.0,
            available: 1000.0,
            snap_points: &snap_points,
            snap_tolerance: 30.0,
            snapped_before: snap_state,
        });

        assert_eq!(still_snapped, vec![500.0, 500.0]);
        assert_eq!(released, vec![526.0, 474.0]);
        assert_eq!(snap_state, None);
    }

    #[test]
    fn direction_delta_matches_pointer_motion_for_vertical_layouts() {
        assert_eq!(
            SplitPaneDirection::Vertical.delta(MoveEvent {
                delta_x: 0.0,
                delta_y: 10.0,
                modifiers: Default::default(),
            }),
            10.0
        );
        assert_eq!(
            SplitPaneDirection::Vertical.delta(MoveEvent {
                delta_x: 0.0,
                delta_y: -10.0,
                modifiers: Default::default(),
            }),
            -10.0
        );
    }

    #[test]
    fn keyboard_delta_matches_divider_motion_for_vertical_layouts() {
        assert_eq!(
            SplitPaneDirection::Vertical.keyboard_delta(MoveEvent {
                delta_x: 0.0,
                delta_y: 10.0,
                modifiers: Default::default(),
            }),
            -10.0
        );
        assert_eq!(
            SplitPaneDirection::Vertical.keyboard_delta(MoveEvent {
                delta_x: 0.0,
                delta_y: -10.0,
                modifiers: Default::default(),
            }),
            10.0
        );
    }

    #[test]
    fn persistence_round_trips_sizes() {
        let encoded = encode_sizes(&[SplitPaneSize::Px(200.0), SplitPaneSize::Percent(40.0)]);

        assert_eq!(encoded, "200px|40%");
        assert_eq!(
            decode_sizes(&encoded),
            vec![SplitPaneSize::Px(200.0), SplitPaneSize::Percent(40.0)]
        );
    }

    #[test]
    fn resize_pair_to_edge_respects_neighbor_minimum() {
        let panes = vec![
            pane(
                None,
                Some(SplitPaneSize::Px(80.0)),
                Some(SplitPaneSize::Px(500.0)),
            ),
            pane(None, Some(SplitPaneSize::Px(260.0)), None),
        ];

        assert_eq!(
            resize_pair_to_edge(&[300.0, 400.0], &panes, 0, PairEdge::MinBefore, 1000.0),
            vec![80.0, 620.0]
        );
        assert_eq!(
            resize_pair_to_edge(&[300.0, 400.0], &panes, 0, PairEdge::MaxBefore, 1000.0),
            vec![440.0, 260.0]
        );
    }

    #[test]
    fn resize_pair_respects_neighbor_maximum() {
        let panes = vec![
            pane(None, Some(SplitPaneSize::Px(100.0)), None),
            pane(None, None, Some(SplitPaneSize::Px(300.0))),
        ];

        assert_eq!(
            resize_pair(ResizePairRequest {
                sizes: &[500.0, 200.0],
                panes: &panes,
                divider_index: 0,
                delta: -400.0,
                available: 1000.0,
                snap_points: &[],
                snap_tolerance: 0.0,
                snapped_before: None,
            })
            .0,
            vec![400.0, 300.0]
        );
        assert_eq!(
            divider_bounds(&[500.0, 200.0], &panes, 0, 1000.0),
            (500.0, 400.0, 700.0)
        );
    }

    #[test]
    fn resize_pair_only_updates_the_adjacent_panes_in_multi_pane_layouts() {
        let panes = vec![
            pane(None, None, None),
            pane(None, None, None),
            pane(None, None, None),
        ];

        assert_eq!(
            resize_pair(ResizePairRequest {
                sizes: &[200.0, 300.0, 500.0],
                panes: &panes,
                divider_index: 1,
                delta: 75.0,
                available: 1000.0,
                snap_points: &[],
                snap_tolerance: 0.0,
                snapped_before: None,
            })
            .0,
            vec![200.0, 375.0, 425.0]
        );
    }

    #[test]
    fn escape_restore_recomputes_callback_sizes_from_current_container() {
        let panes = vec![
            pane(Some(SplitPaneSize::Percent(25.0)), None, None),
            pane(None, None, None),
        ];
        let restored = restore_keyboard_resize(
            KeyboardResizeState {
                stored_sizes: None,
                snapped_before: Some(500.0),
            },
            &panes,
            1600.0,
        );

        assert_eq!(
            restored,
            KeyboardEscapeRestore {
                stored_sizes: None,
                callback_sizes: vec![400.0, 1200.0],
                snapped_before: Some(500.0),
            }
        );
    }

    #[test]
    fn escape_restore_uses_committed_sizes_for_controlled_callback_payloads() {
        let panes = vec![
            controlled_pane(Some(SplitPaneSize::Percent(25.0)), None, None, None),
            pane(None, None, None),
        ];
        let restored = restore_keyboard_resize(
            KeyboardResizeState {
                stored_sizes: Some(vec![320.0, 680.0]),
                snapped_before: None,
            },
            &panes,
            1600.0,
        );

        assert_eq!(restored.stored_sizes, Some(vec![320.0, 680.0]));
        assert_eq!(restored.callback_sizes, vec![400.0, 1200.0]);
    }

    #[test]
    fn resize_pair_to_edge_only_updates_the_adjacent_panes_in_multi_pane_layouts() {
        let panes = vec![
            pane(None, None, None),
            pane(None, Some(SplitPaneSize::Px(100.0)), None),
            pane(None, Some(SplitPaneSize::Px(200.0)), None),
            pane(None, None, None),
        ];

        assert_eq!(
            resize_pair_to_edge(
                &[150.0, 250.0, 300.0, 300.0],
                &panes,
                1,
                PairEdge::MaxBefore,
                1000.0
            ),
            vec![150.0, 350.0, 200.0, 300.0]
        );
    }

    #[test]
    fn resize_event_carries_pointer_type_for_pointer_interactions() {
        let event = SplitPaneResizeEvent {
            sizes: vec![SplitPaneSize::Percent(40.0), SplitPaneSize::Percent(60.0)],
            divider_index: 0,
            source: SplitPaneResizeSource::Pointer,
            pointer_type: Some("touch".to_string()),
        };

        assert_eq!(event.pointer_type.as_deref(), Some("touch"));
    }

    #[component]
    fn SplitPaneSsrHarness() -> Element {
        rsx! {
            SplitPane {
                direction: SplitPaneDirection::Vertical,
                resizable: ReadSignal::new(Signal::new(true)),
                children: rsx! {
                    Pane { "Left pane" }
                    SplitPaneDivider { "Divider" }
                    Pane { "Right pane" }
                }
            }
        }
    }

    #[test]
    fn split_pane_renders_root_and_divider_accessibility_attrs_on_server() {
        let rendered = dioxus_ssr::render_element(rsx! {
            SplitPaneSsrHarness {}
        });

        assert!(rendered.contains("role=\"group\""));
        assert!(rendered.contains("data-orientation=\"vertical\""));
        assert!(rendered.contains("data-resizable=\"true\""));
        assert!(rendered.contains("role=\"separator\""));
        assert!(rendered.contains("aria-orientation=\"horizontal\""));
        assert!(rendered.contains("aria-valuemin=\"0\""));
        assert!(rendered.contains("aria-valuemax=\"0\""));
        assert!(rendered.contains("data-divider-index=\"0\""));
    }
}
