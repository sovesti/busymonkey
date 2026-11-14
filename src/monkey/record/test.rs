use std::sync::{Arc, Mutex, MutexGuard};
use std::time::Duration;

use tokio_stream::{StreamExt, iter, pending};

use crate::monkey::model::{Button, Coordinate, Direction, TimedToken, Token};
use crate::monkey::record::storage::RecordingStorage;
use crate::monkey::record::storage::linear::Linear;
use crate::monkey::record::storage::truncate::TruncateMoves;
use crate::monkey::record::{InputChannel, record_channel};

use super::DynTokenStream;

#[tokio::test]
pub async fn records_something() {
    let storage = record_to_fake(
        Linear::new(),
        FakeChannel(vec![Token::Button(Button::Left, Direction::Click)]),
    )
    .await;
    assert_eq!(1, storage.lock().all().len());
}

#[tokio::test]
pub async fn truncates_moves() {
    let storage = record_to_fake(
        TruncateMoves::new(Linear::new()),
        FakeChannel(vec![
            Token::MoveMouse(0, 1, Coordinate::Abs),
            Token::Button(Button::Left, Direction::Click),
            Token::MoveMouse(0, 2, Coordinate::Abs),
            Token::MoveMouse(0, 3, Coordinate::Abs),
            Token::MoveMouse(0, 4, Coordinate::Abs),
        ]),
    )
    .await;
    assert_eq!(
        vec![
            Token::MoveMouse(0, 1, Coordinate::Abs),
            Token::Button(Button::Left, Direction::Click),
        ],
        tokens(storage.lock().delegate())
    );
}

#[tokio::test]
pub async fn flushes_on_non_move() {
    let storage = record_to_fake(
        TruncateMoves::new(Linear::new()),
        FakeChannel(vec![
            Token::MoveMouse(0, 1, Coordinate::Abs),
            Token::Button(Button::Left, Direction::Click),
            Token::MoveMouse(0, 2, Coordinate::Abs),
            Token::Button(Button::Left, Direction::Click),
        ]),
    )
    .await;
    assert_eq!(
        vec![
            Token::MoveMouse(0, 1, Coordinate::Abs),
            Token::Button(Button::Left, Direction::Click),
            Token::MoveMouse(0, 2, Coordinate::Abs),
            Token::Button(Button::Left, Direction::Click),
        ],
        tokens(storage.lock().delegate())
    );
}

fn tokens(storage: &Linear) -> Vec<Token> {
    storage
        .all()
        .iter()
        .map(TimedToken::token)
        .cloned()
        .collect::<Vec<_>>()
}

async fn record_to_fake<S, C>(storage: S, channel: C) -> FakeStorage<S>
where
    S: RecordingStorage + Clone + Send,
    C: InputChannel + 'static,
{
    let storage = FakeStorage::new(storage);
    tokio::select! {
        _ = record_channel(storage.clone(), Arc::new(channel)) => {},
        _ = tokio::time::sleep(Duration::from_millis(1000)) => {},
    };
    storage
}

#[derive(Clone)]
struct FakeChannel(Vec<Token>);

impl InputChannel for FakeChannel {
    fn subscribe(&self) -> DynTokenStream {
        Box::pin(iter(self.0.clone().into_iter().map(timed).map(Ok)).chain(pending()))
    }
}

fn timed(token: Token) -> TimedToken {
    TimedToken::new(Duration::ZERO, token)
}

#[derive(Clone)]
struct FakeStorage<S>(Arc<Mutex<S>>);

impl<S> FakeStorage<S> {
    fn new(inner: S) -> Self {
        Self(Arc::new(Mutex::new(inner)))
    }

    fn lock(&self) -> MutexGuard<'_, S> {
        self.0.lock().unwrap()
    }
}

impl<S: RecordingStorage> RecordingStorage for FakeStorage<S> {
    fn add(&mut self, token: anyhow::Result<TimedToken>) {
        self.lock().add(token);
    }

    fn active(&self) -> bool {
        self.lock().active()
    }

    fn finish(&mut self) {
        self.lock().finish();
    }
}
