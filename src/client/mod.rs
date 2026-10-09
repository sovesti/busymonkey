mod auth;
mod scripts;

use dioxus::prelude::*;

use auth::AuthView;
use scripts::ScriptsView;

pub fn app() -> Element {
    let authorized = use_signal(|| false);
    rsx! {
        document::Link { rel: "icon", href: asset!("/assets/favicon.ico") },
        Stylesheet { href: asset!("/assets/tailwind.css") },
        main {
            div {
                class: "min-h-screen p-4",
                if authorized() {
                    ScriptsView {},
                } else {
                    AuthView { authorized }
                }
            }
        }
    }
}
