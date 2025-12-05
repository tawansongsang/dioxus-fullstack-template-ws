use dioxus::prelude::*;
use ui::Login;

#[component]
pub fn LoginWeb() -> Element {
    rsx! {
        Login {}
    }
}
