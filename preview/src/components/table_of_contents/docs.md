## Component Structure

Table of contents uses `use_scroll_spy()` to discover headings in the document and highlight the item closest to the configured scroll offset.

## Usage

Pass a heading selector through `scroll_spy_options` to generate controls from rendered document headings. `initial_data` is optional and only needed when server-rendered markup must include placeholder links before browser-side heading discovery runs.

Call the hook state's `reinitialize` callback after dynamically changing the heading list.

## Styling

The root element has `data-table-of-contents="true"`. Each control receives `data-depth`, and the active control receives `data-active="true"`.
