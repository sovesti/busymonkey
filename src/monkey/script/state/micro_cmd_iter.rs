use std::iter::Fuse;

use tokio::time::Instant;

use crate::monkey::script::{
    command::MicroCommand,
    state::MicroCmd,
    user::{PreToken, UserState, UserTokenProto},
};

pub fn micro_cmd_iter<T: IntoIterator<Item = MicroCommand> + 'static>(
    protos: T,
) -> impl UserTokenProto + 'static {
    MicroCmdIter::new(protos)
}

struct MicroCmdIter<T: Iterator<Item = MicroCommand>> {
    active: Option<MicroCmd>,
    inner: Fuse<T>,
}

impl<T: Iterator<Item = MicroCommand>> MicroCmdIter<T> {
    fn new<U: IntoIterator<Item = MicroCommand, IntoIter = T>>(protos: U) -> Self {
        Self {
            active: None,
            inner: protos.into_iter().fuse(),
        }
    }

    fn propagate(&mut self, state: &UserState) {
        loop {
            if !self
                .active
                .as_mut()
                .map_or(true, |proto| proto.is_finished(state))
            {
                break;
            }
            self.active = self.inner.next().map(|cmd| MicroCmd::new(cmd, state));
            if self.active.is_none() {
                break;
            }
        }
    }
}

impl<T: Iterator<Item = MicroCommand>> UserTokenProto for MicroCmdIter<T> {
    fn poll_action(&mut self, state: &mut UserState) -> PreToken {
        self.propagate(state);
        self.active
            .as_mut()
            .map_or_default(|proto| proto.poll_action(state))
    }

    fn poll_timeout(&mut self, state: &UserState) -> Option<Instant> {
        self.propagate(state);
        self.active
            .as_mut()
            .and_then(|proto| proto.poll_timeout(state))
    }

    fn is_finished(&mut self, state: &UserState) -> bool {
        self.propagate(state);
        self.active.is_none()
    }
}
