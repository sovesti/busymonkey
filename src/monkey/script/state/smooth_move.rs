use std::{collections::VecDeque, time::Duration};

use enigo::{Mouse, agent::Token};
use rand::{RngExt, rngs::ThreadRng};
use crate::monkey::Instant;

use crate::monkey::script::{
    command::CommandState,
    state::poll_rate_yield::{PollRateYield, TimedIterator},
    user::User,
    util::vector::Vector,
};

pub fn smooth_move<'a>(
    user: &'a User,
    now: Instant,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
) -> impl CommandState {
    PollRateYield::new(SmoothMoveIter::new(user, x, y, w, h), user.poll_rate, now)
}

struct TimeBuf<T: Clone> {
    max: Duration,
    total: Duration,
    buf: VecDeque<(T, Duration)>,
}

impl<T: Clone> TimeBuf<T> {
    fn new(max: Duration) -> Self {
        Self {
            max, 
            total: Duration::default(), 
            buf: VecDeque::default(),
        }
    }

    fn push(&mut self, obj: T, dur: Duration) {
        self.buf.push_front((obj, dur));
        self.total += dur;
        while self.total > self.max {
            if let Some((_, last)) = self.buf.pop_back() {
                self.total -= last;
            } else {
                break;
            }
        }
    }

    fn back(&mut self) -> Option<T> {
        self.buf.back().map(|(obj, _)| obj.clone())
    }
}

struct SmoothMoveIter<'a> {
    user: &'a User,
    target: Vector,
    gate: Vector,
    rng: ThreadRng,
    vel: Vector,
    cur: Vector,
    pos_buf: TimeBuf<(Vector, Vector)>,
}

impl<'a> SmoothMoveIter<'a> {
    pub fn new(user: &'a User, x: i32, y: i32, w: i32, h: i32) -> Self {
        let target = (Vector::from_coords((x, y)) + Vector::from_coords((x + w, y + h))) / 2f32;
        let gate = (Vector::from_coords((w, h))) / 2f32;
        Self {
            user,
            target,
            gate,
            rng: rand::rng(),
            vel: Vector::from_coords((0, 0)),
            // TODO: temporary stub, because no architecture for location polling is around yet
            cur: Vector::from_coords(enigo::Enigo::new(&enigo::Settings::default()).unwrap().location().unwrap()),
            pos_buf: TimeBuf::new(user.reaction_time),
        }
    }
}

impl<'a> TimedIterator for SmoothMoveIter<'a> {
    type Item = Token;

    fn next(&mut self, delta: Duration) -> Option<Self::Item> {
        if self.is_finished() {
            return None;
        }
        let dt= delta.as_secs_f32();
        let mxdv = self.user.max_acceleration() * dt;
        let (seen_pos, seen_vel) = self.pos_buf.back().unwrap_or((self.cur, self.vel));
        let off = self.target - seen_pos;
        let mut dv = off.unit() * seen_vel.dot(off.unit()) - seen_vel;
        dv *= 0.25f32;
        dv = dv.clamp_len(mxdv);
        dv += off.clamp_len((mxdv * mxdv - dv.sqlen()).clamp(0f32, mxdv * mxdv).sqrt());
        let proximity = off.len().clamp(0f32, 500f32) / 500f32;
        
        dv = dv.rotate(self.rng.random_range(-0.4 * proximity..0.4 * proximity));
        self.vel += dv;
        self.vel = self.vel.clamp_len(self.user.max_speed * proximity);
        self.cur += self.vel * dt;
        let (dx, dy) = self.cur.into_coords();
        self.pos_buf.push((self.cur, self.vel), delta);
        Some(Token::MoveMouse(dx, dy, enigo::Coordinate::Abs))
    }

    fn is_finished(&mut self) -> bool {
        let offset = self.target - self.cur;
        offset.0.abs() <= self.gate.0 && offset.1.abs() <= self.gate.1
    }
}
