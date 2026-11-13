use std::time::Duration;

use enigo::agent::Token;
use tokio::time::Instant;

use crate::monkey::script::{
    user::{
        UserCfg,
        init::{FromInit, Init},
        reaction_buf::ReactionBuf,
    },
    util::vector::Vector,
};

#[non_exhaustive]
pub enum CursorToken {
    Accelerate(Vector),
    SetPosition(Vector),
    Proceed,
    Stop,
}

impl Default for CursorToken {
    fn default() -> Self {
        Self::Proceed
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CursorState {
    pub pos: Vector,
    pub vel: Vector,
}

impl CursorState {
    pub fn set_position(self, pos: Vector) -> Self {
        Self {
            pos,
            vel: Vector::zero(),
        }
    }

    pub fn accelerated(self, acc: Vector, delta: Duration) -> Self {
        let dt = delta.as_secs_f32();
        Self {
            pos: self.pos + self.vel * dt + acc * dt * dt / 2f32,
            vel: self.vel + acc * dt,
        }
    }

    pub fn stopped(self, delta: Duration) -> Self {
        let dt = delta.as_secs_f32();
        Self {
            pos: self.pos + self.vel * dt / 2f32,
            vel: Vector::zero(),
        }
    }

    pub fn proceed(self, delta: Duration) -> Self {
        let dt = delta.as_secs_f32();
        Self {
            pos: self.pos + self.vel * dt,
            vel: self.vel,
        }
    }
}

pub struct CursorBuf {
    buf: ReactionBuf<CursorState>,
    cur: (CursorState, Instant),
}

impl FromInit for CursorBuf {
    fn from_init(init: &Init) -> Self {
        let mut buf = ReactionBuf::<CursorState>::from_init(init);
        let cur = (init.snap.cursor, init.snap.now);
        buf.push(cur.0, cur.1);
        Self { buf, cur }
    }
}

impl CursorBuf {
    pub fn visible(&self) -> Option<&CursorState> {
        self.buf.back()
    }

    pub fn handle_token(
        &mut self,
        token: CursorToken,
        now: Instant,
        cfg: &UserCfg,
    ) -> Option<Token> {
        match token {
            CursorToken::Accelerate(mut acc) => {
                let delta = now - self.cur.1;
                let dt = delta.as_secs_f32();
                acc = acc.clamp_len(cfg.max_acceleration());
                let acc =
                    ((self.cur.0.vel + acc * dt).clamp_len(cfg.max_speed) - self.cur.0.vel) / dt;
                let next = self.cur.0.accelerated(acc, delta);
                self.cur = (next, now);
                self.buf.push(next, now);
                let (x, y) = next.pos.into_coords();
                Some(Token::MoveMouse(x, y, enigo::Coordinate::Abs))
            }
            CursorToken::SetPosition(pos) => {
                let next = self.cur.0.set_position(pos);
                self.cur = (next, now);
                self.buf.push(next, now);
                let (x, y) = next.pos.into_coords();
                Some(Token::MoveMouse(x, y, enigo::Coordinate::Abs))
            }
            CursorToken::Stop => {
                let delta = now - self.cur.1;
                let next = self.cur.0.stopped(delta);
                self.cur = (next, now);
                self.buf.push(next, now);
                let (x, y) = next.pos.into_coords();
                Some(Token::MoveMouse(x, y, enigo::Coordinate::Abs))
            }
            CursorToken::Proceed => {
                let delta = now - self.cur.1;
                let next = self.cur.0.proceed(delta);
                self.cur = (next, now);
                self.buf.push(next, now);
                let (x, y) = next.pos.into_coords();
                Some(Token::MoveMouse(x, y, enigo::Coordinate::Abs))
            }
        }
    }
}
