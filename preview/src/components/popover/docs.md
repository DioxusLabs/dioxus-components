The Popover primitive provides an interactive modal that is positioned relative to the target element. It can be used to display additional options, or actions related to the trigger.

## Component Structure

```rust
// The PopoverRoot is the root component that contains the trigger and content.
PopoverRoot {
    // The PopoverTrigger contains the elements that will trigger the popover to display when clicked.
    PopoverTrigger {
        "Show Popover"
    }
    // The PopoverContent contains the content that will be displayed when the user clicks on the trigger.
    PopoverContent {
        side: ContentSide::Top,
        align: ContentAlign::Center,
        {children}
    }
}
```
