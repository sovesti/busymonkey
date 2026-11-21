use dioxus::prelude::*;

use super::persistence::{use_save_settings, use_settings};
use super::registry::ScriptSettingsRegistry;
use super::{duration, float};
use crate::client::components::{Button, ButtonVariant};
use crate::client::settings::registry::ScriptSettingsRegistryStoreImplExt;
use crate::client::settings::shortcut::registry::{ShortcutAction, use_shortcut_keys};
use crate::monkey::model::parameters::{Parameter, ParameterSchema};

#[component]
pub fn ScriptSettingsView() -> Element {
    let mut save = use_save_settings::<ScriptSettingsRegistry>();
    rsx! {
        if let Some(mut registry) = *use_settings::<ScriptSettingsRegistry>().read() {
            form {
                class: "flex flex-col w-64 bg-inherit items-start gap-4",
                onsubmit: move |evt: FormEvent| async move {
                    evt.prevent_default();
                    registry.store_settings(evt.data());
                    save.call();
                },
                ScriptTitle { registry }
                ScriptHint {}
                ScriptParameterControls { registry }
                Button {
                    variant: ButtonVariant::Solid,
                    class: "self-center",
                    r#type: "submit",
                    "Save"
                }
            }
        }
        if let Some(Err(err)) = save.value() {
            p { class: "text-red", "{err}" }
        }
    }
}

#[component]
fn ScriptTitle(registry: Store<ScriptSettingsRegistry>) -> Element {
    rsx! {
        if let Some(factory) = registry.chosen_factory() {
            p { class: "text-xl", {factory.create().label()} }
        }
    }
}

#[component]
fn ScriptHint() -> Element {
    let keys = use_shortcut_keys(ShortcutAction::ToggleScript);
    rsx! {
        if let Some(keys) = keys() {
            p { class: "text-overlay2", "Hint: press {keys} to toggle script" }
        }
    }
}

#[component]
fn ScriptParameterControls(registry: ReadStore<ScriptSettingsRegistry>) -> Element {
    rsx! {
        if let Some(factory) = registry.chosen_factory() {
            for param in factory.create().iter() {
                ParameterControl { initial: registry.current_value(&param.name), param }
            }
        }
    }
}

#[component]
fn ParameterControl(param: Parameter, initial: serde_json::Value) -> Element {
    let name = &param.name;
    rsx! {
        div { class: "flex flex-col gap-1 min-w-full",
            match param.schema {
                ParameterSchema::Float(schema) => {
                    rsx! {
                        float::Control {
                            name,
                            title: param.label,
                            unit: param.unit,
                            initial,
                            schema,
                        }
                    }
                }
                ParameterSchema::Duration(schema) => {
                    rsx! {
                        duration::Control {
                            name,
                            title: param.label,
                            initial,
                            schema,
                        }
                    }
                }
            }
        }
    }
}
