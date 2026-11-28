use std::time::Duration;

use anyhow::anyhow;
use serde::Deserialize;

use crate::client::settings::persistence::use_settings;
use crate::client::templates::persistence::{
    PersistentRecordedScript, PersistentRecordedScriptStoreExt,
};
use crate::monkey::model::Token;
use crate::monkey::model::parameters::{Parameter, ParameterSchema, ScriptFactory, ScriptMeta};
use crate::monkey::script::DynScript;

fn use_recorded_script(params: serde_json::Value) -> anyhow::Result<DynScript> {
    let stored = use_settings::<PersistentRecordedScript>();
    let params = serde_json::from_value::<RecordedScriptParameters>(params).unwrap_or_default();
    Ok(stored().ok_or(anyhow!("script not loaded"))?.tokens()()
        .to_script(params.delay().unwrap_or_default()))
}

#[derive(Deserialize, Default)]
struct RecordedScriptParameters {
    delay: String,
}

impl RecordedScriptParameters {
    fn delay(&self) -> anyhow::Result<Duration> {
        Ok(Duration::from_secs_f64(self.delay.parse::<f64>()? / 1000.0))
    }
}

inventory::submit! {
    ScriptFactory::new_meta("recorded",
        || { ScriptMeta::new("Recorded script") }, || {
        vec![
            Parameter::new(
                "delay",
                "Delay between restarts",
                ParameterSchema::duration(Duration::ZERO, Duration::from_mins(10)),
            )
        ]
    }, use_recorded_script)
}
