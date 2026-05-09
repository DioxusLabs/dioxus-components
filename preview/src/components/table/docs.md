The Table component displays structured data with support for filtering, column visibility toggling, row selection, sortable columns, and pagination.

## Component Structure

```rust
// The Demo component renders a fully functional data table
Demo {}
```

## Features

- **Filter**: Type in the search box to filter rows by email in real time.
- **Column Visibility**: Click the "Columns" button to show or hide the Status, Email, and Amount columns.
- **Row Selection**: Use the checkboxes to select individual rows or all rows at once.
- **Sorting**: Click the Email column header to toggle ascending/descending sort order.
- **Row Actions**: Click the `···` button on any row to open a context menu with actions like "Copy payment ID", "View customer", and "View payment details".
- **Pagination**: Previous and Next buttons are shown in the footer along with a selected row count.

## Data Structure

```rust
struct Payment {
    id: usize,
    status: &'static str, // "Success" | "Processing" | "Failed"
    email: &'static str,
    amount: &'static str,
}
```