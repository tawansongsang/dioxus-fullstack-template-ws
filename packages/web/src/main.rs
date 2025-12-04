use dioxus::prelude::*;

use routers::Route;
use ui::Navbar;

mod routers;
mod views;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

// fn main() {
//     dioxus::launch(App);
// }

fn main() {
    #[cfg(not(feature = "server"))]
    dioxus::launch(App);

    #[cfg(feature = "server")]
    dioxus::serve(|| async move {
        // Create a new axum router for our Dioxus app

        use db::{sqlserver::get_db_pool, surrealdb};

        let router = dioxus::server::router(App);

        // .. customize it however you want ..
        surrealdb::connection_db().await.unwrap();
        let _sqlserverdb = get_db_pool().await.unwrap();

        // And then return it
        Ok(router)
    })
}

#[component]
fn App() -> Element {
    // Build cool things ✌️

    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        Router::<Route> {}
    }
}

/// A web-specific Router around the shared `Navbar` component
/// which allows us to use the web-specific `Route` enum.
#[component]
fn WebNavbar() -> Element {
    rsx! {
        Navbar {
            Link { to: Route::Home {}, "Home" }
            Link { to: Route::Blog { id: 1 }, "Blog" }
        }

        Outlet::<Route> {}
    }
}
