use std::time::Duration;

use tokio::time::Instant;

use crate::monkey::script::{
    state::poll_rate_yield::{PollRateYield, TimedIterator},
    user::{CursorToken, PreToken, UserState, UserTokenProto},
};

pub fn idle(state: &UserState, duration: Duration) -> impl UserTokenProto + 'static {
    let idle = IdleIter::new(state, duration);
    PollRateYield::new(state, idle)
}

struct IdleIter {
    finish: Instant,
}

impl IdleIter {
    fn new(state: &UserState, duration: Duration) -> Self {
        Self {
            finish: state.now() + duration,
        }
    }
}

impl TimedIterator for IdleIter {
    type Item = PreToken;

    fn next(&mut self, state: &mut UserState, delta: Duration) -> Option<PreToken> {
        let vel = state.cursor()?.vel;
        let dt = delta.as_secs_f32();
        let mxdv = state.cfg().max_acceleration() * dt;
        Some(PreToken::Cursor(if vel.len() > mxdv {
            let acc = -state.cursor()?.vel.unit() * state.cfg().max_acceleration();
            CursorToken::Accelerate(acc)
        } else {
            CursorToken::Stop
        }))
    }

    fn is_finished(&mut self, state: &UserState) -> bool {
        state.now() >= self.finish
    }
}
