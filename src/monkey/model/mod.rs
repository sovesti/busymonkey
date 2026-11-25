pub(crate) mod parameters;

use std::fmt::{self, Display};
use std::time::Duration;

use chrono::{DateTime, Local};
pub use enigo::{Axis, Button, Coordinate, Direction, Key, agent::Token};

#[derive(Clone, Debug, PartialEq)]
pub struct TimedToken {
    timestamp: Duration,
    token: Token,
}

impl TimedToken {
    pub fn new(timestamp: Duration, token: Token) -> Self {
        Self { timestamp, token }
    }

    pub fn token(&self) -> &Token {
        &self.token
    }

    pub fn timestamp(&self) -> Duration {
        self.timestamp
    }

    pub fn format_time(&self) -> String {
        let local: DateTime<Local> = (DateTime::UNIX_EPOCH + self.timestamp()).into();
        local.format("%H:%M:%S").to_string()
    }
}

impl Display for TimedToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.token() {
            Token::Key(key, Direction::Press) => write!(f, "Press {}", key_name(key)),
            Token::Key(key, _) => write!(f, "Release {}", key_name(key)),
            Token::Button(button, Direction::Press) => write!(f, "Press {button:?} mouse button"),
            Token::Button(button, _) => write!(f, "Release {button:?} mouse button"),
            Token::MoveMouse(x, y, _) => write!(f, "Move mouse to ({x}, {y})"),
            Token::Scroll(x, axis) => format_scroll(f, x, axis),
            _ => write!(f, ""),
        }
    }
}

fn key_name(key: &Key) -> String {
    match key {
        Key::Unicode(ch) => ch.to_uppercase().collect(),
        Key::LControl => "Left Ctrl".into(),
        Key::RControl => "Right Ctrl".into(),
        Key::LShift => "Left Shift".into(),
        Key::RShift => "Right Shift".into(),
        #[cfg(feature = "desktop")]
        Key::LMenu => "Left Alt".into(),
        #[cfg(feature = "desktop")]
        Key::ModeChange => "AltGr".into(),
        Key::Meta => "Super".into(),
        Key::Return => "Enter".into(),
        Key::LeftArrow => "Left Arrow".into(),
        Key::RightArrow => "Right Arrow".into(),
        Key::UpArrow => "Up Arrow".into(),
        Key::DownArrow => "Down Arrow".into(),
        _ => format!("{key:?}"),
    }
}

fn format_scroll(f: &mut fmt::Formatter<'_>, x: &i32, axis: &Axis) -> fmt::Result {
    write!(
        f,
        "Scroll {} pixels {}",
        x.abs(),
        match (x.is_positive(), axis) {
            (true, Axis::Horizontal) => "right",
            (true, Axis::Vertical) => "up",
            (false, Axis::Horizontal) => "left",
            (false, Axis::Vertical) => "down",
        }
    )
}
