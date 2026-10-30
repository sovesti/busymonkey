use dioxus::prelude::*;

use crate::client::settings::persistence::{use_save_settings, use_settings};
use crate::client::settings::registry::ScriptSettingsRegistry;
use crate::monkey::model::parameters::{Parameter, ParameterSchema};

pub(crate) mod duration;
pub(crate) mod float;
pub(crate) mod persistence;
pub(crate) mod registry;

#[component]
pub fn Settings() -> Element {
    let mut save = use_save_settings::<ScriptSettingsRegistry>();
    rsx! {
        if let Some(mut registry) = *use_settings::<ScriptSettingsRegistry>().read() {
            form {
                class: "w-64",
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
                button {
                    class: "rounded-m m-1 px-1 py-2 hover:bg-sky-100 cursor-pointer",
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
fn ParameterControl(param: Parameter, initial: serde_json::Value) -> Element {
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
