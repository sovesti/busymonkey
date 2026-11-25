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
    use_tray();
    use_browser();
    app()
}

fn use_tray() {
    match tray::use_tray() {
        Ok(()) => {}
        Err(err) => error!("failed to initialize tray icon: {err:?}"),
    }
}
