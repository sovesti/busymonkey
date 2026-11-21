use dioxus::prelude::*;

use shortcut::ShortuctSettingsView;

use persistence::provide_settings;
use registry::ScriptSettingsRegistry;
use shortcut::registry::ShortcutsRegistry;

pub(crate) mod duration;
pub(crate) mod float;
pub(crate) mod persistence;
pub(crate) mod registry;
pub(crate) mod scripts;
pub(crate) mod shortcut;

#[component]
pub fn SettingsView() -> Element {
    rsx! {
        ShortuctSettingsView {}
    }
}

pub fn provide_all_settings() {
    provide_settings::<ScriptSettingsRegistry>();
    // provide_settings::<ShortcutsRegistry>();
}
