use std::vec;

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::client::settings::persistence::SettingsCategory;
use crate::monkey::model::Token;
use crate::monkey::record::script::RecordedScript;

#[derive(Clone, Debug, Default, Serialize, Deserialize, Store)]
pub struct PersistentRecordedScript {
    tokens: RecordedScript,
}

impl SettingsCategory for PersistentRecordedScript {
    fn name() -> String {
        "recorded".to_owned()
    }
}
