Color picker used when the user needs to make a customized color selection.

## Component Structure

```rust
ColorPicker {
    // The currently selected color in the color picker.
    color,
    on_color_change: move |c: Hsv<encoding::Srgb, f64>| {
        // This callback is triggered when a color is selected in the popover.
        // The HSV parameter contains the selected color.
    },

    // Optional label on the trigger button.
    label: "Pick",
}
```

### Custom Popover

`ColorPicker` renders the default trigger and `ColorPickerSelect` content. If you
want to fully replace the default popover UI, compose the picker from
`ColorPickerRoot`, `ColorPickerTrigger`, and `ColorPickerPopover`:

```rust
ColorPickerRoot {
    color,
    on_color_change: move |c: Hsv<encoding::Srgb, f64>| { /* ... */ },

    ColorPickerTrigger {
        label: "Pick",
    }

    ColorPickerPopover {
        div {
            "Custom color picker content"
        }
    }
}
```

If you only need to add content after the default controls, pass children to
`ColorPicker`; they will be appended inside the default popover.
