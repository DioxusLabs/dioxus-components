The SplitPane component creates resizable pane layouts with accessible pointer and keyboard dividers. Use it for editors, dashboards, file browsers, and other views where users need to allocate space between panels.

## Component Structure

```rust
SplitPane {
    direction: SplitPaneDirection::Horizontal,
    step: 24.0,

    Pane {
        default_size: SplitPaneSize::percent(35.0),
        min_size: SplitPaneSize::px(160.0),
        "Left pane"
    }
    SplitPaneDivider {}
    Pane {
        min_size: SplitPaneSize::px(220.0),
        "Right pane"
    }
}
```

The root `SplitPane` container must resolve to a concrete width for `Horizontal` layouts or a concrete height for `Vertical` layouts. In practice, give the parent or the `SplitPane` itself an explicit size such as `width: 100%` inside a sized flex row, `height: 320px`, or `height: 100%` inside a parent with its own resolved height.

## Sizing

Use `default_size` for uncontrolled panes and `size` for controlled panes. `SplitPaneSize::px` and `SplitPaneSize::percent` can be mixed with `min_size` and `max_size` constraints.

```rust
let mut sidebar = use_signal(|| Some(SplitPaneSize::percent(30.0)));

Pane {
    size: sidebar,
    min_size: SplitPaneSize::px(180.0),
    max_size: SplitPaneSize::percent(50.0),
    "Controlled sidebar"
}
```

## Interaction

Dividers support pointer dragging, arrow keys, Home, End, Escape, and focus styles. Set `step` to control keyboard resize increments. Use `snap_points` and `snap_tolerance` to make a divider lock to common sizes.

```rust
SplitPane {
    snap_points: vec![SplitPaneSize::percent(25.0), SplitPaneSize::percent(50.0)],
    snap_tolerance: 18.0,
    step: 16.0,
}
```

Add more than two panes by alternating `Pane` and `SplitPaneDivider` children:

```rust
SplitPane {
    direction: SplitPaneDirection::Horizontal,
    Pane { default_size: SplitPaneSize::percent(22.0), "Nav" }
    SplitPaneDivider {}
    Pane { default_size: SplitPaneSize::percent(48.0), "Editor" }
    SplitPaneDivider {}
    Pane { min_size: SplitPaneSize::px(220.0), "Inspector" }
}
```

## Persistence

`use_split_pane_persistence` returns restored sizes and an `on_resize_end` callback that stores final pane sizes in browser storage.

```rust
let (stored_sizes, persist_sizes) =
    use_split_pane_persistence("workspace-layout", SplitPaneStorage::Local);

SplitPane {
    on_resize_end: persist_sizes,
    Pane {
        default_size: stored_sizes().and_then(|sizes| sizes.get(0).cloned()),
        "Restored pane"
    }
}
```

## Styling

Pass `divider_class`, `divider_style`, or per-divider `class` and `style` values to customize handles. `Horizontal` layouts use vertical grab handles with `col-resize` semantics, while `Vertical` layouts use horizontal grab handles with `row-resize` semantics. Pane and root attributes are forwarded to the primitive, so layout-specific classes and labels can be supplied where needed.
