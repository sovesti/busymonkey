use std::{collections::HashMap, rc::Rc};

use anyhow::anyhow;
use dioxus::prelude::*;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use serde_json as json;

use crate::client::settings::persistence::SettingsCategory;
use crate::monkey::model::parameters::{DynScript, ScriptFactory};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScriptSettingsRegistry {
    chosen: String,
    known: HashMap<String, json::Value>,
}

impl ScriptSettingsRegistry {
    pub fn current_settings(&self) -> json::Value {
        self.known.get(&self.chosen()).cloned().unwrap_or_default()
    }

    pub fn current_value<V: DeserializeOwned + Default>(&self, key: &str) -> V {
        self.current_settings()
            .as_object()
            .and_then(|obj| obj.get(key).cloned())
            .map(json::from_value)
            .and_then(Result::ok)
            .unwrap_or_default()
    }

    pub fn store_settings(&mut self, data: Rc<FormData>) {
        match serialize_form(data.clone()) {
            Ok(plain) => {
                self.known.insert(self.chosen(), plain);
            }
            Err(err) => error!("failed to store settings {data:?}: {err:?}"),
        }
    }

    pub fn create_with_settings(&self) -> anyhow::Result<DynScript> {
        self.chosen_factory()
            .ok_or_else(|| anyhow!("no factory for {}", &self.chosen))?
            .create_from_data(self.current_settings())
    }

    pub fn chosen_factory(&self) -> Option<&ScriptFactory> {
        inventory::iter::<ScriptFactory>
            .into_iter()
            .find(|factory| factory.has_key(&self.chosen))
    }

    fn chosen(&self) -> String {
        self.chosen.clone()
    }
}

impl Default for ScriptSettingsRegistry {
    fn default() -> Self {
        Self {
            chosen: "dummy".to_owned(),
            known: HashMap::new(),
        }
    }
}

impl SettingsCategory for ScriptSettingsRegistry {
    fn name() -> String {
        "scripts".to_owned()
    }
}

fn serialize_form(data: Rc<FormData>) -> anyhow::Result<json::Value> {
    #[cfg(feature = "desktop")]
    {
        serialized(data)
    }
    #[cfg(any(feature = "server", feature = "web"))]
    {
        Ok(json::Value::Null)
    }
}

#[cfg(feature = "desktop")]
fn serialized(data: Rc<FormData>) -> anyhow::Result<json::Value> {
    Ok(data.parsed_values()?)
}
