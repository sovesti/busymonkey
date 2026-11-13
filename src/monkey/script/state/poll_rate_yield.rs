use crate::monkey::{
    Instant,
    script::user::{PreToken, UserState, UserTokenProto},
};
use std::time::Duration;

pub trait TimedIterator {
    type Item;
    fn next(&mut self, state: &mut UserState, delta: Duration) -> Option<Self::Item>;

    fn is_finished(&mut self, state: &UserState) -> bool;
}

pub struct PollRateYield<T: TimedIterator<Item = PreToken>> {
    iter: T,
    last: Instant,
    rate: Duration,
}

impl<T: TimedIterator<Item = PreToken>> PollRateYield<T> {
    pub fn new(state: &UserState, iter: T) -> Self {
        Self {
            iter: iter,
            rate: state.cfg().poll_rate,
            last: state.now(),
        }
    }
}

impl<T: TimedIterator<Item = PreToken>> UserTokenProto for PollRateYield<T> {
    fn poll_action(&mut self, state: &mut UserState) -> PreToken {
        if state.now() >= self.last + self.rate {
            self.last = state.now();
            self.iter.next(state, self.rate).unwrap_or_default()
        } else {
            PreToken::Skip
        }
    }

    fn poll_timeout(&mut self, _state: &UserState) -> Option<Instant> {
        Some(self.last + self.rate)
    }

    fn is_finished(&mut self, state: &UserState) -> bool {
        self.iter.is_finished(state)
    }
}
