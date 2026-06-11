The Combobox family provides reusable listbox/search interactions for autocomplete-style inputs.
The low-level `Combobox` owns dropdown, highlight, option registry, and submit interaction state;
higher-level components own their own value and query models.

Filtering preserves the order defined by the rendered `ComboboxOption` elements and their `index`
props. If you want query-dependent ranking, control `query`, sort your item data in user code,
render the options in that sorted order, and assign indexes from the sorted list.

## Variants

- `Combobox` is the low-level selectable autocomplete surface.
- `Autocomplete` owns string input value and maps option submit to the input label.
- `MultiSelect` owns a selected-value array, search query, max-values, and selected pills.
- `TagsInput` owns tag parsing and removable pills.
- `VirtualizedComboboxOptions` renders a listbox with only the visible option window while preserving `ComboboxOption` ids and indexes.

## Component Structure

```rust
let mut value = use_signal(|| None::<String>);
let mut query = use_signal(String::new);

Combobox::<String> {
    value: Some(value.into()),
    on_value_change: move |next: Option<String>| {
        value.set(next);
    },
    query: Some(query()),
    on_query_change: move |next| query.set(next),
    placeholder: "Select framework...",
    aria_label: "Select framework",
    list_aria_label: "Frameworks",
    ComboboxEmpty { "No framework found." }
    ComboboxOption::<String> {
        index: 0usize,
        value: "next".to_string(),
        text_value: "Next.js",
        "Next.js"
    }
}
```

## MultiSelect

```rust
MultiSelect::<String> {
    default_values: vec!["mushroom".to_string()],
    max_values: 3usize,
    render_value: |value: String| rsx! { "{value}" },
    placeholder: "Pick toppings...",
    ComboboxOption::<String> {
        index: 0usize,
        value: "mushroom".to_string(),
        text_value: "Mushroom",
        "Mushroom"
    }
}
```

## Virtualized Options

```rust
VirtualizedCombobox::<String> {
    count: 1000usize,
    estimate_size: |_: usize| 36,
    render_option: |index: usize| rsx! {
        ComboboxOption::<String> {
            index,
            value: format!("option-{index}"),
            text_value: format!("Option {index}"),
            "Option {index}"
        }
    }
}
```
