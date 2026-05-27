The Switch component allows users to toggle between two states, such as on and off.

## Component Structure

```rust
// The Switch component includes the switch thumb.
Switch {
    // The current state of the switch, true for on and false for off.
    checked: true,
    // Callback function triggered when the switch state changes.
    on_checked_change: |checked: bool| {
        // Handle the change in switch state.
    }
}
```
