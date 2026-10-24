use std::time::Duration;

const TIME_TO_MAX_SPEED: Duration = Duration::from_millis(500);

/// User metrics needed for personalized activity simulation
///
/// User struct is to be build with a fluent builder `UserBuilder`, as
/// additional fields may be added
#[non_exhaustive]
#[derive(Clone, Copy)]
pub struct User {
    pub reaction_time: Duration,
    pub poll_rate: Duration,
    // Max mouse move speed in pixels per second
    pub max_speed: f32,
}

impl User {
    pub fn max_acceleration(&self) -> f32 {
        self.max_speed / TIME_TO_MAX_SPEED.as_secs() as f32
    }
}

/// Fluent builder for `User`
pub struct UserBuilder {
    reaction_time: Duration,
    poll_rate: Duration,
    max_speed: f32,
}

impl Default for UserBuilder {
    fn default() -> Self {
        Self {
            reaction_time: Duration::from_millis(250),
            poll_rate: Duration::from_millis(2),
            max_speed: 1000f32,
        }
    }
}

impl UserBuilder {
    pub fn reaction_time(mut self, reaction_time: Duration) -> Self {
        self.reaction_time = reaction_time;
        self
    }

    pub fn poll_rate(mut self, mouse_movement_freq: Duration) -> Self {
        self.poll_rate = mouse_movement_freq;
        self
    }

    pub fn max_speed(mut self, max_speed: f32) -> Self {
        self.max_speed = max_speed;
        self
    }

    pub fn build(self) -> User {
        let Self {
            reaction_time,
            poll_rate,
            max_speed,
        } = self;
        User {
            reaction_time,
            poll_rate,
            max_speed,
        }
    }
}
