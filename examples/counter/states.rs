use super::*;
use reduxr::*;

#[derive(Default, Clone)]
pub struct State {
    pub counter: usize,
}

impl Reduce<Action> for State {
    fn reduce(self, action: Action) -> Self {
        match action {
            Action::Increment => State {
                counter: self.counter + 1,
            },
            Action::Decrement => State {
                counter: self.counter - 1,
            },
        }
    }
}
