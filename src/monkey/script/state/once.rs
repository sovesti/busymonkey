use crate::monkey::{
    Instant,
    script::user::{PreToken, UserState, UserTokenProto},
};

pub struct Once {
    token: Option<PreToken>,
    finish: Instant,
}

impl Once {
    pub fn new(state: &UserState, token: PreToken) -> Self {
        Self {
            token: Some(token),
            finish: state.now() + state.cfg().poll_rate,
        }
    }
}

impl UserTokenProto for Once {
    fn poll_action(&mut self, _state: &mut UserState) -> PreToken {
        self.token.take().unwrap_or_default()
    }

    fn poll_timeout(&mut self, _state: &UserState) -> Option<Instant> {
        Some(self.finish)
    }

    fn is_finished(&mut self, _state: &UserState) -> bool {
        matches!(self.token, None)
    }
}
