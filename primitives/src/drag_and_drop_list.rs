//! Defines the [`DragAndDropList`] component and its sub-components.
use crate::collection::{collection_item, use_collection_provider, use_item, CollectionState};
use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Debug)]
enum DropPosition {
    Before,
    Undefined,
    After,
}

impl From<std::cmp::Ordering> for DropPosition {
    fn from(ord: std::cmp::Ordering) -> Self {
        match ord {
            std::cmp::Ordering::Less => Self::Before,
            std::cmp::Ordering::Equal => Self::Undefined,
            std::cmp::Ordering::Greater => Self::After,
        }
    }
}

fn sortable_item_key(children: &Element, index: usize) -> String {
    children
        .as_ref()
        .ok()
        .and_then(|vnode| vnode.key())
        .map(|key| key.to_string())
        .unwrap_or_else(|| index.to_string())
}

#[derive(Clone, PartialEq)]
struct SortableListItem {
    key: String,
    children: Element,
}

impl SortableListItem {
    fn new(children: Element, index: usize) -> Self {
        Self {
            key: sortable_item_key(&children, index),
            children,
        }
    }
}

/// Resolves the final insertion index from a hovered item and pointer position.
fn resolve_drop_index(from: usize, hovered: usize, position: DropPosition) -> usize {
    let slot = match position {
        DropPosition::Before | DropPosition::Undefined => hovered,
        DropPosition::After => hovered + 1,
    };

    if from < slot {
        slot - 1
    } else {
        slot
    }
}

/// Resolves whether the final insertion index is before or after the source item.
fn resolve_drop_position(from: usize, to: usize) -> DropPosition {
    to.cmp(&from).into()
}

/// Context provided by [`DragAndDropListItem`] to its children.
/// Use `use_context::<DragAndDropItemContext>()` to access the current item's index.
#[derive(Clone, Copy)]
pub struct DragAndDropItemContext {
    index: Signal<usize>,
}

impl DragAndDropItemContext {
    /// Returns the index of the current item in the list.
    pub fn index(&self) -> usize {
        (self.index)()
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum DragState {
    Idle,
    Dragging {
        from: usize,
        to: Option<usize>,
        position: DropPosition,
    },
    Dropped {
        from: usize,
        to: usize,
    },
}

/// Context provided by [`DragAndDropList`] to its descendants.
/// Use `use_context::<DragAndDropContext>()` to access list-level operations.
#[derive(Clone, Copy)]
pub struct DragAndDropContext {
    drag: Signal<DragState>,
    list_items: Signal<Vec<SortableListItem>>,
    focus: CollectionState,
    announcement: Signal<String>,
}

impl DragAndDropContext {
    fn drag_from(&self) -> Option<usize> {
        match (self.drag)() {
            DragState::Idle => None,
            DragState::Dragging { from, .. } | DragState::Dropped { from, .. } => Some(from),
        }
    }

    fn drop_to(&self) -> Option<usize> {
        match (self.drag)() {
            DragState::Idle => None,
            DragState::Dragging { to, .. } => to,
            DragState::Dropped { to, .. } => Some(to),
        }
    }

    fn drop_position(&self) -> DropPosition {
        match (self.drag)() {
            DragState::Dragging { position, .. } => position,
            _ => DropPosition::Undefined,
        }
    }

    fn is_dragging(&self) -> bool {
        !matches!((self.drag)(), DragState::Idle)
    }

    fn start_drag(&mut self, index: usize) {
        self.drag.set(DragState::Dragging {
            from: index,
            to: None,
            position: DropPosition::Undefined,
        });
    }

    fn end_drag(&mut self) {
        let focus_target = self.drop_to().or(self.drag_from());
        self.set_focus(focus_target);
        self.drag.set(DragState::Idle);
    }

    fn cancel_drag(&mut self) {
        self.set_focus(self.drag_from());
        self.drag.set(DragState::Idle);
    }

    fn drag_over(&mut self, hovered: usize, position: DropPosition) {
        let DragState::Dragging { from, .. } = (self.drag)() else {
            return;
        };
        let resolved = resolve_drop_index(from, hovered, position);
        self.drag.set(DragState::Dragging {
            from,
            to: Some(resolved),
            position: resolve_drop_position(from, resolved),
        });
    }

    fn drop(&mut self) {
        let DragState::Dragging {
            from, to: Some(to), ..
        } = (self.drag)()
        else {
            return;
        };
        let mut list = (self.list_items)();
        let item = list.remove(from);
        list.insert(to, item);
        self.list_items.set(list);
        self.drag.set(DragState::Dropped { from, to });
    }

    /// Remove the item at the given index from the list.
    pub fn remove(&mut self, index: usize) {
        let mut list = (self.list_items)();
        if index < list.len() {
            list.remove(index);
            let new_len = list.len();
            let focus_target = new_len.checked_sub(1).map(|last| index.min(last));
            let focus_id =
                focus_target.and_then(|index| list.get(index).map(|item| item.key.clone()));
            self.list_items.set(list);
            self.focus.set_focus_key(focus_id);
            self.announcement.set(format!(
                "Removed item from position {}. {} items remaining",
                index + 1,
                new_len
            ));
        }
    }

    fn announce(&mut self, msg: String) {
        self.announcement.set(msg);
    }

    fn item_count(&self) -> usize {
        (self.list_items)().len()
    }

    fn is_focused(&self, index: usize) -> bool {
        self.focus.is_focused(index)
    }

    fn set_focus(&mut self, index: Option<usize>) {
        // Every sortable item carries a key, so resolve focus by key to stay
        // stable across reordering. An out-of-bounds (or `None`) index yields no
        // key, which clears focus rather than pointing at an invalid slot.
        let id =
            index.and_then(|index| (self.list_items)().get(index).map(|item| item.key.clone()));
        self.focus.set_focus_key(id);
    }

    fn focus_next(&mut self) {
        self.focus.focus_next();
    }

    fn focus_prev(&mut self) {
        self.focus.focus_prev();
    }

    fn move_up(&mut self, index: usize) {
        let DragState::Dragging { from, to, .. } = (self.drag)() else {
            return;
        };
        let current = to.unwrap_or(index);
        let len = (self.list_items)().len();
        let new_to = current.checked_sub(1).unwrap_or(len - 1);
        self.drag.set(DragState::Dragging {
            from,
            to: Some(new_to),
            position: resolve_drop_position(from, new_to),
        });
    }

    fn move_down(&mut self, index: usize) {
        let DragState::Dragging { from, to, .. } = (self.drag)() else {
            return;
        };
        let current = to.unwrap_or(index);
        let len = (self.list_items)().len();
        let new_to = (current + 1) % len;
        self.drag.set(DragState::Dragging {
            from,
            to: Some(new_to),
            position: resolve_drop_position(from, new_to),
        });
    }

    fn announce_move(&mut self, index: usize) {
        let pos = self.drop_to().unwrap_or(index) + 1;
        let count = self.item_count();
        self.announce(format!(
            "You have moved the item to position {pos} of {count}"
        ));
    }

    fn toggle_drag(&mut self, index: usize) {
        if self.is_dragging() {
            let from = self.drag_from().unwrap_or(index) + 1;
            let to = self.drop_to().unwrap_or(index) + 1;
            self.drop();
            self.end_drag();
            self.announce(format!(
                "You have dropped the item. It has moved from position {from} to position {to}"
            ));
        } else {
            let count = self.item_count();
            self.start_drag(index);
            self.drag_over(index, DropPosition::Undefined);
            self.announce(format!(
                "You have lifted an item in position {} of {count}",
                index + 1
            ));
        }
    }
}

/// The props for the [`DragAndDropList`] component.
#[derive(Props, Clone, PartialEq)]
pub struct DragAndDropListProps {
    /// Items (labels) to be rendered.
    pub items: Vec<Element>,

    /// Accessible label for the list
    #[props(default)]
    pub aria_label: Option<String>,

    /// Additional attributes to apply to the list element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the list component.
    #[props(default)]
    pub children: Option<Element>,
}

/// The props for the [`DragAndDropListItems`] component.
#[derive(Props, Clone, PartialEq)]
pub struct DragAndDropListItemsProps {
    /// Accessible label for the list.
    pub aria_label: String,

    /// Additional attributes to apply to the inner list element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the inner list element.
    #[props(default)]
    pub children: Option<Element>,
}

/// The props for the [`DragAndDropInstructions`] component.
#[derive(Props, Clone, PartialEq)]
pub struct DragAndDropInstructionsProps {
    /// Additional attributes to apply to the instructions element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// The props for the [`DragAndDropLiveRegion`] component.
#[derive(Props, Clone, PartialEq)]
pub struct DragAndDropLiveRegionProps {
    /// Additional attributes to apply to the live region element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// # DragAndDropList
///
/// A list can be used to display content related to a single subject.
/// The content can consist of multiple elements of varying type and size.
/// Used when a user wants to change a collection order.
///
/// ## Example
///
/// ```rust
///use dioxus::prelude::*;
///use dioxus_primitives::drag_and_drop_list::{DragAndDropList, DragAndDropListItem};
///#[component]
///pub fn Demo() -> Element {
///    let items = ["Item1", "Item2", "Item3"]
///        .map(|t| {
///            rsx! { {t} }
///        })
///        .to_vec();
///    rsx! {
///        DragAndDropList { items }
///    }
///}
/// ```
#[component]
pub fn DragAndDropList(props: DragAndDropListProps) -> Element {
    let drag = use_signal(|| DragState::Idle);
    let list_items = use_signal(|| {
        props
            .items
            .iter()
            .cloned()
            .enumerate()
            .map(|(index, item)| SortableListItem::new(item, index))
            .collect()
    });
    let announcement = use_signal(String::new);
    let focus = use_collection_provider(ReadSignal::new(Signal::new(true)));

    use_context_provider(move || DragAndDropContext {
        drag,
        list_items,
        focus,
        announcement,
    });

    let label = props
        .aria_label
        .as_deref()
        .unwrap_or("Sortable list")
        .to_string();

    let children = props.children.unwrap_or_else(|| {
        rsx! {
            DragAndDropInstructions {}
            DragAndDropListItems {
                aria_label: label,
            }
            DragAndDropLiveRegion {}
        }
    });

    rsx! {
        div {
            ..props.attributes,
            {children}
        }
    }
}

/// Return render data for the current sortable items.
pub fn use_drag_and_drop_list_items() -> Vec<DragAndDropListRenderItem> {
    let ctx: DragAndDropContext = use_context();
    (ctx.list_items)()
        .into_iter()
        .enumerate()
        .map(|(index, item)| {
            // Propagate any `key:` the caller set on the item's root element
            // through to the keyed sortable item fragment.
            DragAndDropListRenderItem {
                index,
                key: item.key,
                children: item.children,
            }
        })
        .collect()
}

/// The inner list element for sortable items.
#[component]
pub fn DragAndDropListItems(props: DragAndDropListItemsProps) -> Element {
    let mut ctx: DragAndDropContext = use_context();
    let children = props.children.unwrap_or_else(|| {
        rsx! {
            for item in use_drag_and_drop_list_items() {
                Fragment {
                    key: "{item.key}",
                    DragAndDropDropIndicator {
                        index: item.index,
                        position: "before",
                    }
                    DragAndDropListItem {
                        index: item.index,
                        item_key: item.key.clone(),
                        {item.children}
                    }
                    DragAndDropDropIndicator {
                        index: item.index,
                        position: "after",
                    }
                }
            }
        }
    });

    rsx! {
        ul {
            aria_label: "{props.aria_label}",
            aria_roledescription: "sortable list",
            aria_describedby: "dnd-instructions",
            ondragover: move |event: Event<DragData>| {
                // Drops can happen in the visual gaps between items. The
                // nearest item still owns target calculation, but the list
                // must accept the final drop for those gap targets.
                event.prevent_default();
                event.data_transfer().set_drop_effect("move");
            },
            ondrop: move |event: Event<DragData>| {
                event.prevent_default();
                ctx.drop();
            },
            ..props.attributes,
            {children}
        }
    }
}

/// Screen-reader instructions for keyboard sorting.
#[component]
pub fn DragAndDropInstructions(props: DragAndDropInstructionsProps) -> Element {
    rsx! {
        div {
            id: "dnd-instructions",
            style: "position:absolute;width:1px;height:1px;overflow:hidden;clip:rect(0,0,0,0);",
            ..props.attributes,
            "Press Enter to start reordering. Use Arrow keys to change position. Press Enter to confirm or Escape to cancel."
        }
    }
}

/// Live region for drag-and-drop announcements.
#[component]
pub fn DragAndDropLiveRegion(props: DragAndDropLiveRegionProps) -> Element {
    let ctx: DragAndDropContext = use_context();
    let announcement = (ctx.announcement)();

    rsx! {
        div {
            role: "status",
            aria_live: "assertive",
            aria_atomic: "true",
            style: "position:absolute;width:1px;height:1px;overflow:hidden;clip:rect(0,0,0,0);",
            ..props.attributes,
            "{announcement}"
        }
    }
}

/// The props for the [`DragAndDropListItemProps`] component.
#[derive(Props, Clone, PartialEq)]
pub struct DragAndDropListItemProps {
    /// The index of the item in the list
    pub index: usize,

    /// Stable identity for this item. Pass the same value as the item's
    /// `key:` when manually rendering sortable items.
    #[props(default)]
    pub item_key: Option<String>,

    /// Additional attributes to apply to the list item element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the list item component.
    pub children: Element,
}

/// The props for the [`DragAndDropDropIndicator`] component.
#[derive(Props, Clone, PartialEq)]
pub struct DragAndDropDropIndicatorProps {
    /// The index of the item this indicator is adjacent to.
    pub index: usize,

    /// The indicator position relative to the item.
    pub position: &'static str,

    /// Additional attributes to apply to the drop indicator element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// Data for rendering a sortable list item.
#[derive(Clone, PartialEq)]
pub struct DragAndDropListRenderItem {
    /// The current index of this item.
    pub index: usize,

    /// The stable key for this item.
    pub key: String,

    /// The rendered item children.
    pub children: Element,
}

/// # DragAndDropListItem
///
/// This component represents an individual draggable item in the dnd list.
/// This must be used inside a [`DragAndDropList`] component.
///
/// ## Example
///
/// ```rust
///use dioxus::prelude::*;
///use dioxus_primitives::drag_and_drop_list::{DragAndDropList, DragAndDropListItem};
///#[component]
///pub fn Demo() -> Element {
///    let items = ["Item1", "Item2", "Item3"]
///        .map(|t| {
///            rsx! { {t} }
///        })
///        .to_vec();
///    rsx! {
///        DragAndDropList { items }
///    }
///}
/// ```
#[component]
pub fn DragAndDropListItem(props: DragAndDropListItemProps) -> Element {
    let mut ctx: DragAndDropContext = use_context();

    let index = props.index;
    let mut item_ctx = use_context_provider(move || DragAndDropItemContext {
        index: Signal::new(index),
    });
    if *item_ctx.index.peek() != index {
        item_ctx.index.set(index);
    }
    let index_signal = item_ctx.index;

    let item_key = props.item_key.clone();
    let item = use_item(collection_item(ctx.focus, index_signal).key(move || item_key.clone()));
    let mut collection_onmounted = item.onmounted();
    let mut item_ref: Signal<Option<std::rc::Rc<MountedData>>> = use_signal(|| None);

    let onkeydown = move |event: Event<KeyboardData>| {
        let key = event.key();

        match key {
            Key::ArrowUp => {
                event.prevent_default();
                if ctx.is_dragging() {
                    ctx.move_up(index);
                    ctx.announce_move(index);
                } else {
                    ctx.focus_prev();
                }
            }
            Key::ArrowDown => {
                event.prevent_default();
                if ctx.is_dragging() {
                    ctx.move_down(index);
                    ctx.announce_move(index);
                } else {
                    ctx.focus_next();
                }
            }
            Key::Enter => {
                event.prevent_default();
                ctx.toggle_drag(index);
            }
            Key::Character(ref c) if c == " " => {
                event.prevent_default();
                ctx.toggle_drag(index);
            }
            Key::Escape => {
                event.prevent_default();
                if ctx.is_dragging() {
                    let pos = ctx.drag_from().unwrap_or(index) + 1;
                    ctx.cancel_drag();
                    ctx.announce(format!(
                        "Movement cancelled. The item has returned to its starting position of {pos}"
                    ));
                }
            }
            Key::Delete | Key::Backspace => {
                event.prevent_default();
                if !ctx.is_dragging() {
                    ctx.remove(index);
                }
            }
            Key::Home => {
                event.prevent_default();
                if !ctx.is_dragging() {
                    ctx.set_focus(Some(0));
                }
            }
            Key::End => {
                event.prevent_default();
                if !ctx.is_dragging() {
                    ctx.set_focus(ctx.item_count().checked_sub(1));
                }
            }
            _ => {}
        };
    };

    rsx! {
        li {
            aria_roledescription: "sortable item",
            draggable: "true",
            tabindex: item.tabindex,
            aria_grabbed: if ctx.drag_from().is_some_and(|from| from == index) { "true" } else { "false" },
            "data-is-grabbing": if ctx.drag_from().is_some_and(|from| from == index) { "true" },
            // Set when the drop target has returned to this item's starting slot —
            // i.e. dropping now would leave it in place. The primitive suppresses
            // the drop indicator in that case (no gap to point to), so styling
            // hooks off this attribute to surface the "stays here" state.
            "data-drop-at-origin": if ctx.drag_from().is_some_and(|from| from == index) && ctx.drop_to() == Some(index) { "true" },
            "data-focus-visible": if ctx.is_focused(index) { "true" },
            onmounted: move |event| {
                item_ref.set(Some(event.data()));
                collection_onmounted(event);
            },
            onfocus: move |_| {
                if !ctx.is_dragging() {
                    ctx.set_focus(Some(index));
                }
            },
            ondragstart: move |event: Event<DragData>| {
                ctx.start_drag(index);
                event.data_transfer().set_effect_allowed("move");
                event.data_transfer().set_drop_effect("move");
                // Note: this is only for Firefox (without it, DnD won't work)
                let _ = event.data_transfer().set_data("text/html", "");
                let mut document_drop_ctx = ctx;
                let mut document_drop = document::eval(
                    r#"
                    function cleanup() {
                        document.removeEventListener("dragover", onDragOver, true);
                        document.removeEventListener("drop", onDrop, true);
                        document.removeEventListener("dragend", onDragEnd, true);
                    }

                    function onDragOver(event) {
                        event.preventDefault();
                        if (event.dataTransfer) {
                            event.dataTransfer.dropEffect = "move";
                        }
                    }

                    function onDrop(event) {
                        event.preventDefault();
                        dioxus.send("drop");
                        cleanup();
                    }

                    function onDragEnd() {
                        dioxus.send("end");
                        cleanup();
                    }

                    document.addEventListener("dragover", onDragOver, true);
                    document.addEventListener("drop", onDrop, true);
                    document.addEventListener("dragend", onDragEnd, true);

                    await dioxus.recv();
                    cleanup();
                    "#,
                );
                spawn(async move {
                    if let Ok(action) = document_drop.recv::<String>().await {
                        if action == "drop" {
                            document_drop_ctx.drop();
                        }
                    }
                    let _ = document_drop.send(true);
                });
            },
            ondragend: move |_| ctx.end_drag(),
            ondragover: move |event: Event<DragData>| {
                event.prevent_default();
                event.data_transfer().set_drop_effect("move");
                async move {
                    if let Some(md) = item_ref() {
                        let cursor_y = event.client_coordinates().y;
                        if let Ok(rect) = md.get_client_rect().await {
                            let mid_y = rect.origin.y + rect.size.height / 2.0;
                            let position = if cursor_y < mid_y {
                                DropPosition::Before
                            } else {
                                DropPosition::After
                            };
                            ctx.drag_over(index, position);
                        }
                    }
                }
            },
            //ondragleave: move |_| ctx.drop_to.set(None),
            onkeydown,
            ..props.attributes,
            {props.children}
        }
    }
}

/// The drop indicator rendered next to a sortable item.
#[component]
pub fn DragAndDropDropIndicator(props: DragAndDropDropIndicatorProps) -> Element {
    let ctx: DragAndDropContext = use_context();
    let render = ctx.drop_to().is_some_and(|to| to == props.index)
        && match props.position {
            "before" => ctx.drop_position() == DropPosition::Before,
            "after" => ctx.drop_position() == DropPosition::After,
            _ => false,
        };
    if !render {
        return rsx! {};
    }

    rsx! {
        div {
            "data-position": "{props.position}",
            ..props.attributes,
        }
    }
}
