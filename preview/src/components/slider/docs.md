The slider component allows users to select a value from a range by sliding a handle along a track.

## Component Structure

```rust
Slider {
    value: 0.0,
    horizontal: true,
    on_value_change: |value: f64| {
        // Handle the change in slider value.
    },
}
```

For a two-thumb range selector, use `RangeSlider`:

```rust
RangeSlider {
    default_value: 20.0..80.0,
    on_value_change: |value: std::ops::Range<f64>| {
        // value.start and value.end give the two endpoints
    },
}
```
