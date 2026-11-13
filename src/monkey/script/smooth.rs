use std::{ops::Range, time::Duration};

use anyhow::anyhow;
use dioxus::prelude::*;
use rand::{RngExt, rngs::SmallRng};
use serde::{Deserialize, Serialize};

use crate::monkey::{
    model::parameters::{Parameter, ParameterSchema, ScriptFactory},
    script::{DynScript, command::{Command, MicroCommand}},
};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Props)]
struct SmoothWanderCfg {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
    delay: Duration,
}

impl Default for SmoothWanderCfg {
    fn default() -> Self {
        Self {
            x: 200f64,
            y: 200f64,
            w: 1000f64,
            h: 1000f64,
            delay: Duration::from_secs(1),
        }
    }
}

impl SmoothWanderCfg {
    fn ranges(&self) -> (Range<i32>, Range<i32>) {
        let x = self.x.ceil() as i32;
        let y = self.y.ceil() as i32;
        let w = self.w.ceil() as i32;
        let h = self.h.ceil() as i32;
        (x..x + w, y..y + h)
    }
}

struct SmoothWanderScript {
    cfg: SmoothWanderCfg,
    rng: SmallRng,
    is_delay: bool,
}

impl SmoothWanderScript {
    fn new(cfg: SmoothWanderCfg) -> Self {
        Self {
            cfg,
            rng: rand::make_rng(),
            is_delay: false,
        }
    }
}

impl Iterator for SmoothWanderScript {
    type Item = Command;

    fn next(&mut self) -> Option<Command> {
        self.is_delay = !self.is_delay;
        if self.is_delay {
            println!("idle");
            Some(Command::Single(MicroCommand::Idle(self.cfg.delay)))
        } else {
            let (range_x, range_y) = self.cfg.ranges();
            let x = self.rng.random_range(range_x);
            let y = self.rng.random_range(range_y);
            println!("move to rect");
            Some(Command::Single(MicroCommand::MoveToRect(
                x - 5,
                y - 5,
                10,
                10,
            )))
        }
    }
}

fn parse_cfg(value: serde_json::Value) -> anyhow::Result<SmoothWanderCfg> {
    Ok(SmoothWanderCfg {
        x: value.get("x").ok_or(anyhow!("No x value"))?.as_f64().ok_or(anyhow!("x value not float"))?,
        y: value.get("y").ok_or(anyhow!("No y value"))?.as_f64().ok_or(anyhow!("y value not float"))?,
        w: value.get("w").ok_or(anyhow!("No w value"))?.as_f64().ok_or(anyhow!("w value not float"))?,
        h: value.get("h").ok_or(anyhow!("No h value"))?.as_f64().ok_or(anyhow!("h value not float"))?,
        delay: Duration::from_secs_f64(value.get("delay").ok_or(anyhow!("No delay value"))?.as_f64().ok_or(anyhow!("delay value not float"))?),
    })
}

fn smooth_wander(value: serde_json::Value) -> anyhow::Result<DynScript> {
    println!("assemble smooth wander script from settings\n{value}");
    let cfg = parse_cfg(value);
    println!("gives: {cfg:?}");
    Ok(Box::new(SmoothWanderScript::new(cfg?)))
}

inventory::submit! {
    ScriptFactory::new("dummy", || {
        vec![
            Parameter::new(
                "x",
                "Coordinate of top left corner of wander area, px",
                ParameterSchema::float(0.0, 1920.0),
            ),
            Parameter::new(
                "y",
                "Coordinate of top left corner of wander area, px",
                ParameterSchema::float(0.0, 1080.0),
            ),
            Parameter::new(
                "w",
                "Wander area width, px",
                ParameterSchema::float(0.0, 1920.0),
            ),
            Parameter::new(
                "h",
                "Wander area height, px",
                ParameterSchema::float(0.0, 1080.0),
            ),
            Parameter::new(
                "delay",
                "Delay between wanders",
                ParameterSchema::duration(Duration::ZERO, Duration::from_secs(10)),
            ),
        ]
    }, smooth_wander)
}
