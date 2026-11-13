use std::time::Duration;

use crate::monkey::Instant;

use crate::monkey::script::{
    state::poll_rate_yield::{PollRateYield, TimedIterator},
    user::{CursorState, CursorToken, PreToken, UserState, UserTokenProto},
    util::vector::Vector,
};

pub fn smooth_move(
    state: &UserState,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
) -> impl UserTokenProto + 'static {
    PollRateYield::new(state, SmoothMoveIter::new(state, x, y, w, h))
}

struct SmoothMoveIter {
    target: Vector,
    _gate: Vector,
    start: (Instant, Vector),
}

const TOTAL: Duration = Duration::from_secs(1);

fn smoothstep(x: f32) -> f32 {
    3f32 * x * x - 2f32 * x * x * x
}

impl SmoothMoveIter {
    pub fn new(state: &UserState, x: i32, y: i32, w: i32, h: i32) -> Self {
        let target = (Vector::from_coords((x, y)) + Vector::from_coords((x + w, y + h))) / 2f32;
        let _gate = (Vector::from_coords((w, h))) / 2f32;
        let start = (
            state.now(),
            state.cursor().map_or(Vector::zero(), |cp| cp.pos),
        );
        Self {
            target,
            _gate,
            start,
        }
    }
}

impl TimedIterator for SmoothMoveIter {
    type Item = PreToken;

    fn next(&mut self, state: &mut UserState, _delta: Duration) -> Option<Self::Item> {
        (!self.is_finished(state)).then_some(())?;
        let now = state.now();
        let progress = (now - self.start.0).as_secs_f32() / TOTAL.as_secs_f32();
        let al = smoothstep(progress);
        let pos = self.start.1 * (1f32 - al) + self.target * al;
        Some(PreToken::Cursor(CursorToken::SetPosition(pos)))
    }

    fn is_finished(&mut self, state: &UserState) -> bool {
        state.now() >= self.start.0 + TOTAL
    }
}
