use std::time::Duration;

use dioxus::prelude::*;

use crate::client::settings::registry::ScriptSettingsRegistry;
use crate::monkey::model::parameters::{Parameter, ParameterSchema};

pub(crate) mod duration;
pub(crate) mod float;
pub(crate) mod registry;

#[component]
pub fn Settings() -> Element {
    let params = vec![
        Parameter::new(
            "speed",
            "Movement speed, px/s",
            ParameterSchema::float(0.0, 100.0),
        ),
        Parameter::new(
            "interval",
            "Interval between circles",
            ParameterSchema::duration(Duration::ZERO, Duration::from_secs(10)),
        ),
    ];
    let mut registry = use_context::<Signal<ScriptSettingsRegistry>>();
    rsx! {
        form {
            class: "w-64",
            onsubmit: move |evt: FormEvent| async move {
                evt.prevent_default();
                registry.write().store_settings(evt.data());
            },
            for param in params.into_iter() {
                ParameterControl { initial: registry().current_value(&param.name), param }
            },
            button {
                class: "rounded-m m-1 px-1 py-2 hover:bg-sky-100 cursor-pointer",
                r#type: "submit",
                "Apply"
            }
        }
    }
}

#[component]
fn ParameterControl(param: Parameter, initial: String) -> Element {
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
