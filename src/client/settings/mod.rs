use dioxus::prelude::*;

use shortcut::ShortuctSettingsView;

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
