use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::UNIX_EPOCH;
use tokio::sync::broadcast;

use anyhow::{Context, bail};
use dioxus::logger::tracing;
use dioxus::prelude::*;
use enigo::{Axis, Coordinate, Direction};
use rdev::EventType;
use tokio_stream::StreamExt;
use tokio_stream::wrappers::BroadcastStream;

use crate::monkey::model::{Button, Key, TimedToken, Token};
use crate::monkey::record::keycodes::rdev_to_monkey;
use crate::monkey::record::{DynInputChannel, InputChannel};

use super::DynTokenStream;

pub fn provide_rdev() {
    provide_context(RdevInputChannel::start() as DynInputChannel);
}

pub struct RdevInputChannel {
    _handle: JoinHandle<()>,
    tx: broadcast::Sender<TimedToken>,
}

impl RdevInputChannel {
    pub fn start() -> Arc<Self> {
        let (tx, _) = broadcast::channel(64);
        let state = RdevState::new(tx.clone());
        let _handle = thread::spawn(move || state.spawn());
        Arc::new(Self { _handle, tx })
    }
}

impl InputChannel for RdevInputChannel {
    fn subscribe(&self) -> DynTokenStream {
        Box::pin(BroadcastStream::new(self.tx.subscribe()).map(|item| item.context("rdev event")))
    }
}

struct RdevState {
    tx: broadcast::Sender<TimedToken>,
}

impl RdevState {
    fn new(tx: broadcast::Sender<TimedToken>) -> Self {
        Self { tx }
    }

    fn spawn(mut self) {
        if let Err(err) = rdev::listen(move |event| self.accept(event)) {
            tracing::error!("error while listening: {err:?}");
        }
    }

    fn accept(&mut self, event: rdev::Event) {
        if let Err(err) = self
            .dispatch(event.clone())
            .with_context(|| format!("recording {event:?}"))
        {
            tracing::warn!("{err:?}");
        }
    }

    fn dispatch(&mut self, event: rdev::Event) -> anyhow::Result<()> {
        if self.tx.receiver_count() == 0 {
            return Ok(());
        }
        let tokens: Vec<_> = RdevEvent(event).try_into()?;
        tokens
            .into_iter()
            .try_for_each(|token| self.tx.send(token).map(|_| ()))?;
        Ok(())
    }
}

struct RdevEvent(rdev::Event);

const WHEEL_DELTA: i32 = 120;

impl TryFrom<RdevEvent> for Vec<TimedToken> {
    type Error = anyhow::Error;

    fn try_from(RdevEvent(event): RdevEvent) -> anyhow::Result<Self> {
        let tokens: &[Token] = match event.event_type {
            EventType::KeyPress(key) => &[key_token(key, Direction::Press)?],
            EventType::KeyRelease(key) => &[key_token(key, Direction::Release)?],
            EventType::ButtonPress(button) => &[button_token(button, Direction::Press)?],
            EventType::ButtonRelease(button) => &[button_token(button, Direction::Release)?],
            EventType::MouseMove { x, y } => {
                &[Token::MoveMouse(x as i32, y as i32, Coordinate::Abs)]
            }
            EventType::Wheel { delta_x, delta_y } => &[
                Token::Scroll(delta_x as i32 * WHEEL_DELTA, Axis::Horizontal),
                Token::Scroll(delta_y as i32 * WHEEL_DELTA, Axis::Vertical),
            ],
        };
        let timestamp = event.time.duration_since(UNIX_EPOCH)?;
        Ok(tokens
            .into_iter()
            .map(|token| TimedToken::new(timestamp, token.clone()))
            .collect())
    }
}

fn key_token(key: rdev::Key, direction: Direction) -> anyhow::Result<Token> {
    Ok(Token::Key(RdevKey(key).try_into()?, direction))
}

fn button_token(key: rdev::Button, direction: Direction) -> anyhow::Result<Token> {
    Ok(Token::Button(RdevButton(key).try_into()?, direction))
}

struct RdevKey(rdev::Key);

impl TryFrom<RdevKey> for Key {
    type Error = anyhow::Error;

    fn try_from(RdevKey(key): RdevKey) -> anyhow::Result<Self> {
        rdev_to_monkey(key)
    }
}

struct RdevButton(rdev::Button);

impl TryFrom<RdevButton> for Button {
    type Error = anyhow::Error;

    fn try_from(RdevButton(button): RdevButton) -> anyhow::Result<Self> {
        Ok(match button {
            rdev::Button::Left => Button::Left,
            rdev::Button::Right => Button::Right,
            rdev::Button::Middle => Button::Middle,
            rdev::Button::Unknown(4) => Button::Back,
            rdev::Button::Unknown(5) => Button::Forward,
            rdev::Button::Unknown(id) => bail!("unknown rdev button {id}"),
        })
    }
}

#[cfg(test)]
mod test {
    use std::time::{Duration, UNIX_EPOCH};

    use crate::monkey::model::{self, Button, Direction, TimedToken, Token};
    use crate::monkey::record::rdev::{RdevEvent, RdevKey};

    #[test]
    fn key_converts() {
        assert_eq!(model::Key::A, RdevKey(rdev::Key::KeyA).try_into().unwrap());
    }

    #[test]
    fn token_converts() {
        let actual: Vec<TimedToken> = RdevEvent(rdev::Event {
            time: UNIX_EPOCH,
            name: None,
            event_type: rdev::EventType::ButtonPress(rdev::Button::Middle),
        })
        .try_into()
        .unwrap();
        assert_eq!(
            vec![TimedToken::new(
                Duration::ZERO,
                Token::Button(Button::Middle, Direction::Press),
            )],
            actual
        );
    }
}
