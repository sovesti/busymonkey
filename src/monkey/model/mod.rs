pub(crate) mod parameters;

use std::time::Duration;

pub use enigo::{Axis, Button, Coordinate, Direction, InputError, InputResult, Key, agent::Token};

#[derive(Clone, Debug, PartialEq)]
pub struct TimedToken {
    timestamp: Duration,
    token: Token,
}

impl TimedToken {
    pub fn new(timestamp: Duration, token: Token) -> Self {
        Self { timestamp, token }
    }

    pub fn token(&self) -> &Token {
        &self.token
    }

    pub fn timestamp(&self) -> Duration {
        self.timestamp
    }
}
