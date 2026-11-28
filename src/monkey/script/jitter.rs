use std::{collections::VecDeque, time::Duration};

use rand::{Rng, RngExt};

use crate::monkey::{
    Instant,
    model::{Coordinate, Token},
    script::TokenProto,
};

pub struct RandomJitter<S, R> {
    inner: S,
    rng: R,
    max_delay: Duration,
    max_offset: i32,
    now: Instant,
    queue: VecDeque<(Instant, Token)>,
    offset: (i32, i32),
}

impl<S: TokenProto, R: Rng> RandomJitter<S, R> {
    pub fn new(inner: S, rng: R, max_delay: Duration, max_offset: i32, now: Instant) -> Self {
        Self {
            inner,
            rng,
            max_delay,
            max_offset,
            now,
            queue: VecDeque::new(),
            offset: (0, 0),
        }
    }

    fn pull(&mut self) {
        while let Some(token) = self.inner.poll_action() {
            let token = self.shift(token);
            let earliest = self.queue.back().map_or(self.now, |(at, _)| *at);
            let at = (self.now + self.delay()).max(earliest);
            self.queue.push_back((at, token));
        }
    }

    fn delay(&mut self) -> Duration {
        self.max_delay.mul_f64(self.rng.random_range(0.0..=1.0))
    }

    fn shift(&mut self, token: Token) -> Token {
        let Token::MoveMouse(x, y, coordinate) = token else {
            return token;
        };
        let previous = self.offset;
        self.offset = (self.random_offset(), self.random_offset());
        match coordinate {
            Coordinate::Abs => Token::MoveMouse(x + self.offset.0, y + self.offset.1, coordinate),
            Coordinate::Rel => Token::MoveMouse(
                x + self.offset.0 - previous.0,
                y + self.offset.1 - previous.1,
                coordinate,
            ),
        }
    }

    fn random_offset(&mut self) -> i32 {
        self.rng.random_range(-self.max_offset..=self.max_offset)
    }
}

impl<S: TokenProto, R: Rng> TokenProto for RandomJitter<S, R> {
    fn poll_action(&mut self) -> Option<Token> {
        self.pull();
        match self.queue.front() {
            Some((at, _)) if *at <= self.now => self.queue.pop_front().map(|(_, token)| token),
            _ => None,
        }
    }

    fn poll_timeout(&mut self) -> Option<Instant> {
        let queued = self.queue.front().map(|(at, _)| *at);
        match (self.inner.poll_timeout(), queued) {
            (Some(inner), Some(queued)) => Some(inner.min(queued)),
            (inner, queued) => inner.or(queued),
        }
    }

    fn handle_timeout(&mut self, now: Instant) {
        self.now = now;
        self.inner.handle_timeout(now);
    }

    fn is_finished(&mut self) -> bool {
        self.queue.is_empty() && self.inner.is_finished()
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::VecDeque, time::Duration};

    use rand::{SeedableRng, rngs::StdRng};

    use super::RandomJitter;
    use crate::monkey::{
        Instant,
        model::{Button, Coordinate, Direction, Token},
        script::TokenProto,
    };

    const MAX_DELAY: Duration = Duration::from_millis(5);
    const MAX_OFFSET: i32 = 3;

    struct Scripted {
        steps: VecDeque<(Instant, Token)>,
        now: Instant,
    }

    impl TokenProto for Scripted {
        fn poll_action(&mut self) -> Option<Token> {
            match self.steps.front() {
                Some((at, _)) if *at <= self.now => self.steps.pop_front().map(|(_, token)| token),
                _ => None,
            }
        }

        fn poll_timeout(&mut self) -> Option<Instant> {
            self.steps.front().map(|(at, _)| *at)
        }

        fn handle_timeout(&mut self, now: Instant) {
            self.now = now;
        }

        fn is_finished(&mut self) -> bool {
            self.steps.is_empty()
        }
    }

    fn every_20ms(start: Instant, tokens: Vec<Token>) -> Vec<(Instant, Token)> {
        tokens
            .into_iter()
            .enumerate()
            .map(|(i, token)| (start + Duration::from_millis(20 * i as u64), token))
            .collect()
    }

    fn scripted(start: Instant, steps: &[(Instant, Token)]) -> Scripted {
        Scripted {
            steps: steps.iter().cloned().collect(),
            now: start,
        }
    }

    fn jittered(
        start: Instant,
        steps: &[(Instant, Token)],
        max_delay: Duration,
        max_offset: i32,
        seed: u64,
    ) -> RandomJitter<Scripted, StdRng> {
        RandomJitter::new(
            scripted(start, steps),
            StdRng::seed_from_u64(seed),
            max_delay,
            max_offset,
            start,
        )
    }

    fn run(mut script: impl TokenProto, mut now: Instant) -> Vec<(Instant, Token)> {
        let mut out = Vec::new();
        loop {
            while let Some(token) = script.poll_action() {
                out.push((now, token));
            }
            match script.poll_timeout() {
                Some(next) => {
                    now = now.max(next);
                    script.handle_timeout(now);
                }
                None => return out,
            }
        }
    }

    fn moves(coordinate: Coordinate) -> Vec<Token> {
        (0..50)
            .map(|i| Token::MoveMouse(10 + i % 7, 5 - i % 3, coordinate))
            .collect()
    }

    fn position(token: &Token) -> (i32, i32) {
        match token {
            Token::MoveMouse(x, y, _) => (*x, *y),
            other => panic!("not a move: {other:?}"),
        }
    }

    #[test]
    fn zero_jitter_keeps_script_unchanged() {
        let start = Instant::now();
        let steps = every_20ms(start, moves(Coordinate::Abs));
        let out = run(jittered(start, &steps, Duration::ZERO, 0, 1), start);
        assert_eq!(out, steps);
    }

    #[test]
    fn delays_are_bounded_and_keep_order() {
        let start = Instant::now();
        let steps = every_20ms(start, moves(Coordinate::Abs));
        let out = run(jittered(start, &steps, MAX_DELAY, 0, 2), start);
        assert_eq!(out.len(), steps.len());
        for ((expected_at, expected), (at, token)) in steps.iter().zip(&out) {
            assert_eq!(token, expected);
            assert!(*at >= *expected_at && *at <= *expected_at + MAX_DELAY);
        }
        assert!(out.iter().zip(&steps).any(|((at, _), (expected_at, _))| at != expected_at));
    }

    #[test]
    fn absolute_moves_stay_within_offset() {
        let start = Instant::now();
        let steps = every_20ms(start, moves(Coordinate::Abs));
        let out = run(jittered(start, &steps, Duration::ZERO, MAX_OFFSET, 3), start);
        for ((_, expected), (_, token)) in steps.iter().zip(&out) {
            let (ex, ey) = position(expected);
            let (x, y) = position(token);
            assert!((x - ex).abs() <= MAX_OFFSET && (y - ey).abs() <= MAX_OFFSET);
        }
    }

    #[test]
    fn relative_moves_do_not_drift() {
        let start = Instant::now();
        let steps = every_20ms(start, moves(Coordinate::Rel));
        let out = run(jittered(start, &steps, Duration::ZERO, MAX_OFFSET, 4), start);
        let mut expected = (0, 0);
        let mut actual = (0, 0);
        for ((_, expected_token), (_, token)) in steps.iter().zip(&out) {
            let (ex, ey) = position(expected_token);
            let (x, y) = position(token);
            expected = (expected.0 + ex, expected.1 + ey);
            actual = (actual.0 + x, actual.1 + y);
            assert!((actual.0 - expected.0).abs() <= MAX_OFFSET);
            assert!((actual.1 - expected.1).abs() <= MAX_OFFSET);
        }
    }

    #[test]
    fn other_tokens_are_not_moved() {
        let start = Instant::now();
        let click = Token::Button(Button::Left, Direction::Click);
        let steps = every_20ms(start, vec![click.clone(), click.clone()]);
        let out = run(jittered(start, &steps, MAX_DELAY, MAX_OFFSET, 5), start);
        let tokens: Vec<_> = out.into_iter().map(|(_, token)| token).collect();
        assert_eq!(tokens, [click.clone(), click]);
    }

    #[test]
    fn finishes_only_after_queue_is_drained() {
        let start = Instant::now();
        let click = Token::Button(Button::Left, Direction::Click);
        let steps = vec![(start, click)];
        let mut script = jittered(start, &steps, Duration::from_secs(1), 0, 6);
        assert_eq!(script.poll_action(), None);
        assert!(script.inner.is_finished());
        assert!(!script.is_finished());
        script.handle_timeout(start + Duration::from_secs(1));
        assert!(script.poll_action().is_some());
        assert!(script.is_finished());
    }

    #[test]
    fn same_seed_gives_same_result() {
        let start = Instant::now();
        let steps = every_20ms(start, moves(Coordinate::Abs));
        let first = run(jittered(start, &steps, MAX_DELAY, MAX_OFFSET, 7), start);
        let second = run(jittered(start, &steps, MAX_DELAY, MAX_OFFSET, 7), start);
        assert_eq!(first, second);
    }
}
