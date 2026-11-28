use enigo::{Direction, Key, agent::Token};

use crate::monkey::script::user::init::{FromInit, Init};

pub struct KeyboardToken {
    pub key: Key,
    pub dir: Direction,
}

pub struct KeyboardBuf;

impl FromInit for KeyboardBuf {
    fn from_init(_init: &Init) -> Self {
        Self
    }
}

impl KeyboardBuf {
    pub fn handle_token(&mut self, token: KeyboardToken) -> Token {
        Token::Key(token.key, token.dir)
    }
}
