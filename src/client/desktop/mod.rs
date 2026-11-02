pub(crate) mod settings;
mod tray;

use dioxus::prelude::*;

use crate::client::app;

pub fn launch() {
    tray::launch(AppInTray);
}

#[component]
fn AppInTray() -> Element {
    try_init();
    app()
}

fn try_init() {
    match tray::init() {
        Ok(()) => {}
        Err(err) => error!("failed to initialize tray icon: {err:?}"),
    }
}
