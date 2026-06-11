# use_combobox Architecture Plan

## Goal

Add a reusable `use_combobox()` primitive hook that can power higher-level components such as `Autocomplete`, `Select`, `MultiSelect`, `TagsInput`, and `PillsInput`.

The hook should follow the spirit of Mantine's `useCombobox`: shared dropdown, focus, highlighted-option, and option-submit behavior lives in one store, while each higher-level component owns its own value and query model.

## Source Model

Mantine's `useCombobox()` provides a store with:

- controlled or uncontrolled dropdown open state
- event-source-aware open, close, and toggle methods
- highlighted option index navigation
- first, active, next, previous, reset, update, and click selected option methods
- list id and target/search focus refs
- deferred focus and selected-index updates

Mantine components then compose that store differently:

- `Autocomplete` owns input text and maps option submit to setting the input label.
- Mantine `Select` owns a single selected value and search text.
- `MultiSelect` owns selected value arrays, pills, search text, and removal behavior.
- `TagsInput` owns tag parsing, tag arrays, pills, and search text.

The key architectural point is that `useCombobox()` is the shared interaction engine, not the owner of every component's value state.

## Local Constraints

The local repo already has related primitive infrastructure:

- `primitives/src/combobox/context.rs` contains `ComboboxContext`.
- `primitives/src/combobox/components/*` contains the current primitive components.
- `primitives/src/selectable.rs` owns generic selectable value behavior.
- `primitives/src/listbox.rs`, `focus.rs`, and `selection.rs` provide list/focus/selection concepts.
- `dioxus-components/src/components/combobox/*` contains styled wrappers.
- The current local multi-value select primitive is named `SelectMulti`; a separate `MultiSelect` component should still be added rather than treating `SelectMulti` as the final public multi-select surface.
- Local `Select`/`SelectMulti` currently use typeahead behavior rather than Mantine-style searchable query text.
- Local `Autocomplete`, `TagsInput`, `PillsInput`, and `MultiSelect` component modules do not exist yet; those names describe the target Mantine-inspired component set.

The new hook must not create an unrelated parallel system. It should become the backing store for the combobox primitive layer while preserving existing public component behavior where practical.

## Existing Hooks and Helpers to Reuse

The implementation should fit the repo's existing "provider hook plus per-element hook" pattern instead of inventing a separate prop-building style.

Relevant existing pieces:

- `use_controlled`: existing controlled/uncontrolled state helper. `use_disclosure()` should build on this or replace repeated boolean open-state usage with a public transition-aware abstraction.
- `use_focus_provider`: creates a signal-backed `FocusState` used by roving focus systems.
- `use_focus_entry_disabled`: registers an item index and disabled state with a focus provider.
- `use_focus_control_disabled` / `use_focus_controlled_item_disabled`: return `onmounted` handlers for focusable elements and keep mounted-node focus control outside component bodies.
- `use_deferred_focus`: handles "focus first/last after something opens" behavior.
- `use_listbox_container`: wires listbox id/render state and animated open rendering.
- `use_listbox_option`: registers an option id, index, text value, disabled state, and value metadata.
- `use_selectable_root` / `use_selectable_option`: current higher-level composition of controlled open state, focus state, option registry, and selected values.
- `pointer_select_start`, `pointer_select_commit`, and `pointer_select_cancel`: normalize pointer selection and avoid accidental touch scroll selection.

Preferred combobox direction:

- Factor reusable focus/listbox/option-registry behavior out of `SelectableContext` where needed rather than duplicating it inside `ComboboxStore`.
- Keep `SelectableContext` for components that own selected values, but avoid making `use_combobox()` depend on selected-value ownership.
- Follow the local hook shape where root hooks provide clone/copy signal-backed context handles and element hooks return event/mounted handlers to spread into rendered elements.
- If a hook needs to return a bundle for spreading, use a small typed struct with event handlers and computed ids/attributes rather than ad hoc attributes in unrelated component code.

## Core Boundary

`use_combobox()` should be value-agnostic.

It should own:

- dropdown opened state
- open, close, and toggle behavior
- event source for open/close requests
- highlighted option index
- option registration
- disabled and invisible option skipping
- active-option selection
- selected-option submit request
- target/search focus wiring
- focus target/search methods
- list and active-descendant ids

It should not own:

- selected value
- selected value arrays
- tag arrays
- pill rendering
- query filtering in the first implementation
- virtualizer scroll state
- component-specific clear, blur, or create-new-item behavior

Higher-level components should compose `use_combobox()` and then own their domain state.

## Public API Shape

Add a new module:

```text
primitives/src/combobox/hook.rs
```

Export it from:

```text
primitives/src/combobox/mod.rs
```

Initial public types:

```rust
pub enum ComboboxDropdownEventSource {
    Keyboard,
    Mouse,
    Unknown,
}

pub struct UseComboboxOptions {
    pub opened: Option<bool>,
    pub default_opened: Option<bool>,
    pub on_opened_change: Option<EventHandler<bool>>,
    pub on_dropdown_open: Option<EventHandler<ComboboxDropdownEventSource>>,
    pub on_dropdown_close: Option<EventHandler<ComboboxDropdownEventSource>>,
    pub loop_navigation: bool,
}

pub struct ComboboxStore {
    // private fields
}

pub struct ComboboxSubmittedOption {
    pub id: String,
    pub index: usize,
    // Additional metadata can be added here without making the hook own selected values.
}

pub fn use_combobox(options: UseComboboxOptions) -> ComboboxStore;
```

Do not expose the current crate-internal `Controlled<T>` in this public API unless it is intentionally made public and documented. The initial hook can use explicit `opened`, `default_opened`, and callback fields, or it can delegate that open-state trio to a new public `use_disclosure()` hook if that hook is added first.

Preferred direction: introduce `use_disclosure()` as the reusable public open/closed state primitive, then have `use_combobox()` build on it for dropdown state. `use_disclosure()` should own controlled/uncontrolled boolean state plus transition-aware `open`, `close`, and `toggle` helpers. `use_combobox()` should add combobox-specific event-source callbacks on top.

`ComboboxStore` should be a clone/copy signal-backed handle that can be moved into Dioxus closures and contexts. It should follow the local shape of `ComboboxContext` and `SelectableContext`: cheap handles containing signals, memos, and callbacks rather than a uniquely borrowed mutable state object.

## Store Methods

The store should expose methods equivalent to Mantine's behavior, adapted to Rust naming and local semantics:

```rust
impl ComboboxStore {
    pub fn dropdown_opened(&self) -> bool;
    pub fn open_dropdown(&self, source: ComboboxDropdownEventSource);
    pub fn close_dropdown(&self, source: ComboboxDropdownEventSource);
    pub fn toggle_dropdown(&self, source: ComboboxDropdownEventSource);

    pub fn highlighted_option_index(&self) -> Option<usize>;
    pub fn select_option(&self, index: usize) -> Option<ComboboxOptionKey>;
    pub fn select_first_option(&self) -> Option<ComboboxOptionKey>;
    pub fn select_active_option(&self) -> Option<ComboboxOptionKey>;
    pub fn select_next_option(&self) -> Option<ComboboxOptionKey>;
    pub fn select_previous_option(&self) -> Option<ComboboxOptionKey>;
    pub fn reset_selected_option(&self);
    pub fn update_selected_option_index(&self, target: ComboboxIndexTarget);

    pub fn submitted_option(&self) -> Option<ComboboxSubmittedOption>;

    pub fn focus_target(&self);
    pub fn focus_search_input(&self);
}
```

Use `highlighted_option_index` internally and publicly when possible. Mantine calls this `selectedOptionIndex`, but in this repo "selected" already means selected value, so "highlighted" is less ambiguous.

The method signatures above are intentionally signal-handle style. Future implementation may adjust exact argument types, but it should not require an exclusive `&mut ComboboxStore` borrow for normal event handlers.

Mounted-node registration for targets and search inputs should stay behind declarative element hooks such as `use_combobox_target()` and `use_combobox_search()`, whose handles expose `spread()` for the rendered element. Raw attribute helpers such as `use_combobox_target_attributes()` and `use_combobox_search_attributes()` may remain as compatibility wrappers, but `ComboboxStore` should not expose public mount-registration methods.

Navigation methods should return stable option keys or submitted-option metadata, not selected values. Root/context code should translate the returned option metadata into the component's value behavior.

## Option Registry

Mantine uses DOM queries under a list id. Dioxus should prefer an explicit Rust registry.

Each option should register:

- stable id
- index/order
- option value or submit payload data
- disabled state
- visible state
- active state
- mounted node, if available

`ComboboxOption` should register and unregister itself through the store/context lifecycle.

Submit ownership should stay at the combobox root/context boundary, matching Mantine's `Combobox` `onOptionSubmit` model. Options provide value/id metadata; the store can request submission of the currently highlighted option, but the root-level context decides how to handle that submitted value. Avoid per-option submit handlers as the default model because they make `Autocomplete`, `Select`, `SelectMulti`, creatable tags, and custom option rendering harder to compose.

The preferred dispatch shape is:

```text
keyboard/pointer event
  -> store selects or looks up highlighted option
  -> store returns ComboboxSubmittedOption metadata
  -> ComboboxContext/root on_option_submit handles component-specific value changes
```

This keeps the store value-agnostic while still giving `Autocomplete`, `Select`, `SelectMulti`, `TagsInput`, and custom combobox users a single root-level submit path.

Navigation should walk the registry and skip disabled or invisible options. Dynamic lists and filtering should update the registry without leaving stale highlighted indices.

Existing rendered attributes should be preserved:

- `data-highlighted`
- `data-disabled`
- `data-selected`
- `aria-selected`
- `aria-disabled`

Only add Mantine-style `data-combobox-*` attributes if there is a concrete styling, testing, or compatibility reason. Do not replace the existing attributes because styled wrappers may depend on them.

## Open and Close Semantics

Open and close callbacks should follow Mantine's transition semantics:

- `open_dropdown` should call `on_dropdown_open` only when transitioning from closed to open.
- `close_dropdown` should call `on_dropdown_close` only when transitioning from open to closed.
- `on_opened_change` should reflect actual state changes, not repeated requests to set the current state.
- event source should be preserved through `toggle_dropdown`.

This should be handled inside the store instead of relying directly on the current `use_controlled` helper, because the helper may call callbacks on set requests even when transition-specific callbacks should not fire.

## Focus Model

React refs from Mantine map to Dioxus mounted-node registration.

The store should hold optional mounted data for:

- target
- search input
- registered options, if option scrolling/focus is later supported

`focus_target()` and `focus_search_input()` should no-op safely when mounted data is absent, including SSR. If deferred focus is needed to match Mantine behavior, implement it explicitly and clean up timers/tasks on drop.

The composition model should account for more than the current `ComboboxInput` component. Mantine separates target, events target, dropdown target, search input, dropdown, options, and option components. The Dioxus implementation should support these shapes from the beginning:

- input-as-target autocomplete
- button or custom element as target
- search input inside dropdown
- pill input where events target and dropdown target are not the same rendered node
- non-input target with keyboard event handling

The first implementation should include these distinct primitive pieces instead of treating `ComboboxInput` as the only valid target. The store/context should support separate target, events target, dropdown target, and search input wiring from the beginning.

## Query and Filtering Boundary

Keep query and filtering in the existing component/context layer for the first implementation.

Current combobox behavior includes:

- controlled or uncontrolled query
- `default_query`
- `on_query_change`
- filter function
- empty rendering
- closed input showing selected text while open input shows query

Moving all of that into `use_combobox()` immediately would make the hook too opinionated for `TagsInput`, `PillsInput`, and other future inputs. A later `use_autocomplete()` or component-specific wrapper can combine `use_combobox()` with query state.

## Compatibility Wrapper

The existing `Combobox<T>` component should remain compatible.

It can keep value ownership through existing selectable primitives while delegating interaction state to `ComboboxStore`:

- option submit calls existing selected-value machinery
- single-select submit closes the dropdown
- current query/filter behavior remains in the wrapper/context
- existing styled wrappers continue to render the same visible structure

This preserves the public component API while making the shared interaction logic reusable.

## Virtualization

Virtualized combobox support should be designed as an adapter, not baked into the base hook.

Future API:

```rust
pub fn use_virtualized_combobox(options: UseVirtualizedComboboxOptions) -> ComboboxStore;
```

Virtualized options should provide:

- total option count
- disabled predicate by index
- option id by index
- active option index
- highlighted option index
- external highlighted-index setter
- scroll-to-index callback
- submit callback by index

The base hook should not require all options to be mounted. The first implementation should keep the registry abstraction narrow enough that a virtual registry backend can be added without redesigning every store method.

The lower-level `primitives/src/virtual/*` virtualizer algorithms are the intended reusable layer for combobox virtualization. The existing `virtual_list` component renders generic list/listitem semantics. A virtualized combobox adapter must provide listbox/option semantics instead:

- stable option ids for `aria-activedescendant`
- `role="listbox"` and `role="option"` where appropriate
- highlighted option state even when the highlighted row is not mounted
- scroll-to-index integration
- disabled option lookup by index

Do not treat the current `virtual_list` wrapper as automatically accessible for combobox usage without this role/id integration.

The public virtualizer module is a low-level primitive API, not a complete accessible component. Consumers are responsible for roles, ids, keyboard behavior, focus management, and scroll container wiring.

## Higher-Level Component Composition

Expected composition model:

```text
use_combobox()
  owns dropdown, highlighted index, focus, registry, submit dispatch

Autocomplete
  owns String value/search and maps submit to set input label

Select
  local current component owns Option<T> value plus typeahead behavior; a future searchable Select would add query/search state

SelectMulti
  remains the existing select primitive/component path

MultiSelect
  should be added separately; owns Vec<T>, search, max-values, pill removal, and maps submit to toggle/add

TagsInput
  owns Vec<String>, parser, duplicate handling, search, and maps submit/Enter to add tag

PillsInput
  owns pill layout, keyboard removal, and input composition
```

## Implementation Waves

### Wave 1: Store Foundation

- Add `primitives/src/combobox/hook.rs`.
- Add public event-source/options/store types.
- Implement controlled/uncontrolled open state.
- Implement transition-aware open/close/toggle callbacks.
- Implement highlighted index state.
- Implement normal mounted option registry.
- Add unit tests for store-only behavior.

### Wave 2: Full Primitive Anatomy Integration

- Refactor `ComboboxContext` to carry or wrap `ComboboxStore`.
- Refactor internal `use_combobox_root` to initialize the store.
- Add or refactor primitive pieces for `ComboboxTarget`, `ComboboxEventsTarget`, `ComboboxDropdownTarget`, and `ComboboxSearch`.
- Update `ComboboxInput` to compose the appropriate target/events/search behavior instead of being the only target model.
- Add or rename `ComboboxOptions` if needed, while keeping `ComboboxList` as a compatibility alias if the current API already exposes it.
- Update `ComboboxOption` to register with the store.
- Keep root-level option submit handling in `ComboboxContext`; options should provide values, not own submit callbacks by default.
- Preserve existing component props and rendered attributes.

Wave 2 acceptance criteria:

- `ComboboxTarget`, `ComboboxEventsTarget`, `ComboboxDropdownTarget`, and `ComboboxSearch` can mount independently.
- events target and dropdown target can be different DOM nodes.
- `ComboboxInput` is composition sugar over the lower-level primitives, not the only target model.
- ARIA ids and `aria-activedescendant` wire correctly through split target/search/list anatomy.
- `ComboboxList` remains compatible or aliases `ComboboxOptions` without breaking existing users.
- existing option attributes and selected/highlighted behavior are preserved.
- new element-level hooks reuse the existing focus/listbox/pointer helper patterns where applicable.

### Wave 3: Compatibility Tests

- Add SSR/render tests for:
  - `aria-controls`
  - `aria-activedescendant`
  - `aria-selected`
  - `aria-disabled`
  - data attributes
  - empty state rendering
  - existing `Combobox<T>` value behavior
- Add behavior tests for:
  - disabled option skipping
  - invisible option skipping
  - loop and no-loop navigation
  - active option selection
  - dynamic registry updates
  - submit selected option
- Add targeted browser tests for:
  - keyboard navigation
  - pointer selection
  - focus and blur behavior
  - `focus_target()` and `focus_search_input()`
  - mounted dropdown behavior

### Wave 4: Higher-Level Components

After the primitive store is stable:

- build or refactor `Autocomplete` on top of `use_combobox()`
- refactor `Select` if needed
- keep existing `SelectMulti`
- add a separate `MultiSelect` on top of `use_combobox()`
- add `PillsInput`
- add `TagsInput`

Each component should own its own value/query model and use the store only for combobox interaction behavior.

### Wave 5: Virtualized Combobox

- Add a virtual registry/provider shape.
- Add `use_virtualized_combobox()`.
- Do not integrate combobox virtualization through the current public `VirtualList` component as-is.
- Reuse the public lower-level virtualizer algorithms.
- Add a combobox-specific virtualized listbox wrapper that renders listbox/option roles, stable option ids, active descendant wiring, and scroll-to-index behavior.
- Add examples for large option sets.

## Validation

Minimum validation before considering the primitive complete:

- targeted Rust tests for the new hook and context
- SSR/render tests for ARIA and data attributes
- targeted browser tests for focus, keyboard, pointer, blur, and mounted-node behavior
- existing combobox preview still works
- existing select/multi-select usage still compiles
- no public API break unless intentionally documented

## Decisions

- Introduce a public `use_disclosure()` hook for reusable controlled/uncontrolled open state, then use it from `use_combobox()` for dropdown state. `use_combobox()` should still own combobox-specific event-source semantics.
- Support explicit option order, with registration order as a fallback for simple mounted lists. Virtualized comboboxes must use explicit absolute indices.
- Defer `scroll_into_view` until mounted option scrolling and virtualizer scroll-to-index integration are implemented.
- Keep query state out of `use_combobox()`. Add a separate `use_autocomplete()` hook later if query/filter behavior repeats across components.
- Let virtualized support expose the same public combobox behavior surface, but allow a compatible wrapper type such as `VirtualizedComboboxStore` internally if the virtual registry/provider needs extra state.

## Open Decisions

None currently. Revisit after implementation discovery reveals new constraints.
