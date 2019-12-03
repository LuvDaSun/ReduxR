extern crate reduxr;

use super::*;
use reduxr::*;

#[derive(Default, Clone)]
pub struct State {
    pub power: bool,
}

impl Reduce<Action> for State {
    fn reduce(self, action: Action) -> Self {
        match action {
            Action::TurnOn => State { power: true },
            Action::TurnOff => State { power: false },
            _ => self,
        }
    }
}
