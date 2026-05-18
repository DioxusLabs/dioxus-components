use dioxus::prelude::*;
use dioxus_kit_core::dialog::{self, DialogDescriptionProps, DialogRootProps, DialogTitleProps};
use dioxus_kit_core::{dioxus_attributes::attributes, merge_attributes};

/// A styled close button intended for use inside a [`Dialog`]. Renders a
/// `button` element pre-wired with the close icon styling, default
/// `type="button"`, and an accessible label.
#[component]
pub fn DialogClose(
    #[props(extends = GlobalAttributes)]
    #[props(extends = button)]
    attributes: Vec<Attribute>,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let base = attributes!(button {
        class: Styles::dx_dialog_close,
        r#type: "button",
        aria_label: "Close",
    });
    let merged = merge_attributes(vec![base, attributes]);
    rsx! {
        button {
            onclick: move |event| {
                if let Some(f) = &onclick {
                    f.call(event);
                }
            },
            ..merged,
            {children}
        }
    }
}

#[css_module("/src/dialog/style.css")]
struct Styles;

#[component]
pub fn Dialog(props: DialogRootProps) -> Element {
    let base = attributes!(div {
        class: Styles::dx_dialog,
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        dialog::DialogRoot {
            class: Styles::dx_dialog_backdrop,
            id: props.id,
            is_modal: props.is_modal,
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            dialog::DialogContent {
                class: None,
                attributes: merged,
                {props.children}
            }
        }
    }
}

#[component]
pub fn DialogTitle(props: DialogTitleProps) -> Element {
    let base = attributes!(h2 {
        class: Styles::dx_dialog_title,
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        dialog::DialogTitle {
            id: props.id,
            attributes: merged,
            {props.children}
        }
    }
}

#[component]
pub fn DialogDescription(props: DialogDescriptionProps) -> Element {
    let base = attributes!(p {
        class: Styles::dx_dialog_description,
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        dialog::DialogDescription {
            id: props.id,
            attributes: merged,
            {props.children}
        }
    }
}
