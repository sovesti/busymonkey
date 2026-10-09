use std::time::Instant;

use super::model::Token;

pub(crate) mod dummy;

pub trait MonkeyScript {
    fn poll_action(&mut self) -> Option<Token>;

    fn poll_timeout(&self) -> Option<Instant>;

    fn handle_timeout(&mut self, now: Instant);
}
