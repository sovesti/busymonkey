use enigo::{Axis, Button, Direction, Key, agent::Token};
use std::{
    iter::{Iterator, once, repeat},
    time::Duration,
};

/// Middle-level commands to be turned into sequence of `Token`s
pub enum MicroCommand {
    Idle(Duration),
    MoveToPosition(i32, i32, Duration),
    MouseButton(Button, Direction),
    Key(Key, Direction),
    Scroll(i32, Axis),
}

impl MicroCommand {
    fn click() -> Self {
        Self::MouseButton(Button::Left, Direction::Click)
    }

    fn right_click() -> Self {
        Self::MouseButton(Button::Right, Direction::Click)
    }
}

/// High-level commands, a sequence of commands, annotated with
/// additional semantics, which interpreter may or may not use.
///
/// Must have a fallback to a sequence of microcommands (`into_micros` method)
#[non_exhaustive]
pub enum Command {
    Single(MicroCommand),
    Shortcut(Vec<Key>),
    Text(String),
}

impl Command {
    /// Fallback to an iterator over microcommands
    pub fn into_micros(self) -> Box<dyn Iterator<Item = MicroCommand>> {
        match self {
            Self::Single(cmd) => Box::new(once(cmd)),
            Self::Shortcut(keys) => Box::new(
                vec![Direction::Press, Direction::Release]
                    .into_iter()
                    .zip(repeat(keys))
                    .flat_map(move |(dir, keys)| {
                        keys.into_iter().map(move |key| MicroCommand::Key(key, dir))
                    }),
            ),
            Command::Text(txt) => todo!(),
        }
    }
}

pub trait Script: Iterator<Item = Command> {}

pub type DynScript = Box<dyn Script>;

impl<T: Iterator<Item = Command>> Script for T {}
