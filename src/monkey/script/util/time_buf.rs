use std::{collections::VecDeque, time::Duration};

use crate::monkey::Instant;

/// Buffer that stores objects with timestamps, and returns the objects with
/// the lates timestamp no earlier than `max` duration before last received
/// timestamp
pub struct TimeBuf<T> {
    max: Duration,
    from: Instant,
    buf: VecDeque<(T, Instant)>,
}

impl<T> TimeBuf<T> {
    pub fn new(max: Duration, now: Instant) -> Self {
        Self {
            max,
            from: now - max,
            buf: VecDeque::default(),
        }
    }

    pub fn handle_timeout(&mut self, now: Instant) {
        self.from = now - self.max;
        while let Some((obj, time)) = self.buf.pop_back() {
            if let Some((_, next_time)) = self.buf.back()
                && self.from >= *next_time
            {
                continue;
            }
            self.buf.push_back((obj, time));
            break;
        }
    }

    pub fn push(&mut self, obj: T, at: Instant) {
        self.buf.push_front((obj, at));
        self.handle_timeout(at);
    }

    pub fn back(&mut self) -> Option<&T> {
        self.buf.back().and_then(
            |(obj, time)| {
                if self.from >= *time { Some(obj) } else { None }
            },
        )
    }

    pub fn pop_back(&mut self) -> Option<T> {
        let Some((obj, time)) = self.buf.pop_back() else {
            return None;
        };
        if self.from >= time {
            Some(obj)
        } else {
            self.buf.push_back((obj, time));
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_matches;

    #[test]
    fn smoke() {
        let now = Instant::now();
        let sec = Duration::from_secs(1);
        let mut buf = TimeBuf::<usize>::new(Duration::from_secs(3), now);
        assert_matches!(buf.back(), None);
        buf.push(42, now + sec);
        assert_matches!(buf.back(), None);
        buf.handle_timeout(now + 2 * sec);
        assert_matches!(buf.back(), None);
        buf.push(43, now + 4 * sec);
        assert_eq!(buf.back(), Some(&42));
        buf.handle_timeout(now + 6 * sec);
        assert_eq!(buf.back(), Some(&42));
        buf.push(44, now + 7 * sec);
        assert_eq!(buf.back(), Some(&43));
        buf.push(45, now + 100 * sec);
        assert_eq!(buf.back(), Some(&44));
    }
}
