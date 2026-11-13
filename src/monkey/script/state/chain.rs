use std::iter::Fuse;

use crate::monkey::script::user::{PreToken, UserState, UserTokenProto};

type Proto = Box<dyn UserTokenProto>;

pub fn chain<T: IntoIterator<Item = Proto> + 'static>(protos: T) -> impl UserTokenProto + 'static {
    Chain::new(protos)
}

struct Chain<T: Iterator<Item = Proto>> {
    active: Option<Proto>,
    inner: Fuse<T>,
}

impl<T: Iterator<Item = Proto>> Chain<T> {
    fn new<U: IntoIterator<Item = Proto, IntoIter = T>>(protos: U) -> Self {
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
                .map_or(false, |proto| proto.is_finished(state))
            {
                break;
            }
            self.active = self.inner.next();
            if self.active.is_none() {
                break;
            }
        }
    }
}

impl<T: Iterator<Item = Proto>> UserTokenProto for Chain<T> {
    fn poll_action(&mut self, state: &mut UserState) -> PreToken {
        self.propagate(state);
        self.active
            .as_mut()
            .map_or_default(|proto| proto.poll_action(state))
    }

    fn poll_timeout(&mut self, state: &UserState) -> Option<tokio::time::Instant> {
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
