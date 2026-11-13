use tokio::time::Instant;

use crate::monkey::script::{
    command::MicroCommand,
    state::{idle, once::Once, smooth_move::smooth_move},
    user::{ClickToken, KeyboardToken, PreToken, UserState, UserTokenProto},
};

pub struct MicroCmd {
    inner: Box<dyn UserTokenProto>,
}

impl MicroCmd {
    pub fn new(cmd: MicroCommand, state: &UserState) -> Self {
        Self {
            inner: match cmd {
                MicroCommand::MoveToRect(x, y, w, h) => Box::new(smooth_move(state, x, y, w, h)),
                MicroCommand::MouseButton(btn, dir) => {
                    Box::new(Once::new(state, PreToken::Click(ClickToken { btn, dir })))
                }
                MicroCommand::Key(key, dir) => Box::new(Once::new(
                    state,
                    PreToken::Keyboard(KeyboardToken { key, dir }),
                )),
                MicroCommand::Idle(duration) => Box::new(idle(state, duration)),
            },
        }
    }
}

impl UserTokenProto for MicroCmd {
    fn poll_action(&mut self, state: &mut UserState) -> PreToken {
        self.inner.poll_action(state)
    }

    fn poll_timeout(&mut self, state: &UserState) -> Option<Instant> {
        self.inner.poll_timeout(state)
    }

    fn is_finished(&mut self, state: &UserState) -> bool {
        self.inner.is_finished(state)
    }
}
