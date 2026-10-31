use dioxus::prelude::*;

use super::persistence::{use_save_settings, use_settings};
use super::registry::ScriptSettingsRegistry;
use super::{duration, float};
use crate::client::components::Button;
use crate::monkey::model::parameters::{Parameter, ParameterSchema};

#[component]
pub fn ScriptSettingsView() -> Element {
    let mut save = use_save_settings::<ScriptSettingsRegistry>();
    rsx! {
        if let Some(mut registry) = *use_settings::<ScriptSettingsRegistry>().read() {
            form {
                class: "w-64 bg-inherit",
                onsubmit: move |evt: FormEvent| async move {
                    evt.prevent_default();
                    registry.write().store_settings(evt.data());
                    save.call();
                },
                if let Some(factory) = registry().chosen_factory() {
                    for param in factory.create().iter() {
                        ParameterControl { initial: registry().current_value(&param.name), param }
                    }
                },
                Button {
                    r#type: "submit",
                    "Apply"
                }
            }
        },
        if let Some(Err(err)) = save.value() {
            p { class: "text-red", "{err}" }
        }
    }
}

#[component]
pub(crate) fn ParameterControl(param: Parameter, initial: serde_json::Value) -> Element {
    let name = &param.name;
    rsx! {
        match param.schema {
            ParameterSchema::Float(schema) => {
                rsx! { float::Control { name, title: param.label, initial, schema } }
            },
            ParameterSchema::Duration(schema) => {
                rsx! { duration::Control { name, title: param.label, initial, schema } }
            },
        }
    }
}
