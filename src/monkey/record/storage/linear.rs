use dioxus::logger::tracing;
use dioxus::prelude::*;

use crate::monkey::model::TimedToken;
use crate::monkey::record::storage::RecordingStorage;

#[derive(Clone, Default, Store)]
pub struct Linear {
    all: Vec<TimedToken>,
}

impl Linear {
    pub fn new() -> Self {
        Self::default()
    }

    #[cfg(test)]
    pub fn all(&self) -> &[TimedToken] {
        &self.all
    }
}

impl RecordingStorage for Linear {
    fn add(&mut self, token: anyhow::Result<TimedToken>) {
        match token {
            Ok(next) => self.all.push(next),
            Err(err) => tracing::error!("error while recording tokens: {err:?}"),
        }
    }

    fn active(&self) -> bool {
        true
    }
}
