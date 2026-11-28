use dioxus::prelude::*;

use super::persistence::{use_save_settings, use_settings};
use super::registry::ScriptSettingsRegistry;
use super::{duration, float};
use crate::client::components::{Button, ButtonVariant, Select, SelectOption};
use crate::client::settings::registry::{
    ScriptSettingsRegistryStoreExt, ScriptSettingsRegistryStoreImplExt,
};
use crate::client::settings::shortcut::registry::{
    ShortcutAction, ShortcutsRegistry, ShortcutsRegistryStoreImplExt,
};
use crate::client::templates::preview::RecordedScriptPreview;
use crate::monkey::model::parameters::{Parameter, ParameterSchema, ScriptFactory};

#[component]
pub fn ScriptSettingsView() -> Element {
    let mut save = use_save_settings::<ScriptSettingsRegistry>();
    rsx! {
        if let Some(mut registry) = *use_settings::<ScriptSettingsRegistry>().read() {
            div { class: "flex flex-row bg-inherit justify-start gap-4",
                ScriptSelection { registry }
                SettingsForm { save, registry }
            }
        }
    }
}

#[component]
fn ScriptSelection(registry: Store<ScriptSettingsRegistry>) -> Element {
    let chosen = registry.chosen().cloned();
    let scripts = inventory::iter::<ScriptFactory>
        .into_iter()
        .enumerate()
        .map(|(i, factory)| {
            rsx! {
                FactorySelectOption { i, name: factory.key(), label: factory.label() }
            }
        });
    rsx! {
        div { class: "flex flex-col w-96 bg-inherit gap-4",
            ScriptHint {}
            Select::<String> {
                default_value: chosen.clone(),
                on_value_change: move |value: Option<String>| {
                    if let Some(value) = value {
                        registry.choose(value);
                    }
                },
                {scripts}
            }
            if chosen == "recorded" {
                RecordedScriptPreview {}
            }
        }
    }
}

#[component]
fn FactorySelectOption(i: usize, name: String, label: String) -> Element {
    rsx! {
        SelectOption::<String> { index: i, value: name, text_value: label.clone(), {label} }
    }
}

#[component]
fn SettingsForm(save: Action<(), ()>, registry: Store<ScriptSettingsRegistry>) -> Element {
    rsx! {
        form {
            class: "flex flex-col w-64 bg-inherit items-start gap-4",
            onsubmit: move |evt: FormEvent| async move {
                evt.prevent_default();
                registry.store_settings(evt.data());
                save.call();
            },
            ScriptTitle { registry }
            ScriptParameterControls { registry }
            Button {
                variant: ButtonVariant::Solid,
                class: "self-center",
                r#type: "submit",
                "Save"
            }
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
    let keys = use_store(ShortcutsRegistry::default).load_or_default(ShortcutAction::ToggleScript);
    rsx! {
        if let Some(keys) = keys {
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
