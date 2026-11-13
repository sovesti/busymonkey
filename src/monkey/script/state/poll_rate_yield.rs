use crate::monkey::script::{MonkeyScript, command::CommandState};
use enigo::agent::Token;
use std::time::Duration;
use crate::monkey::Instant;

pub trait TimedIterator {
    type Item;
    fn next(&mut self, delta: Duration) -> Option<Self::Item>;

    fn is_finished(&mut self) -> bool;
}

pub struct PollRateYield<T: TimedIterator<Item = Token>> {
    iter: T,
    last: Instant,
    now: Instant,
    rate: Duration,
}

impl<T: TimedIterator<Item = Token>> PollRateYield<T> {
    pub fn new(iter: T, rate: Duration, now: Instant) -> Self {
        Self {
            iter: iter,
            rate,
            now,
            last: now,
        }
    }
}

impl<T: TimedIterator<Item = Token>> MonkeyScript for PollRateYield<T> {
    fn poll_action(&mut self) -> Option<Token> {
        if self.now < self.last {
            None
        } else {
            self.last += self.rate;
            self.iter.next(self.rate)
        }
    }

    fn poll_timeout(&mut self) -> Option<Instant> {
        Some(self.last + self.rate)
    }

    fn handle_timeout(&mut self, now: Instant) {
        self.now = now;
    }
}

impl<T: TimedIterator<Item = Token>> CommandState for PollRateYield<T> {
    fn is_finished(&mut self) -> bool {
        self.iter.is_finished()
    }
}
