extern crate reduxr;

use super::*;
use reduxr::*;

#[derive(Default, Clone)]
pub struct LampState {
    pub power: bool,
}

impl Reduce<LampAction> for LampState {
    fn reduce(self, action: LampAction) -> Self {
        match action {
            LampAction::TurnOn => LampState { power: true },
            LampAction::TurnOff => LampState { power: false },
            _ => self,
        }
    }
}
