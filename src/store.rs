use super::reduce::*;
use std::marker::PhantomData;

pub struct Store<State, Action> {
    state: State,
    action: PhantomData<Action>,
}

impl<State, Action> Store<State, Action>
where
    State: Clone + Reduce<Action>,
{
    pub fn new(state: State) -> Self {
        let action = PhantomData;
        Self { state, action }
    }

    pub fn dispatch(&mut self, action: &Action) {
        self.state = self.state.reduce(action);
    }

    pub fn get_state(&self) -> State {
        self.state.clone()
    }
}

impl<State, Action> Default for Store<State, Action>
where
    State: Default + Clone + Reduce<Action>,
{
    fn default() -> Self {
        Store::new(State::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    enum LampAction {
        Switch,
    }

    #[derive(Default, Clone)]
    struct LampState {
        power: bool,
    }

    impl Reduce<LampAction> for LampState {
        fn reduce(&self, action: &LampAction) -> Self {
            match action {
                LampAction::Switch => LampState { power: !self.power },
            }
        }
    }

    #[test]
    fn store_test() {
        let mut store: Store<LampState, LampAction> = Store::default();

        let state = store.get_state();
        assert_eq!(state.power, false);

        store.dispatch(&LampAction::Switch);
        let state = store.get_state();
        assert_eq!(state.power, true);

        store.dispatch(&LampAction::Switch);
        let state = store.get_state();
        assert_eq!(state.power, false);
    }
}
