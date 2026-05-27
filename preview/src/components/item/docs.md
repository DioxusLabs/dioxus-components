A versatile component for displaying content with media, title, description, and actions.

## Component Structure

```rust
ItemGroup {
    Item {
        // Available variants: Default, Outline, Muted
        variant: ItemVariant::Outline,

        // Available sizes: Default, Sm
        size: ItemSize::Default,

        ItemHeader {
            "Optional header"
        }

        ItemMedia {
            // Media variants: Default, Icon, Image
            variant: ItemMediaVariant::Image,
            img { src: "/path/to/image.png", alt: "Description" }
        }

        ItemContent {
            ItemTitle { "Item title" }
            ItemDescription { "Detailed description that can span multiple lines." }
        }

        ItemActions {
            button { "Primary action" }
        }

        ItemFooter {
            "Optional footer"
        }
    }

    ItemSeparator {}

    Item {
        // ... next item in the group
    }
}
```
