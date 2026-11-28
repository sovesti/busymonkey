use crate::monkey::script::{command::Command, user::UserTokenProto};

pub trait CommandIntoProto: Fn(Command) -> Box<dyn UserTokenProto> {}

impl<T: Fn(Command) -> Box<dyn UserTokenProto>> CommandIntoProto for T {}

#[cfg(feature = "desktop")]
pub fn fallback() -> impl CommandIntoProto {
    use crate::monkey::script::state::micro_cmd_iter;
    |cmd| Box::new(micro_cmd_iter(cmd.into_micros()))
}
