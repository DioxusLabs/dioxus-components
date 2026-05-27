//! Proc macro for creating dynamic attribute lists in Dioxus
//!
//! This crate provides the `attributes!` macro which allows creating
//! `Vec<Attribute>` for use with element spread patterns.
//!
//! # Example
//!
//! ```rust,ignore
//! use dioxus::prelude::*;
//! use dioxus_attributes::attributes;
//!
//! fn MyComponent() -> Element {
//!     let attrs = attributes!(div {
//!         class: "my-class",
//!         "data-custom": "value",
//!         onclick: |_| println!("clicked"),
//!     });
//!
//!     rsx! {
//!         div { ..attrs }
//!     }
//! }
//! ```

use proc_macro::TokenStream;
use quote::ToTokens;

mod attribute_list;

use attribute_list::AttributeList;

/// Create a `Vec<Attribute>` from RSX attribute syntax.
///
/// The macro requires an element name followed by braced attributes.
/// This allows proper namespace and volatility lookup for attributes.
///
/// Accepts the same syntax as attributes inside rsx! elements:
/// - Built-in attributes: `class: "value"`
/// - Custom attributes: `"data-custom": value`
/// - Event handlers: `onclick: |_| {}`
/// - Shorthand attributes: `class,` (uses variable named `class`)
/// - Spreads: `..existing_attrs`
///
/// # Example
///
/// ```rust,ignore
/// let attrs = attributes!(button {
///     class: "btn btn-primary",
///     onclick: move |_| println!("clicked"),
///     "data-testid": "my-button",
/// });
///
/// rsx! {
///     button { ..attrs, "Click me" }
/// }
/// ```
#[proc_macro]
pub fn attributes(tokens: TokenStream) -> TokenStream {
    match syn::parse::<AttributeList>(tokens) {
        Ok(list) => list.into_token_stream().into(),
        Err(err) => err.to_compile_error().into(),
    }
}
