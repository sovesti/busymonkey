use std::time::Duration;

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
