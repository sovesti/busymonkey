use enigo::agent::Agent;
use tokio::time::Instant;

use crate::monkey::engine;
use crate::monkey::script::{
    TokenProto,
    command::DynScript,
    user::{Init, TakeSnap, UserBuilder, UserCommandDispatcher, fallback},
};

pub async fn start_script(script: anyhow::Result<DynScript>) -> anyhow::Result<()> {
    let mut engine = engine::default_engine()?;
    let user = UserBuilder::default().build();
    let init = Init {
        cfg: user,
        snap: engine.take_snap()?,
    };
    let mut proto = UserCommandDispatcher::new(&init, fallback(), script?);
    loop {
        if proto.is_finished() {
            break;
        }
        if let Some(token) = proto.poll_action() {
            engine.execute(&token)?;
        }
        if let Some(time) = proto.poll_timeout() {
            crate::monkey::sleep_until(time).await;
        }
        proto.handle_timeout(Instant::now());
    }
    Ok(())
}
