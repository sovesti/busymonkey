use dioxus::prelude::*;

use persistence::SettingsCategory;
use registry::ScriptSettingsRegistry;
use shortcut::ShortuctSettingsView;

use crate::client::templates::persistence::PersistentRecordedScript;

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

#[component]
pub fn SettingsProvider(children: Element) -> Element {
    let settings = use_resource(ScriptSettingsRegistry::load_or_default);
    use_context_provider(|| settings);
    let settings = use_resource(PersistentRecordedScript::load_or_default);
    use_context_provider(|| settings);
    children
}
