use std::{ops::Range, time::Duration};

use anyhow::anyhow;
use dioxus::prelude::*;
use rand::{RngExt, rngs::SmallRng};
use serde::{Deserialize, Serialize};

use crate::monkey::model::parameters::{Parameter, ParameterSchema, ScriptFactory, ScriptMeta};
use crate::monkey::script::DynScript;
use crate::monkey::script::command::{Command, MicroCommand};

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

fn get_f64_key(key: &str, value: &serde_json::Value) -> anyhow::Result<f64> {
    value
        .get(key)
        .ok_or(anyhow!("No {key} value"))?
        .as_str()
        .ok_or(anyhow!("{key} is not string"))?
        .parse::<f64>()
        .context("Value is not correct f64")
}

fn parse_cfg(value: serde_json::Value) -> anyhow::Result<SmoothWanderCfg> {
    Ok(SmoothWanderCfg {
        x: get_f64_key("x", &value)?,
        y: get_f64_key("y", &value)?,
        w: get_f64_key("w", &value)?,
        h: get_f64_key("h", &value)?,
        delay: Duration::from_secs_f64(get_f64_key("delay", &value)? / 1000f64),
    })
}

fn smooth_wander(value: serde_json::Value) -> anyhow::Result<DynScript> {
    Ok(Box::new(SmoothWanderScript::new(parse_cfg(value)?)))
}

inventory::submit! {
    ScriptFactory::new_meta("dummy",
        || { ScriptMeta::new("Smooth mouse moving") }, || {
        vec![
            Parameter::new(
                "x",
                "Top left corner x",
                ParameterSchema::float(0.0, 1920.0),
            ),
            Parameter::new(
                "y",
                "Top left corner y",
                ParameterSchema::float(0.0, 1080.0),
            ),
            Parameter::new(
                "w",
                "Wander area width",
                ParameterSchema::float(0.0, 1920.0),
            ),
            Parameter::new(
                "h",
                "Wander area height",
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
