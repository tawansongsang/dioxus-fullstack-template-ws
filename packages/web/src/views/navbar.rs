use dioxus::prelude::*;

use ui::Navbar;

use crate::routers::Route;

/// A web-specific Router around the shared `Navbar` component
/// which allows us to use the web-specific `Route` enum.
#[component]
pub fn WebNavbar() -> Element {
    rsx! {
        Navbar {
            Link { to: Route::Home {}, "Home" }
            Link { to: Route::Blog { id: 1 }, "Blog" }
            Link { to: Route::LoginWeb {}, "Login" }
        }

        Outlet::<Route> {}
    }
}
