use dioxus::prelude::*;

use crate::views::{Blog, Home, LoginWeb, WebNavbar};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(WebNavbar)]
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
    #[route("/login")]
    LoginWeb { }
}
