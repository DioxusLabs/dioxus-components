The VirtualList component virtualizes large lists by rendering only visible rows plus a buffer. It is useful for long, dynamic-height datasets where rendering every row at once would be expensive.

## Component Structure

```rust
VirtualList {
    // Total number of items.
    count: 2000usize,
    // Render buffer (approximate row count) above and below viewport.
    buffer: 8usize,
    // Row renderer callback receives the absolute index.
    render_item: move |idx: usize| rsx! {
        article { key: "{idx}", "{rows[idx].title}" }
    },
}
```
