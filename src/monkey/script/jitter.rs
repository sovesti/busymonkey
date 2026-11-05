
use rand::RngExt;
use std::time::Duration;

use super::MonkeyScript;
use crate::monkey::{Instant, model::Token};

pub struct RandomJitter<S: MonkeyScript> {
    inner: S,
    pixels: i32,
    delay_ms: u64,
}

impl<S: MonkeyScript> RandomJitter<S> {
    pub fn new(inner: S, pixels: i32, delay_ms: u64) -> Self {
        Self { inner, pixels, delay_ms }
    }
}

impl<S: MonkeyScript> MonkeyScript for RandomJitter<S> {
    fn poll_action(&mut self) -> Option<Token> {
        let mut rng = rand::rng();
        self.inner.poll_action().map(|token| match token {
            Token::MoveMouse(x, y, mode) => Token::MoveMouse(
                x.saturating_add(rng.random_range(-self.pixels..=self.pixels)),
                y.saturating_add(rng.random_range(-self.pixels..=self.pixels)),
                mode,
            ),
            other => other,
        })
    }

    fn poll_timeout(&mut self) -> Option<Instant> {
        self.inner.poll_timeout().map(|time| {
            time + Duration::from_millis(
                rand::rng().random_range(0..=self.delay_ms)
            )
        })
    }

    fn handle_timeout(&mut self, now: Instant) {
        self.inner.handle_timeout(now);
    }
}
