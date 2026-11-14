use dioxus::prelude::*;

use crate::monkey::model::{Coordinate, TimedToken, Token};
use crate::monkey::record::storage::RecordingStorage;

#[derive(Clone, Store)]
pub struct TruncateMoves<S> {
    delegate: S,
    previous: Option<TimedToken>,
}

impl<S: RecordingStorage> RecordingStorage for TruncateMoves<S> {
    fn add(&mut self, token: anyhow::Result<TimedToken>) {
        match token {
            Ok(next) => self.truncate_abs_move(next),
            Err(err) => self.delegate.add(Err(err)),
        }
    }

    fn active(&self) -> bool {
        self.delegate.active()
    }

    fn finish(&mut self) {
        if let Some(last) = self.previous.take() {
            self.delegate.add(Ok(last));
        }
    }
}

impl<S: RecordingStorage> TruncateMoves<S> {
    pub fn new(delegate: S) -> Self {
        Self {
            delegate,
            previous: None,
        }
    }

    #[cfg(test)]
    pub fn delegate(&self) -> &S {
        &self.delegate
    }

    fn truncate_abs_move(&mut self, next: TimedToken) {
        if is_zero_scroll(&next) {
            return;
        }
        if is_abs_move(&next) {
            self.store(next);
            return;
        }
        if let Some(prev) = self.previous.take() {
            self.delegate.add(Ok(prev));
        }
        self.delegate.add(Ok(next));
    }

    fn store(&mut self, next: TimedToken) {
        let _ = self.previous.insert(next);
    }
}

fn is_zero_scroll(next: &TimedToken) -> bool {
    matches!(next.token(), Token::Scroll(x, _) if *x == 0)
}

fn is_abs_move(next: &TimedToken) -> bool {
    matches!(next.token(), Token::MoveMouse(_, _, Coordinate::Abs))
}
