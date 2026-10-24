use std::{collections::HashMap, rc::Rc};

use dioxus::prelude::*;
use serde_json as json;

#[derive(Clone)]
pub struct ScriptSettingsRegistry {
    chosen: String,
    known: HashMap<String, json::Value>,
}

impl ScriptSettingsRegistry {
    pub fn new() -> Self {
        Self {
            chosen: "dummy".to_owned(),
            known: HashMap::new(),
        }
    }

    pub fn current_settings(&self) -> json::Value {
        self.known.get(&self.chosen()).cloned().unwrap_or_default()
    }

    pub fn current_value(&self, key: &str) -> json::Value {
        self.current_settings()
            .as_object()
            .map_or_default(|obj| obj.get(key).cloned().unwrap_or_default())
    }

    pub fn store_settings(&mut self, data: Rc<FormData>) {
        match serialize_form(data.clone()) {
            Ok(plain) => {
                debug!("saving settings for {}: {}", self.chosen, &plain);
                self.known.insert(self.chosen(), plain);
            }
            Err(err) => error!("failed to store settings {data:?}: {err:?}"),
        }
    }

    fn chosen(&self) -> String {
        self.chosen.clone()
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
