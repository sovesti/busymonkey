use dioxus::prelude::*;

use crate::monkey::model::TimedToken;
use crate::monkey::record::storage::RecordingStorage;

#[derive(Store)]
pub struct Stoppable<S> {
    delegate: S,
    stopped: ReadSignal<bool>,
}

impl<S: RecordingStorage> RecordingStorage for Stoppable<S> {
    fn add(&mut self, token: anyhow::Result<TimedToken>) {
        self.delegate.add(token)
    }

    fn active(&self) -> bool {
        *self.stopped.read()
    }
}

impl<S: RecordingStorage> Stoppable<S> {
    pub fn new(delegate: S, stopped: ReadSignal<bool>) -> Self {
        Self { delegate, stopped }
    }
}
