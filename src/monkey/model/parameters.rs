use std::time::Duration;

use crate::monkey::script::MonkeyScript;

inventory::collect!(ScriptFactory);

#[derive(Clone, PartialEq)]
pub struct ScriptParametersSchema {
    key: &'static str,
    defined: Vec<Parameter>,
}

impl ScriptParametersSchema {
    pub fn iter(&self) -> impl Iterator<Item = Parameter> {
        self.defined.clone().into_iter()
    }
}

pub type DynScript = Box<dyn MonkeyScript>;

pub struct ScriptFactory {
    key: &'static str,
    parameters: fn() -> Vec<Parameter>,
    create: fn(serde_json::Value) -> anyhow::Result<DynScript>,
}

impl ScriptFactory {
    pub const fn new(
        key: &'static str,
        parameters: fn() -> Vec<Parameter>,
        create: fn(serde_json::Value) -> anyhow::Result<DynScript>,
    ) -> Self {
        Self {
            key,
            parameters,
            create,
        }
    }

    pub fn create(&self) -> ScriptParametersSchema {
        ScriptParametersSchema {
            key: self.key,
            defined: (self.parameters)(),
        }
    }

    pub fn create_from_data(&self, data: serde_json::Value) -> anyhow::Result<DynScript> {
        (self.create)(data)
    }
}

#[derive(Clone, PartialEq)]
pub struct Parameter {
    pub name: String,
    pub label: String,
    pub schema: ParameterSchema,
}

impl Parameter {
    pub fn new(name: impl ToString, label: impl ToString, schema: ParameterSchema) -> Self {
        Self {
            name: name.to_string(),
            label: label.to_string(),
            schema,
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum ParameterSchema {
    Float(FloatSchema),
    Duration(DurationSchema),
}

impl ParameterSchema {
    pub fn float(from: f64, to: f64) -> Self {
        Self::Float(FloatSchema { from, to })
    }

    pub fn duration(from: Duration, to: Duration) -> Self {
        Self::Duration(DurationSchema { from, to })
    }
}

#[derive(Clone, Default, PartialEq)]
pub struct FloatSchema {
    pub from: f64,
    pub to: f64,
}

#[derive(Clone, Default, PartialEq)]
pub struct DurationSchema {
    pub from: Duration,
    pub to: Duration,
}
