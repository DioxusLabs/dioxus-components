# Tag group

A Tag Group is a focusable group of tags (labels, categories, filters and similar items) with keyboard navigation, optional selection and removal.

## Structure

Single selection with [`TagGroup`](component.rs):

```rust
TagGroup {
    value: Some(value.into()),
    on_value_change: move |value| { /* ... */ },
    TagGroupLabel { "Labels" }
    TagList {
        TagGroupEmpty { "No tags" }
        Tag { index: 0usize, value: "bug", "bug" RemoveButton {} }
        Tag { index: 1usize, value: "feature", disabled: true, "feature" }
    }
}
```
