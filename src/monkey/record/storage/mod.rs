use dioxus::prelude::*;

use crate::monkey::model::TimedToken;

pub(crate) mod linear;
pub(crate) mod stoppable;
pub(crate) mod truncate;

pub trait RecordingStorage: 'static {
    fn add(&mut self, token: anyhow::Result<TimedToken>);
    fn active(&self) -> bool;
    fn finish(&mut self) {}
}

impl<S: RecordingStorage> RecordingStorage for Store<S> {
    fn add(&mut self, token: anyhow::Result<TimedToken>) {
        self.write().add(token);
    }

    fn active(&self) -> bool {
        self.read().active()
    }

    fn finish(&mut self) {
        self.write().finish();
    }
}
