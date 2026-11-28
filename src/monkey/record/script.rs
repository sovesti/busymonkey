use std::time::Duration;
use std::{iter, vec};

use serde::{Deserialize, Serialize};

use crate::monkey::model::Token;
use crate::monkey::model::parameters::{Parameter, ParameterSchema, ScriptFactory, ScriptMeta};
use crate::monkey::script::{Command, DynScript, MicroCommand};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct RecordedScript {
    tokens: Vec<Token>,
}

impl AsMut<Vec<Token>> for RecordedScript {
    fn as_mut(&mut self) -> &mut Vec<Token> {
        &mut self.tokens
    }
}

impl AsRef<Vec<Token>> for RecordedScript {
    fn as_ref(&self) -> &Vec<Token> {
        &self.tokens
    }
}

impl RecordedScript {
    pub fn to_script(self, delay: Duration) -> DynScript {
        Box::new(
            self.into_iter()
                .map(to_command)
                .chain(iter::once(Command::Single(MicroCommand::Idle(delay))))
                .cycle(),
        )
    }
}

fn to_command(token: Token) -> Command {
    todo!()
}

impl IntoIterator for RecordedScript {
    type Item = Token;
    type IntoIter = vec::IntoIter<Token>;

    fn into_iter(self) -> Self::IntoIter {
        self.tokens.into_iter()
    }
}
