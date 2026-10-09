use std::{
    collections::VecDeque,
    f64::consts::PI,
    time::{Duration, Instant},
};

use crate::monkey::{
    engine::{self, Agent},
    model::{Coordinate, Token},
    script::MonkeyScript,
};

struct DummyState {
    updated: Instant,
    position: usize,
}

impl Default for DummyState {
    fn default() -> Self {
        Self {
            updated: Instant::now(),
            position: 0,
        }
    }
}

impl DummyState {
    fn increment(&mut self) {
        self.position += 1;
        self.updated += Duration::from_millis(20);
    }

    fn move_to(&self) -> Token {
        let (x, y) = self.coordinates();
        Token::MoveMouse(x, y, Coordinate::Rel)
    }

    fn coordinates(&self) -> (i32, i32) {
        let (x, y) = ((self.position as f64) / 50.0 * 2.0 * PI).sin_cos();
        let scale = |a| (a * 10.0) as i32;
        (scale(x), scale(y))
    }
}

#[derive(Default)]
pub struct DummyScript {
    state: DummyState,
    tokens: VecDeque<Token>,
}

impl MonkeyScript for DummyScript {
    fn poll_action(&mut self) -> Option<Token> {
        self.tokens.pop_front()
    }

    fn poll_timeout(&self) -> Option<Instant> {
        Some(self.state.updated + Duration::from_millis(20))
    }

    fn handle_timeout(&mut self, now: Instant) {
        while now.duration_since(self.state.updated) > Duration::from_millis(20) {
            self.state.increment();
            self.tokens.push_back(self.state.move_to());
        }
    }
}

pub async fn start_script() -> anyhow::Result<()> {
    let mut script = DummyScript::default();
    let mut engine = engine::default_engine()?;
    loop {
        if let Some(token) = script.poll_action() {
            engine.execute(&token)?;
            continue;
        }
        if let Some(time) = script.poll_timeout() {
            wasm_timer::Delay::new_at(time).await?;
        }
        script.handle_timeout(Instant::now());
    }
}
