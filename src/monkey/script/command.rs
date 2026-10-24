use crate::monkey::script::MonkeyScript;
use enigo::{Button, Direction, Key};
use std::iter::{Iterator, once, repeat};

/// Middle-level commands to be turned into sequence of `Token`s
pub enum MicroCommand {
    MoveToRect(i32, i32, i32, i32),
    MouseButton(Button, Direction),
    Key(Key, Direction),
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
        }
    }
}

/// A state, built from a single Command, yielding a sequence of `Token`s,
/// handling timeouts and current mouse position, and reporting finish of the
/// command execution
pub trait CommandState: MonkeyScript {
    fn is_finished(&mut self) -> bool;
}
