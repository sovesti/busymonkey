use enigo::agent::Agent;
use tokio::time::Instant;

use crate::monkey::script::{
    user::{UserCfg, cursor::CursorState},
    util::{time_buf::TimeBuf, vector::Vector},
};

pub struct Snapshot {
    pub now: Instant,
    pub cursor: CursorState,
}

pub trait TakeSnap {
    fn take_snap(&mut self) -> anyhow::Result<Snapshot>;
}

impl<T: Agent> TakeSnap for T {
    fn take_snap(&mut self) -> anyhow::Result<Snapshot> {
        Ok(Snapshot {
            now: Instant::now(),
            cursor: CursorState {
                pos: Vector::from_coords(self.location()?),
                vel: Vector::zero(),
            },
        })
    }
}

pub struct Init {
    pub snap: Snapshot,
    pub cfg: UserCfg,
}

pub trait FromInit: Sized {
    fn from_init(init: &Init) -> Self;
}

impl<T> FromInit for TimeBuf<T> {
    fn from_init(init: &Init) -> Self {
        Self::new(init.cfg.reaction_time, init.snap.now)
    }
}
