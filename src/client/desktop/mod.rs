pub(crate) mod browser;
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
    use_future(|| async {
        match browser::serve().await {
            Err(err) => error!("error initialising server for communication with extension: {err:?}"),
            Ok(()) => {}
        }
    });
    app()
}

fn try_init() {
    match tray::init() {
        Ok(()) => {}
        Err(err) => error!("failed to initialize tray icon: {err:?}"),
    }
}
