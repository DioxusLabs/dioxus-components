use dioxus::prelude::*;
use dioxus_icons::lucide::ChevronDown;
use dioxus_primitives::navbar::{
    self, NavbarContentProps, NavbarItemProps, NavbarNavProps, NavbarProps, NavbarTriggerProps,
};
#[css_module("/src/components/navbar/style.css")]
struct Styles;

#[component]
pub fn Navbar(props: NavbarProps) -> Element {
    rsx! {
        navbar::Navbar {
            class: Styles::dx_navbar,
            disabled: props.disabled,
            roving_loop: props.roving_loop,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn NavbarNav(props: NavbarNavProps) -> Element {
    rsx! {
        navbar::NavbarNav {
            class: Styles::dx_navbar_nav,
            index: props.index,
            disabled: props.disabled,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn NavbarTrigger(props: NavbarTriggerProps) -> Element {
    rsx! {
        navbar::NavbarTrigger { class: Styles::dx_navbar_trigger, attributes: props.attributes,
            {props.children}
            ChevronDown {
                class: Styles::dx_navbar_expand_icon,
                size: "20px",
                stroke: "var(--secondary-color-4)",
            }
        }
    }
}

#[component]
pub fn NavbarContent(props: NavbarContentProps) -> Element {
    rsx! {
        navbar::NavbarContent {
            class: Styles::dx_navbar_content,
            id: props.id,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn NavbarItem(props: NavbarItemProps) -> Element {
    rsx! {
        navbar::NavbarItem {
            class: Styles::dx_navbar_item.to_string(),
            index: props.index,
            value: props.value,
            disabled: props.disabled,
            new_tab: props.new_tab,
            to: props.to,
            active_class: props.active_class,
            attributes: props.attributes,
            on_select: props.on_select,
            onclick: props.onclick,
            onmounted: props.onmounted,
            {props.children}
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::components::navbar::{Navbar, NavbarContent, NavbarItem, NavbarNav, NavbarTrigger};
    use dioxus::prelude::*;
    use dioxus_test::{
        by_role, by_testid,
        matchers::{attribute, eq, has_focus, some},
        render, Result, Role,
    };

    #[tokio::test]
    async fn hovering_a_nav_opens_it() -> Result<()> {
        #[derive(Routable, Clone, PartialEq)]
        enum Route {
            #[route("/")]
            TestComponent {},
        }
        #[component]
        fn App() -> Element {
            rsx! {
                Router::<Route> {}
            }
        }
        #[component]
        fn TestComponent() -> Element {
            rsx! {
                Navbar { aria_label: "Components",
                    NavbarNav { index: 0usize, "data-testid": "inputs-nav",
                        NavbarTrigger { "Inputs" }
                        NavbarContent {
                            NavbarItem {
                                index: 0usize,
                                value: "calendar".to_string(),
                                to: Route::TestComponent {},
                                "Calendar"
                            }
                        }
                    }
                }
            }
        }
        let tester = render(App);

        tester.query(by_testid("inputs-nav")).hover().await?;

        tester
            .query(by_testid("inputs-nav"))
            .expect(attribute("data-state", some(eq("open"))))
            .await
    }

    #[tokio::test]
    async fn clicking_an_item_of_a_hovered_nav_navigates_to_its_target() -> Result<()> {
        #[derive(Routable, Clone, PartialEq)]
        enum Route {
            #[route("/")]
            TestComponent {},
            #[route("/calendar")]
            CalendarPage {},
        }
        #[component]
        fn App() -> Element {
            rsx! {
                Router::<Route> {}
            }
        }
        #[component]
        fn CalendarPage() -> Element {
            rsx! {
                div { "data-testid": "calendar-page" }
            }
        }
        #[component]
        fn TestComponent() -> Element {
            rsx! {
                Navbar { aria_label: "Components",
                    NavbarNav { index: 0usize, "data-testid": "inputs-nav",
                        NavbarTrigger { "Inputs" }
                        NavbarContent {
                            NavbarItem {
                                index: 0usize,
                                value: "calendar".to_string(),
                                to: Route::CalendarPage {},
                                "Calendar"
                            }
                        }
                    }
                }
            }
        }
        let tester = render(App);

        tester.query(by_testid("inputs-nav")).hover().await?;
        tester
            .query(by_role(Role::MenuItem).having_name(eq("Calendar")))
            .click()
            .await?;

        tester.query(by_testid("calendar-page")).await?;
        Ok(())
    }

    #[tokio::test]
    async fn tapping_a_trigger_opens_its_nav() -> Result<()> {
        #[derive(Routable, Clone, PartialEq)]
        enum Route {
            #[route("/")]
            TestComponent {},
        }
        #[component]
        fn App() -> Element {
            rsx! {
                Router::<Route> {}
            }
        }
        #[component]
        fn TestComponent() -> Element {
            rsx! {
                Navbar { aria_label: "Components",
                    NavbarNav { index: 0usize, "data-testid": "inputs-nav",
                        NavbarTrigger { "Inputs" }
                        NavbarContent {
                            NavbarItem {
                                index: 0usize,
                                value: "calendar".to_string(),
                                to: Route::TestComponent {},
                                "Calendar"
                            }
                        }
                    }
                }
            }
        }
        let tester = render(App);

        tester
            .query(by_role(Role::MenuItem).having_name(eq("Inputs")))
            .tap()
            .await?;

        tester
            .query(by_testid("inputs-nav"))
            .expect(attribute("data-state", some(eq("open"))))
            .await
    }

    #[tokio::test]
    #[ignore = "`tap()` in dioxus-test only dispatches `click`, but the trigger opens the nav on `pointerdown`"]
    async fn tapping_an_item_of_a_tapped_nav_navigates_to_its_target() -> Result<()> {
        #[derive(Routable, Clone, PartialEq)]
        enum Route {
            #[route("/")]
            TestComponent {},
            #[route("/calendar")]
            CalendarPage {},
        }
        #[component]
        fn App() -> Element {
            rsx! {
                Router::<Route> {}
            }
        }
        #[component]
        fn CalendarPage() -> Element {
            rsx! {
                div { "data-testid": "calendar-page" }
            }
        }
        #[component]
        fn TestComponent() -> Element {
            rsx! {
                Navbar { aria_label: "Components",
                    NavbarNav { index: 0usize,
                        NavbarTrigger { "Inputs" }
                        NavbarContent {
                            NavbarItem {
                                index: 0usize,
                                value: "calendar".to_string(),
                                to: Route::CalendarPage {},
                                "Calendar"
                            }
                        }
                    }
                }
            }
        }
        let tester = render(App);
        tester
            .query(by_role(Role::MenuItem).having_name(eq("Inputs")))
            .tap()
            .await?;

        tester
            .query(by_role(Role::MenuItem).having_name(eq("Calendar")))
            .tap()
            .await?;

        tester.query(by_testid("calendar-page")).await?;
        Ok(())
    }

    #[tokio::test]
    async fn arrow_right_moves_focus_to_the_next_nav() -> Result<()> {
        #[derive(Routable, Clone, PartialEq)]
        enum Route {
            #[route("/")]
            TestComponent {},
        }
        #[component]
        fn App() -> Element {
            rsx! {
                Router::<Route> {}
            }
        }
        #[component]
        fn TestComponent() -> Element {
            rsx! {
                Navbar { aria_label: "Components",
                    NavbarNav { index: 0usize,
                        NavbarTrigger { "Inputs" }
                        NavbarContent {
                            NavbarItem {
                                index: 0usize,
                                value: "calendar".to_string(),
                                to: Route::TestComponent {},
                                "Calendar"
                            }
                        }
                    }
                    NavbarNav { index: 1usize,
                        NavbarTrigger { "Information" }
                        NavbarContent {
                            NavbarItem {
                                index: 0usize,
                                value: "toast".to_string(),
                                to: Route::TestComponent {},
                                "Toast"
                            }
                        }
                    }
                }
            }
        }
        let tester = render(App);
        tester.query(by_role(Role::MenuBar)).focus().await?;

        tester.key_down(Key::Named(NamedKey::ArrowRight), Modifiers::empty())?;

        tester
            .query(by_role(Role::MenuItem).having_name(eq("Information")))
            .expect(has_focus())
            .await
    }

    #[tokio::test]
    async fn arrow_left_moves_focus_to_the_previous_nav() -> Result<()> {
        #[derive(Routable, Clone, PartialEq)]
        enum Route {
            #[route("/")]
            TestComponent {},
        }
        #[component]
        fn App() -> Element {
            rsx! {
                Router::<Route> {}
            }
        }
        #[component]
        fn TestComponent() -> Element {
            rsx! {
                Navbar { aria_label: "Components",
                    NavbarNav { index: 0usize,
                        NavbarTrigger { "Inputs" }
                        NavbarContent {
                            NavbarItem {
                                index: 0usize,
                                value: "calendar".to_string(),
                                to: Route::TestComponent {},
                                "Calendar"
                            }
                        }
                    }
                    NavbarNav { index: 1usize,
                        NavbarTrigger { "Information" }
                        NavbarContent {
                            NavbarItem {
                                index: 0usize,
                                value: "toast".to_string(),
                                to: Route::TestComponent {},
                                "Toast"
                            }
                        }
                    }
                }
            }
        }
        let tester = render(App);
        tester.query(by_role(Role::MenuBar)).focus().await?;
        tester.key_down(Key::Named(NamedKey::ArrowRight), Modifiers::empty())?;
        tester
            .query(by_role(Role::MenuItem).having_name(eq("Information")))
            .expect(has_focus())
            .await?;

        tester.key_down(Key::Named(NamedKey::ArrowLeft), Modifiers::empty())?;

        tester
            .query(by_role(Role::MenuItem).having_name(eq("Inputs")))
            .expect(has_focus())
            .await
    }

    #[tokio::test]
    async fn arrow_down_on_a_nav_focuses_its_first_item() -> Result<()> {
        #[derive(Routable, Clone, PartialEq)]
        enum Route {
            #[route("/")]
            TestComponent {},
        }
        #[component]
        fn App() -> Element {
            rsx! {
                Router::<Route> {}
            }
        }
        #[component]
        fn TestComponent() -> Element {
            rsx! {
                Navbar { aria_label: "Components",
                    NavbarNav { index: 0usize,
                        NavbarTrigger { "Inputs" }
                        NavbarContent {
                            NavbarItem {
                                index: 0usize,
                                value: "calendar".to_string(),
                                to: Route::TestComponent {},
                                "Calendar"
                            }
                            NavbarItem {
                                index: 1usize,
                                value: "checkbox".to_string(),
                                to: Route::TestComponent {},
                                "Checkbox"
                            }
                        }
                    }
                }
            }
        }
        let tester = render(App);
        tester.query(by_role(Role::MenuBar)).focus().await?;
        tester
            .query(by_role(Role::MenuItem).having_name(eq("Inputs")))
            .expect(has_focus())
            .await?;

        tester.key_down(Key::Named(NamedKey::ArrowDown), Modifiers::empty())?;

        tester
            .query(by_role(Role::MenuItem).having_name(eq("Calendar")))
            .expect(has_focus())
            .await
    }

    #[tokio::test]
    async fn arrow_down_skips_disabled_item() -> Result<()> {
        #[derive(Routable, Clone, PartialEq)]
        enum Route {
            #[route("/")]
            TestComponent {},
        }
        #[component]
        fn App() -> Element {
            rsx! {
                Router::<Route> {}
            }
        }
        #[component]
        fn TestComponent() -> Element {
            rsx! {
                Navbar { aria_label: "Components",
                    NavbarNav { index: 0usize,
                        NavbarTrigger { "Inputs" }
                        NavbarContent {
                            NavbarItem {
                                index: 0usize,
                                value: "calendar".to_string(),
                                to: Route::TestComponent {},
                                "Calendar"
                            }
                            NavbarItem {
                                index: 1usize,
                                value: "slider".to_string(),
                                to: Route::TestComponent {},
                                disabled: true,
                                "Slider"
                            }
                            NavbarItem {
                                index: 2usize,
                                value: "checkbox".to_string(),
                                to: Route::TestComponent {},
                                "Checkbox"
                            }
                        }
                    }
                }
            }
        }
        let tester = render(App);
        tester.query(by_role(Role::MenuBar)).focus().await?;
        tester
            .query(by_role(Role::MenuItem).having_name(eq("Inputs")))
            .expect(has_focus())
            .await?;
        tester.key_down(Key::Named(NamedKey::ArrowDown), Modifiers::empty())?;
        tester
            .query(by_role(Role::MenuItem).having_name(eq("Calendar")))
            .expect(has_focus())
            .await?;

        tester.key_down(Key::Named(NamedKey::ArrowDown), Modifiers::empty())?;

        tester
            .query(by_role(Role::MenuItem).having_name(eq("Checkbox")))
            .expect(has_focus())
            .await
    }

    #[tokio::test]
    async fn disabled_item_is_marked_as_disabled() -> Result<()> {
        #[derive(Routable, Clone, PartialEq)]
        enum Route {
            #[route("/")]
            TestComponent {},
        }
        #[component]
        fn App() -> Element {
            rsx! {
                Router::<Route> {}
            }
        }
        #[component]
        fn TestComponent() -> Element {
            rsx! {
                Navbar { aria_label: "Components",
                    NavbarNav { index: 0usize,
                        NavbarTrigger { "Inputs" }
                        NavbarContent {
                            NavbarItem {
                                index: 0usize,
                                value: "slider".to_string(),
                                to: Route::TestComponent {},
                                disabled: true,
                                "Slider"
                            }
                        }
                    }
                }
            }
        }
        let tester = render(App);
        tester.query(by_role(Role::MenuBar)).focus().await?;
        tester
            .query(by_role(Role::MenuItem).having_name(eq("Inputs")))
            .expect(has_focus())
            .await?;

        tester.key_down(Key::Named(NamedKey::ArrowDown), Modifiers::empty())?;

        tester
            .query(by_role(Role::MenuItem).having_name(eq("Slider")))
            .expect(attribute("data-disabled", some(eq("true"))))
            .await
    }

    #[tokio::test]
    async fn enter_on_the_focused_item_navigates_to_its_target() -> Result<()> {
        #[derive(Routable, Clone, PartialEq)]
        enum Route {
            #[route("/")]
            TestComponent {},
            #[route("/checkbox")]
            CheckboxPage {},
        }
        #[component]
        fn App() -> Element {
            rsx! {
                Router::<Route> {}
            }
        }
        #[component]
        fn CheckboxPage() -> Element {
            rsx! {
                div { "data-testid": "checkbox-page" }
            }
        }
        #[component]
        fn TestComponent() -> Element {
            rsx! {
                Navbar { aria_label: "Components",
                    NavbarNav { index: 0usize,
                        NavbarTrigger { "Inputs" }
                        NavbarContent {
                            NavbarItem {
                                index: 0usize,
                                value: "checkbox".to_string(),
                                to: Route::CheckboxPage {},
                                "Checkbox"
                            }
                        }
                    }
                }
            }
        }
        let tester = render(App);
        tester.query(by_role(Role::MenuBar)).focus().await?;
        tester
            .query(by_role(Role::MenuItem).having_name(eq("Inputs")))
            .expect(has_focus())
            .await?;
        tester.key_down(Key::Named(NamedKey::ArrowDown), Modifiers::empty())?;
        tester
            .query(by_role(Role::MenuItem).having_name(eq("Checkbox")))
            .expect(has_focus())
            .await?;

        tester.key_down(Key::Named(NamedKey::Enter), Modifiers::empty())?;

        tester.query(by_testid("checkbox-page")).await?;
        Ok(())
    }
}
