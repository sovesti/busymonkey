use enigo::{Axis, Key, agent::Token};

use crate::monkey::script::user::init::{FromInit, Init};

pub struct ScrollToken {
    pub val: i32,
    pub axis: Axis,
}

pub struct ScrollBuf;

impl FromInit for ScrollBuf {
    fn from_init(_init: &Init) -> Self {
        Self
    }
}

impl ScrollBuf {
    pub fn handle_token(&mut self, token: ScrollToken) -> Token {
        Token::Scroll(token.val, token.axis)
    }
}
