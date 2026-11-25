#[allow(unused)]
pub(crate) mod browser;
pub(crate) mod settings;
mod tray;

use dioxus::prelude::*;

use crate::client::app;
use browser::use_browser;

pub fn launch() {
    tray::launch(AppInTray);
}

#[component]
fn AppInTray() -> Element {
    try_init();
    use_browser();
    app()
}

fn try_init() {
    match tray::init() {
        Ok(()) => {}
        Err(err) => error!("failed to initialize tray icon: {err:?}"),
    }
}
