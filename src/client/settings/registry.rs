use std::{collections::HashMap, rc::Rc};

use dioxus::prelude::*;

#[derive(Clone)]
pub struct ScriptSettingsRegistry {
    chosen: String,
    known: HashMap<String, String>,
}

impl ScriptSettingsRegistry {
    pub fn new() -> Self {
        Self {
            chosen: "dummy".to_owned(),
            known: HashMap::new(),
        }
    }

    pub fn current_settings(&self) -> serde_json::Value {
        self.known
            .get(&self.chosen())
            .map(String::as_str)
            .map(serde_json::from_str)
            .and_then(Result::ok)
            .unwrap_or_default()
    }

    pub fn current_value(&self, key: &str) -> serde_json::Value {
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

fn serialize_form(data: Rc<FormData>) -> anyhow::Result<String> {
    #[cfg(any(feature = "desktop", feature = "web"))]
    {
        serialized(data)
    }
    #[cfg(feature = "server")]
    {
        Ok("".to_owned())
    }
}

#[cfg(any(feature = "desktop", feature = "web"))]
fn serialized(data: Rc<FormData>) -> anyhow::Result<String> {
    let parsed: serde_json::Value = data.parsed_values()?;
    Ok(serde_json::to_string(&parsed)?)
}
