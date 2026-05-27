use super::super::component::*;
use crate::components::button::{Button, ButtonVariant};
use crate::components::input::Input;
use crate::components::label::Label;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        Card { style: "width: 100%; max-width: 24rem;",
            CardHeader {
                CardTitle { "Login to your account" }
                CardDescription { "Enter your email below to login to your account" }
                CardAction {
                    Button { variant: ButtonVariant::Ghost, "Sign Up" }
                }
            }
            CardContent {
                form { id: "login-form",
                    div { style: "display: flex; flex-direction: column; gap: 1.5rem;",
                        div { style: "display: grid; gap: 0.5rem;",
                            Label { html_for: "email", "Email" }
                            Input {
                                id: "email",
                                r#type: "email",
                                placeholder: "m@example.com",
                            }
                        }
                        div { style: "display: grid; gap: 0.5rem;",
                            div { style: "display: flex; align-items: center;",
                                Label { html_for: "password", "Password" }
                                a {
                                    href: "#",
                                    style: "margin-left: auto; font-size: 0.875rem; color: var(--secondary-color-5); text-decoration: underline; text-underline-offset: 4px;",
                                    "Forgot your password?"
                                }
                            }
                            Input { id: "password", r#type: "password" }
                        }
                    }
                }
            }
            CardFooter { style: "flex-direction: column; gap: 0.5rem;",
                Button {
                    r#type: "submit",
                    form: "login-form",
                    style: "width: 100%;",
                    "Login"
                }
                Button { variant: ButtonVariant::Outline, style: "width: 100%;", "Login with Google" }
            }
        }
    }
}
