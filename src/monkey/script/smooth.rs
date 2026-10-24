use rand::{RngExt, rngs::ThreadRng};

use super::super::model::Token;
use crate::monkey::{
    Instant, engine,
    script::{
        MonkeyScript,
        command::{Command, CommandState, MicroCommand},
        state::{once::Once, smooth_move::smooth_move},
        user::{User, UserBuilder},
    },
};
use enigo::agent::Agent;

pub struct SmoothState<'a> {
    user: &'a User,
    iter: Box<dyn Iterator<Item = MicroCommand>>,
    state: Option<Box<dyn CommandState + 'a>>,
    now: Instant,
}

impl<'a> SmoothState<'a> {
    pub fn new(user: &'a User, command: Command, now: Instant) -> Self {
        let mut iter = command.into_micros();
        let state = iter.next().map(|cmd| micro_command_state(user, now, cmd));
        Self {
            user,
            iter,
            state,
            now,
        }
    }
}

fn micro_command_state<'a>(
    user: &'a User,
    now: Instant,
    cmd: MicroCommand,
) -> Box<dyn CommandState + 'a> {
    match cmd {
        MicroCommand::MoveToRect(x, y, w, h) => Box::new(smooth_move(user, now, x, y, w, h)),
        MicroCommand::Key(key, dir) => Box::new(Once::new(user, Token::Key(key, dir), now)),
        MicroCommand::MouseButton(btn, dir) => {
            Box::new(Once::new(user, Token::Button(btn, dir), now))
        }
    }
}

impl<'a> MonkeyScript for SmoothState<'a> {
    fn poll_action(&mut self) -> Option<Token> {
        loop {
            match &mut self.state {
                Some(state) => {
                    if !state.is_finished() {
                        return state.poll_action();
                    }
                }
                None => return None,
            }
            self.state = self
                .iter
                .next()
                .map(|cmd| micro_command_state(self.user, self.now, cmd));
        }
    }

    fn poll_timeout(&mut self) -> Option<Instant> {
        self.state.as_mut().and_then(|state| state.poll_timeout())
    }

    fn handle_timeout(&mut self, now: Instant) {
        self.now = now;
        self.state.as_mut().map(|state| state.handle_timeout(now));
    }
}

impl<'a> CommandState for SmoothState<'a> {
    fn is_finished(&mut self) -> bool {
        matches!(self.state, None)
    }
}

fn random_move(rng: &mut ThreadRng) -> Command {
    Command::Single(MicroCommand::MoveToRect(
        rng.random_range(200..1000),
        rng.random_range(200..1000),
        10,
        10,
    ))
}

#[cfg(any(feature = "desktop", feature = "web"))]
pub async fn run_smooth_wander() -> anyhow::Result<()> {
    println!("Goooal");
    let user = UserBuilder::default().build();
    let mut rng = rand::rng();
    let mut state = SmoothState::new(&user, random_move(&mut rng), Instant::now());
    let mut engine = engine::default_engine()?;
    loop {
        loop {
            if state.is_finished() {
                state = SmoothState::new(&user, random_move(&mut rng), Instant::now());
            } else {
                break;
            }
        }
        if let Some(token) = state.poll_action() {
            engine.execute(&token)?;
            continue;
        }
        if let Some(time) = state.poll_timeout() {
            crate::monkey::sleep_until(time).await;
        }
        state.handle_timeout(Instant::now());
    }
}
