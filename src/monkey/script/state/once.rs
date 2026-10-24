use crate::monkey::Instant;
use enigo::agent::Token;

use crate::monkey::script::{MonkeyScript, command::CommandState, user::User};

pub struct Once {
    token: Option<Token>,
    finish: Instant,
}

impl Once {
    pub fn new(user: &User, token: Token, time: Instant) -> Self {
        Self {
            token: Some(token),
            finish: time + user.poll_rate,
        }
    }
}

impl MonkeyScript for Once {
    fn poll_action(&mut self) -> Option<Token> {
        self.token.take()
    }

    fn poll_timeout(&mut self) -> Option<Instant> {
        Some(self.finish)
    }

    fn handle_timeout(&mut self, _now: Instant) {}
}

impl CommandState for Once {
    fn is_finished(&mut self) -> bool {
        matches!(self.token, None)
    }
}
