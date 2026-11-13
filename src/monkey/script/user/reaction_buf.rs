use std::time::Duration;

use tokio::time::Instant;

use crate::monkey::script::{
    user::init::{FromInit, Init},
    util::time_buf::TimeBuf,
};

/// `TimeBuf` wrapper that delays objects for `UserCfg::reaction_time`
pub struct ReactionBuf<T>(TimeBuf<T>);

impl<T> ReactionBuf<T> {
    pub fn new(delay: Duration, now: Instant) -> Self {
        Self(TimeBuf::new(delay, now))
    }

    pub fn handle_timeout(&mut self, now: Instant) {
        self.0.handle_timeout(now)
    }

    pub fn push(&mut self, obj: T, at: Instant) {
        self.0.push(obj, at)
    }

    pub fn back(&self) -> Option<&T> {
        self.0.back()
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.0.pop_back()
    }
}

impl<T> FromInit for ReactionBuf<T> {
    fn from_init(init: &Init) -> Self {
        Self::new(init.cfg.reaction_time, init.snap.now)
    }
}
