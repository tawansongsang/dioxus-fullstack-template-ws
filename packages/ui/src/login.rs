use dioxus::prelude::*;

use crate::components::card::{
    Card, CardAction, CardContent, CardDescription, CardFooter, CardHeader, CardTitle,
};

const CARD_CSS: Asset = asset!("/assets/dx-components-theme.css");

#[component]
pub fn Login() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: CARD_CSS }

        Card { style: "width: 100%; max-width: 24rem;",
            CardHeader {
                CardTitle { "Login to your account" }
                CardDescription { "Enter your email below to login to your account" }
                CardAction {
                    button { "Sign up" }
                }
            }
            CardContent {
                form {
                    div { style: "display: flex; flex-direction: column; gap: 1.5rem;",
                        div { style: "display: grid; gap: 0.5rem;",
                            label { r#for: "email", "Email" }
                            input {
                                id: "email",
                                r#type: "email",
                                placeholder: "m@example.com",
                            }
                        }
                        div { style: "display: grid; gap: 0.5rem;",
                            div { style: "display: flex; align-items: center;",
                                label { r#for: "password", "Password" }
                                a {
                                    href: "#",
                                    style: "margin-left: auto; font-size: 0.875rem; color: var(--secondary-color-5); text-decoration: underline; text-underline-offset: 4px;",
                                    "Forgot your password?"
                                }
                            }
                            input { id: "password", r#type: "password" }
                        }
                    }
                }
            }
            CardFooter { style: "flex-direction: column; gap: 0.5rem;",
                button { r#type: "submit", style: "width: 100%;", "Login" }
                button { style: "width: 100%;", "Login with Google" }
            }
        }
    }
}
