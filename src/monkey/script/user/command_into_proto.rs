use crate::monkey::script::{command::Command, state::micro_cmd_iter, user::UserTokenProto};

pub trait CommandIntoProto: Fn(Command) -> Box<dyn UserTokenProto> {}

impl<T: Fn(Command) -> Box<dyn UserTokenProto>> CommandIntoProto for T {}

pub fn fallback() -> impl CommandIntoProto {
    |cmd| Box::new(micro_cmd_iter(cmd.into_micros()))
}
