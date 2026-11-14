mod auth;
mod components;
#[cfg(feature = "desktop")]
pub(crate) mod desktop;
mod scripts;
mod settings;
mod templates;
mod views;

use dioxus::prelude::*;
use views::ViewsInSidebar;

use crate::client::settings::{persistence::provide_settings, registry::ScriptSettingsRegistry};
use crate::monkey::record::provide_input_channel;

pub fn app() -> Element {
    provide_settings::<ScriptSettingsRegistry>();
    provide_input_channel();
    rsx! {
        document::Link { rel: "icon", href: asset!("/assets/favicon.ico") }
        Stylesheet { href: asset!("/assets/dx-components-theme.css") }
        Stylesheet { href: asset!("/assets/main.css") }
        Stylesheet { href: asset!("/assets/tailwind.css") }
        main {
            div {
                class: "min-h-screen bg-base p-4",
                ViewsInSidebar {}
            }
        }
    }
}
