use std::time::Duration;

use crate::monkey::Instant;
use enigo::agent::Token;

use crate::monkey::script::{MonkeyScript, command::CommandState};

pub struct Wait {
    is_finished: bool,
    finish: Instant,
}

impl Wait {
    pub fn new(time: Instant, wait: Duration) -> Self {
        Self {
            is_finished: false,
            finish: time + wait,
        }
    }
}

impl MonkeyScript for Wait {
    fn poll_action(&mut self) -> Option<Token> {
        None
    }

    fn poll_timeout(&mut self) -> Option<Instant> {
        Some(self.finish)
    }

    fn handle_timeout(&mut self, now: Instant) {
        if now > self.finish {
            self.is_finished = true;
        }
    }
}

impl CommandState for Wait {
    fn is_finished(&mut self) -> bool {
        self.is_finished
    }
}
