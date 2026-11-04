use std::marker::PhantomData;

use super::UserCfg;
use crate::monkey::script::{
    TokenProto,
    command::Command,
    util::{time_buf::TimeBuf, vector::Vector},
};
use dioxus::core::SpawnIfAsync;
use enigo::agent::Token;
use tokio::time::Instant;

pub trait CommandProto {
    fn into_proto(cmd: Command) -> Box<dyn UserTokenProto>;
}

pub struct UserCommandDispatcher<T: CommandProto> {
    active: Option<Box<dyn UserTokenProto>>,
    state: UserState,
    cmdproto: PhantomData<T>,
}

impl<T: CommandProto> UserCommandDispatcher<T> {
    fn poll_proto(&mut self) {
        while self
            .active
            .as_mut()
            .map_or(true, |proto| proto.is_finished(&mut self.state))
        {
            if let Some(cmd) = self.state.poll_command() {
                self.active = Some(T::into_proto(cmd));
            } else {
                break;
            }
        }
    }
}

pub struct UserState {
    cfg: UserCfg,
    now: Instant,
    cmds: TimeBuf<Command>,
    cursor: TimeBuf<CursorState>,
}

impl UserState {
    fn poll_command(&mut self) -> Option<Command> {
        self.cmds.pop_back()
    }

    fn handle_command(&mut self, cmd: Command) {
        self.cmds.push(cmd, self.now);
    }
}

pub struct CursorState {
    pub pos: Vector,
    pub vel: Vector,
}

pub trait UserTokenProto {
    fn poll_action(&mut self, state: &mut UserState) -> Option<Token>;

    fn poll_timeout(&mut self, state: &mut UserState) -> Option<Instant>;

    fn handle_timeout(&mut self, state: &mut UserState, now: Instant);

    fn is_finished(&mut self, state: &mut UserState) -> bool;
}

impl<T: CommandProto> TokenProto for UserCommandDispatcher<T> {
    fn poll_action(&mut self) -> Option<Token> {
        self.active
            .as_mut()
            .map(|proto| proto.poll_action(&mut self.state))
            .flatten()
    }

    fn poll_timeout(&mut self) -> Option<Instant> {
        self.active
            .as_mut()
            .map(|proto| proto.poll_timeout(&mut self.state))
            .flatten()
    }

    fn handle_timeout(&mut self, now: Instant) {
        self.active
            .as_mut()
            .map(|proto| proto.handle_timeout(&mut self.state, now));
    }

    fn is_finished(&mut self) -> bool {
        false
    }
}
