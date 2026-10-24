use std::time::Duration;

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
