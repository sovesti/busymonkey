use enigo::agent::Token;
use tokio::time::Instant;

use crate::monkey::script::{
    TokenProto,
    command::Command,
    user::{
        CommandIntoProto, UserState,
        init::{FromInit, Init},
        state::{TokenSource, UserTokenProto},
    },
};

pub trait Script: Iterator<Item = Command> {}

pub type DynScript = Box<dyn Script>;

impl<T: Iterator<Item = Command>> Script for T {}

pub struct UserCommandDispatcher<T: CommandIntoProto, U: Script> {
    active: Option<Box<dyn UserTokenProto>>,
    state: UserState,
    cmd_into_proto: T,
    script: U,
}

impl<T: CommandIntoProto, U: Script> UserCommandDispatcher<T, U> {
    pub fn new(init: &Init, cmd_into_proto: T, script: U) -> Self {
        Self {
            active: None,
            state: UserState::from_init(init),
            cmd_into_proto,
            script,
        }
    }

    fn poll_command(&mut self) -> Option<Command> {
        self.script.next()
    }

    fn poll_proto(&mut self) {
        while self
            .active
            .as_mut()
            .map_or(true, |proto| proto.is_finished(&mut self.state))
        {
            if let Some(cmd) = self.poll_command() {
                self.active = Some((self.cmd_into_proto)(cmd));
            } else {
                break;
            }
        }
    }
}

impl<T: CommandIntoProto, U: Script> TokenProto for UserCommandDispatcher<T, U> {
    fn poll_action(&mut self) -> Option<Token> {
        self.poll_proto();
        let token = self.active.as_mut()?.poll_action(&mut self.state);
        self.state.handle_pre_token(token);
        self.state.poll_token()
    }

    fn poll_timeout(&mut self) -> Option<Instant> {
        self.poll_proto();
        self.active
            .as_mut()
            .map(|proto| proto.poll_timeout(&mut self.state))
            .flatten()
    }

    fn handle_timeout(&mut self, now: Instant) {
        self.state.handle_timeout(now);
    }

    fn is_finished(&mut self) -> bool {
        self.poll_proto();
        self.active.is_none()
    }
}
