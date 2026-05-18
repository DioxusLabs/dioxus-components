//! Shared option selection helpers.

use dioxus::prelude::{Signal, WritableExt};
use std::{any::Any, rc::Rc};

trait DynPartialEq: Any {
    fn eq(&self, other: &dyn Any) -> bool;
}

impl<T: PartialEq + 'static> DynPartialEq for T {
    fn eq(&self, other: &dyn Any) -> bool {
        other.downcast_ref::<T>() == Some(self)
    }
}

/// Type-erased value that still supports equality.
#[derive(Clone)]
pub(crate) struct RcPartialEqValue {
    value: Rc<dyn DynPartialEq>,
}

impl RcPartialEqValue {
    /// Create a new type-erased value.
    pub(crate) fn new<T: PartialEq + 'static>(value: T) -> Self {
        Self {
            value: Rc::new(value),
        }
    }

    /// Borrow this value as [`Any`].
    pub(crate) fn as_any(&self) -> &dyn Any {
        (&*self.value) as &dyn Any
    }

    /// Downcast this value to its concrete type.
    pub(crate) fn as_ref<T: PartialEq + 'static>(&self) -> Option<&T> {
        self.as_any().downcast_ref::<T>()
    }
}

impl PartialEq for RcPartialEqValue {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&*other.value)
    }
}

/// Registered option metadata shared by select-like components.
#[derive(PartialEq)]
pub(crate) struct OptionState {
    /// Focus/navigation index.
    pub(crate) tab_index: usize,
    /// Programmatic option value.
    pub(crate) value: RcPartialEqValue,
    /// Display/search text.
    pub(crate) text_value: String,
    /// DOM id.
    pub(crate) id: String,
    /// Whether this option is disabled.
    pub(crate) disabled: bool,
}

/// Resolve an option's searchable text value.
pub(crate) fn option_text_value<T: 'static>(
    value: &T,
    text_value: Option<String>,
    component_name: &str,
) -> String {
    text_value.unwrap_or_else(|| {
        let as_any: &dyn Any = value;
        as_any
            .downcast_ref::<String>()
            .cloned()
            .or_else(|| as_any.downcast_ref::<&str>().map(|s| s.to_string()))
            .unwrap_or_else(|| {
                tracing::warn!(
                    "{component_name} with non-string types requires text_value to be set"
                );
                String::new()
            })
    })
}

/// Display text for selected values in selection order.
pub(crate) fn selected_text<'a>(
    values: impl IntoIterator<Item = &'a RcPartialEqValue>,
    options: &[OptionState],
) -> Option<String> {
    let parts: Vec<String> = values
        .into_iter()
        .filter_map(|value| {
            options
                .iter()
                .find(|option| &option.value == value)
                .map(|option| option.text_value.clone())
        })
        .collect();

    (!parts.is_empty()).then(|| parts.join(", "))
}

/// Insert or update a registered option.
pub(crate) fn sync_option(mut options: Signal<Vec<OptionState>>, option_state: OptionState) {
    sync_option_state(&mut options.write(), option_state);
}

fn sync_option_state(options: &mut Vec<OptionState>, option_state: OptionState) {
    if let Some(position) = options
        .iter()
        .position(|option| option.id == option_state.id)
    {
        if options[position].tab_index == option_state.tab_index {
            options[position] = option_state;
        } else {
            options.remove(position);
            insert_option(options, option_state);
        }
    } else {
        insert_option(options, option_state);
    }
}

fn insert_option(options: &mut Vec<OptionState>, option_state: OptionState) {
    let insert_at = options.partition_point(|option| option.tab_index <= option_state.tab_index);
    options.insert(insert_at, option_state);
}

/// Remove a registered option by id.
pub(crate) fn remove_option(mut options: Signal<Vec<OptionState>>, id: &str) {
    options.write().retain(|option| option.id != id);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn option(id: &str, tab_index: usize) -> OptionState {
        OptionState {
            tab_index,
            value: RcPartialEqValue::new(id.to_string()),
            text_value: id.to_string(),
            id: id.to_string(),
            disabled: false,
        }
    }

    fn ids(options: &[OptionState]) -> Vec<&str> {
        options.iter().map(|option| option.id.as_str()).collect()
    }

    fn tab_indices(options: &[OptionState]) -> Vec<usize> {
        options.iter().map(|option| option.tab_index).collect()
    }

    #[test]
    fn sync_option_state_repositions_when_tab_index_changes() {
        let mut options = vec![option("a", 0), option("b", 1), option("c", 2)];

        sync_option_state(&mut options, option("a", 3));

        assert_eq!(ids(&options), ["b", "c", "a"]);
        assert_eq!(tab_indices(&options), [1, 2, 3]);
    }

    #[test]
    fn sync_option_state_keeps_sorted_order_when_tab_index_decreases() {
        let mut options = vec![option("a", 0), option("b", 1), option("c", 2)];

        sync_option_state(&mut options, option("c", 0));

        assert_eq!(ids(&options), ["a", "c", "b"]);
        assert_eq!(tab_indices(&options), [0, 0, 1]);
    }
}
