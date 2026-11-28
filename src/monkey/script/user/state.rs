use std::collections::VecDeque;

use super::UserCfg;
use crate::monkey::script::user::{
    ScrollToken, click::{ClickBuf, ClickToken}, cursor::{CursorBuf, CursorState, CursorToken}, init::{FromInit, Init}, keyboard::{KeyboardBuf, KeyboardToken}, scroll::ScrollBuf,
};
use enigo::agent::Token;
use tokio::time::Instant;

pub enum PreToken {
    Skip,
    Cursor(CursorToken),
    Click(ClickToken),
    Keyboard(KeyboardToken),
    Scroll(ScrollToken),
}

impl Default for PreToken {
    fn default() -> Self {
        Self::Skip
    }
}

pub trait TokenSource {
    fn handle_pre_token(&mut self, token: PreToken);

    fn poll_token(&mut self) -> Option<Token>;

    fn handle_timeout(&mut self, now: Instant);
}

pub struct UserState {
    cfg: UserCfg,
    now: Instant,
    cursor: CursorBuf,
    click: ClickBuf,
    keyboard: KeyboardBuf,
    scroll: ScrollBuf,
    tokens: VecDeque<Token>,
}

impl FromInit for UserState {
    fn from_init(init: &Init) -> Self {
        Self {
            cfg: init.cfg,
            now: init.snap.now,
            cursor: CursorBuf::from_init(init),
            click: ClickBuf::from_init(init),
            keyboard: KeyboardBuf::from_init(init),
            scroll: ScrollBuf::from_init(init),
            tokens: VecDeque::new(),
        }
    }
}

impl TokenSource for UserState {
    fn poll_token(&mut self) -> Option<Token> {
        self.tokens.pop_back()
    }

    fn handle_timeout(&mut self, now: Instant) {
        self.now = now;
    }

    fn handle_pre_token(&mut self, token: PreToken) {
        if let Some(token) = match token {
            PreToken::Cursor(token) => self.cursor.handle_token(token, self.now, &self.cfg),
            PreToken::Click(token) => Some(self.click.handle_token(token)),
            PreToken::Keyboard(token) => Some(self.keyboard.handle_token(token)),
            PreToken::Scroll(token) => Some(self.scroll.handle_token(token)),
            PreToken::Skip => None,
        } {
            self.tokens.push_front(token);
        }
    }
}

impl UserState {
    pub fn cfg(&self) -> &UserCfg {
        &self.cfg
    }

    pub fn now(&self) -> Instant {
        self.now
    }

    pub fn cursor(&self) -> Option<&CursorState> {
        self.cursor.visible()
    }
}

/// TokenProto that reads state from and passes tokens through a `UserState`
///
/// Timeouts are handled at polling time
pub trait UserTokenProto {
    /// Put action in `state`
    fn poll_action(&mut self, state: &mut UserState) -> PreToken;

    fn poll_timeout(&mut self, state: &UserState) -> Option<Instant>;

    fn is_finished(&mut self, state: &UserState) -> bool;
}
