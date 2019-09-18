use super::middleware::*;
use super::reduce::*;
use std::marker::PhantomData;

pub struct Store<State, Action> {
    state: State,
    middleware: Vec<Middleware<Action>>,
    action: PhantomData<Action>,
}

impl<State, Action> Store<State, Action>
where
    State: Clone + Reduce<Action>,
{
    pub fn new(state: State) -> Self {
        let action = PhantomData;
        let middleware = Vec::default();
        Self {
            state,
            middleware,
            action,
        }
    }

    pub fn add_middleware(mut self, middleware: Middleware<Action>) -> Self {
        self.middleware.push(middleware);
        self
    }

    pub fn dispatch(&mut self, action: &Action) {
        self.dispatch_index(action, 0);
    }

    fn dispatch_index(&mut self, action: &Action, index: usize) {
        let middleware = self.middleware.get(index);

        match middleware {
            Option::None => self.state = self.state.reduce(action),
            Option::Some(middleware) => {
                middleware(action);
                self.dispatch_index(action, index + 1);
            }
        };
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

    #[test]
    fn store_middleware_test() {
        let mut store: Store<LampState, LampAction> = Store::default()
            .add_middleware(|_| {})
            .add_middleware(|_| {});

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
