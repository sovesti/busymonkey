use enigo::{Button, Direction, agent::Token};

use crate::monkey::script::user::init::{FromInit, Init};

pub struct ClickToken {
    pub btn: Button,
    pub dir: Direction,
}

#[derive(Default)]
pub struct ClickBuf;

impl FromInit for ClickBuf {
    fn from_init(_init: &Init) -> Self {
        Self
    }
}

impl ClickBuf {
    pub fn handle_token(&mut self, token: ClickToken) -> Token {
        Token::Button(token.btn, token.dir)
    }
}
