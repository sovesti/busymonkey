use std::{collections::VecDeque, f64::consts::PI, time::Duration};

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use super::Instant;

use crate::monkey::{
    engine::{self, Agent},
    model::{
        Coordinate, Token,
        parameters::{DynScript, Parameter, ParameterSchema, ScriptFactory},
    },
    script::MonkeyScript,
};

struct DummyState {
    updated: Instant,
    finished: Instant,
    position: usize,
}

impl Default for DummyState {
    fn default() -> Self {
        Self {
            finished: Instant::now() + Duration::from_hours(1),
            updated: Instant::now(),
            position: 0,
        }
    }
}

impl DummyState {
    fn increment(&mut self) {
        self.position += 1;
        if self.position % 50 == 0 {
            self.finished = Instant::now();
        }
        self.updated = self.updated + Duration::from_millis(20);
    }

    fn move_to(&self, speed: f64) -> Token {
        let (x, y) = self.coordinates(speed);
        Token::MoveMouse(x, y, Coordinate::Rel)
    }

    fn coordinates(&self, speed: f64) -> (i32, i32) {
        let (x, y) = ((self.position as f64) / 50.0 * 2.0 * PI).sin_cos();
        let scale = |a| (a * speed) as i32;
        (scale(x), scale(y))
    }
}

#[derive(Default)]
pub struct DummyScript {
    parameters: DummyParameters,
    state: DummyState,
    tokens: VecDeque<Token>,
}

impl DummyScript {
    pub fn new(raw: serde_json::Value) -> anyhow::Result<Box<dyn MonkeyScript>> {
        Ok(Box::new(Self {
            parameters: serde_json::from_value(raw).unwrap_or_default(),
            ..Default::default()
        }))
    }

    fn interval(&self) -> Duration {
        self.parameters.interval
    }
}

impl MonkeyScript for DummyScript {
    fn poll_action(&mut self) -> Option<Token> {
        self.tokens.pop_front()
    }

    fn poll_timeout(&self) -> Option<Instant> {
        Some(
            (self.state.updated + Duration::from_millis(20))
                .min(self.state.finished + self.interval()),
        )
    }

    fn handle_timeout(&mut self, now: Instant) {
        if now.duration_since(self.state.finished) > self.interval() {
            return;
        }
        while now.duration_since(self.state.updated) > Duration::from_millis(20) {
            self.state.increment();
            self.tokens
                .push_back(self.state.move_to(self.parameters.speed));
        }
    }
}

pub async fn start_dummy_script(mut script: DynScript) -> anyhow::Result<()> {
    let mut engine = engine::default_engine()?;
    loop {
        if let Some(token) = script.poll_action() {
            engine.execute(&token)?;
            continue;
        }
        if let Some(time) = script.poll_timeout() {
            crate::monkey::sleep_until(time).await;
        }
        script.handle_timeout(Instant::now());
    }
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Props)]
pub struct DummyParameters {
    speed: f64,
    interval: Duration,
}

impl Default for DummyParameters {
    fn default() -> Self {
        Self {
            speed: 10.0,
            interval: Duration::from_secs(1),
        }
    }
}

inventory::submit! {
    ScriptFactory::new("dummy", || {
        vec![
            Parameter::new(
                "speed",
                "Movement speed, px/s",
                ParameterSchema::float(0.0, 100.0),
            ),
            Parameter::new(
                "interval",
                "Interval between circles",
                ParameterSchema::duration(Duration::ZERO, Duration::from_secs(10)),
            ),
        ]
    }, DummyScript::new)
}
