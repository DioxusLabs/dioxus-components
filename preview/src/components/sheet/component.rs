use dioxus::prelude::*;
use dioxus_icons::lucide::X;
use dioxus_primitives::dialog::{
    self, DialogCtx, DialogDescriptionProps, DialogRootProps, DialogTitleProps,
};
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::merge_attributes;

#[css_module("/src/components/sheet/style.css")]
struct Styles;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum SheetSide {
    Top,
    #[default]
    Right,
    Bottom,
    Left,
}

impl SheetSide {
    pub fn as_str(&self) -> &'static str {
        match self {
            SheetSide::Top => "top",
            SheetSide::Right => "right",
            SheetSide::Bottom => "bottom",
            SheetSide::Left => "left",
        }
    }
}

#[component]
pub fn Sheet(props: DialogRootProps) -> Element {
    let content_base = attributes!(div {
        class: Styles::dx_sheet,
        "data-slot": "sheet-content",
        "data-side": SheetSide::Right.as_str(),
    });
    let content_attributes = merge_attributes(vec![content_base, props.attributes]);

    rsx! {
        dialog::DialogRoot {
            class: Styles::dx_sheet_root,
            "data-slot": "sheet-root",
            id: props.id,
            is_modal: props.is_modal,
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            dialog::DialogContent {
                class: None,
                attributes: content_attributes,
                {props.children}
            }
        }
    }
}

#[component]
pub fn SheetContentClose(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let base = attributes!(button {
        class: Styles::dx_sheet_close,
    });
    let attributes = merge_attributes(vec![base, attributes]);

    rsx! {
        SheetClose { attributes,
            X { size: "20px" }
        }
    }
}

#[component]
pub fn SheetHeader(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div { class: Styles::dx_sheet_header, "data-slot": "sheet-header", ..attributes, {children} }
    }
}

#[component]
pub fn SheetFooter(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div { class: Styles::dx_sheet_footer, "data-slot": "sheet-footer", ..attributes, {children} }
    }
}

#[component]
pub fn SheetTitle(props: DialogTitleProps) -> Element {
    rsx! {
        dialog::DialogTitle {
            id: props.id,
            class: Styles::dx_sheet_title,
            "data-slot": "sheet-title",
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn SheetDescription(props: DialogDescriptionProps) -> Element {
    rsx! {
        dialog::DialogDescription {
            id: props.id,
            class: Styles::dx_sheet_description,
            "data-slot": "sheet-description",
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn SheetClose(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    r#as: Option<Callback<Vec<Attribute>, Element>>,
    children: Element,
) -> Element {
    let ctx: DialogCtx = use_context();

    let base = attributes! {
        button {
            onclick: move |_| {
                ctx.set_open(false);
            }
        }
    };
    let merged = merge_attributes(vec![base, attributes]);

    if let Some(dynamic) = r#as {
        dynamic.call(merged)
    } else {
        rsx! {
            button { ..merged, {children} }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::components::sheet::{Sheet, SheetContentClose, SheetSide};
    use dioxus::prelude::*;
    use dioxus_test::{
        by_testid,
        matchers::{attribute, contains_substring, eq, has_focus, inner_html, some},
        render, Result,
    };

    #[tokio::test]
    async fn sheet_is_initially_closed() -> Result<()> {
        #[component]
        fn TestComponent() -> Element {
            rsx! {
                Sheet {
                    open: false,
                    "data-side": SheetSide::Top.as_str(),
                    div {
                        "Sheet content"
                    }
                }
            }
        }
        let tester = render(TestComponent);

        tester
            .query(r#"[data-slot="sheet-root"]"#)
            .expect_no_matching_element()
            .await
    }

    #[tokio::test]
    async fn sheet_opens_when_requested() -> Result<()> {
        #[component]
        fn TestComponent() -> Element {
            let mut open = use_signal(|| false);
            rsx! {
                button {
                    "data-testid": "open-button",
                    onclick: move |_| {
                        open.set(true);
                    }
                }
                Sheet {
                    open: open(),
                    on_open_change: move |v| open.set(v),
                    "data-side": SheetSide::Top.as_str(),
                    div {
                        "Sheet content"
                    }
                }
            }
        }
        let tester = render(TestComponent);

        tester.query(by_testid("open-button")).click().await?;

        tester
            .query(r#"[data-slot="sheet-root"]"#)
            .expect(attribute("data-state", some(eq("open"))))
            .await?;
        tester
            .query(r#"[data-slot="sheet-root"]"#)
            .expect(inner_html(contains_substring("Sheet content")))
            .await
    }

    #[tokio::test]
    async fn sheet_closes_when_closee_button_clicked() -> Result<()> {
        #[component]
        fn TestComponent() -> Element {
            let mut open = use_signal(|| false);
            rsx! {
                button {
                    "data-testid": "open-button",
                    onclick: move |_| {
                        open.set(true);
                    }
                }
                Sheet {
                    open: open(),
                    on_open_change: move |v| open.set(v),
                    "data-side": SheetSide::Top.as_str(),
                    SheetContentClose {
                        "data-testid": "close-button",
                    }
                }
            }
        }
        let tester = render(TestComponent);
        tester.query(by_testid("open-button")).click().await?;

        tester.query(by_testid("close-button")).click().await?;

        tester
            .query(r#"[data-slot="sheet-root"]"#)
            .expect_no_matching_element()
            .await
    }

    #[tokio::test]
    #[ignore = "Currently fails because the listener for the escape key is implemented in JavaScript, so is not available in the native stack"]
    async fn sheet_closes_when_escape_pressed() -> Result<()> {
        #[component]
        fn TestComponent() -> Element {
            let mut open = use_signal(|| false);
            rsx! {
                button {
                    "data-testid": "open-button",
                    onclick: move |_| {
                        open.set(true);
                    }
                }
                Sheet {
                    open: open(),
                    on_open_change: move |v| open.set(v),
                    "data-side": SheetSide::Top.as_str(),
                    input {
                        "data-testid": "first-input",
                    }
                    SheetContentClose {
                        "data-testid": "close-button",
                    }
                }
            }
        }
        let tester = render(TestComponent);
        tester.query(by_testid("open-button")).click().await?;

        tester.key_down(Key::Named(NamedKey::Escape), Modifiers::empty())?;

        tester
            .query(r#"[data-slot="sheet-root"]"#)
            .expect_no_matching_element()
            .await
    }

    #[tokio::test]
    #[ignore = "Currently fails because the focus trap is implemented in JavaScript, so it's unavailable in the native stack"]
    async fn first_input_is_focused_when_sheet_opens() -> Result<()> {
        let tester = render(SheetWithThreeInputsAndCloseButton);

        tester.query(by_testid("open-button")).click().await?;

        tester
            .query(by_testid("first-input"))
            .expect(has_focus())
            .await
    }

    #[tokio::test]
    async fn pressing_tab_puts_focus_on_second_element() -> Result<()> {
        let tester = render(SheetWithThreeInputsAndCloseButton);
        tester.query(by_testid("open-button")).click().await?;
        // Focus trap is not available in the native stack, so we must set the focus manually.
        tester.query(by_testid("first-input")).focus().await?;

        tester.key_down(Key::Named(NamedKey::Tab), Modifiers::empty())?;

        tester
            .query(by_testid("second-input"))
            .expect(has_focus())
            .await
    }

    #[tokio::test]
    async fn pressing_tab_twice_puts_focus_on_third_element() -> Result<()> {
        let tester = render(SheetWithThreeInputsAndCloseButton);
        tester.query(by_testid("open-button")).click().await?;
        // Focus trap is not available in the native stack, so we must set the focus manually.
        tester.query(by_testid("first-input")).focus().await?;

        tester.key_down(Key::Named(NamedKey::Tab), Modifiers::empty())?;
        tester.key_down(Key::Named(NamedKey::Tab), Modifiers::empty())?;

        tester
            .query(by_testid("third-input"))
            .expect(has_focus())
            .await
    }

    #[tokio::test]
    async fn pressing_tab_thrice_puts_focus_on_close_button() -> Result<()> {
        let tester = render(SheetWithThreeInputsAndCloseButton);
        tester.query(by_testid("open-button")).click().await?;
        // Focus trap is not available in the native stack, so we must set the focus manually.
        tester.query(by_testid("first-input")).focus().await?;

        tester.key_down(Key::Named(NamedKey::Tab), Modifiers::empty())?;
        tester.key_down(Key::Named(NamedKey::Tab), Modifiers::empty())?;
        tester.key_down(Key::Named(NamedKey::Tab), Modifiers::empty())?;

        tester
            .query(by_testid("close-button"))
            .expect(has_focus())
            .await
    }

    #[tokio::test]
    #[ignore = "Currently fails because the focus trap is implemented in JavaScript, so it's unavailable in the native stack"]
    async fn pressing_tab_four_times_puts_focus_back_on_first_element() -> Result<()> {
        let tester = render(SheetWithThreeInputsAndCloseButton);
        tester.query(by_testid("open-button")).click().await?;

        tester.key_down(Key::Named(NamedKey::Tab), Modifiers::empty())?;
        tester.key_down(Key::Named(NamedKey::Tab), Modifiers::empty())?;
        tester.key_down(Key::Named(NamedKey::Tab), Modifiers::empty())?;
        tester.key_down(Key::Named(NamedKey::Tab), Modifiers::empty())?;

        tester
            .query(by_testid("first-input"))
            .expect(has_focus())
            .await
    }

    #[component]
    fn SheetWithThreeInputsAndCloseButton() -> Element {
        let mut open = use_signal(|| false);
        rsx! {
            button {
                "data-testid": "open-button",
                onclick: move |_| {
                    open.set(true);
                }
            }
            Sheet {
                open: open(),
                on_open_change: move |v| open.set(v),
                "data-side": SheetSide::Top.as_str(),
                input {
                    "data-testid": "first-input",
                }
                input {
                    "data-testid": "second-input",
                }
                input {
                    "data-testid": "third-input",
                }
                SheetContentClose {
                    "data-testid": "close-button",
                }
            }
        }
    }
}
