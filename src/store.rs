use crate::*;
use std::sync::RwLock;

type Dispatcher<State, Action> = Box<dyn Send + Sync + Fn(&Store<State, Action>, Action)>;

/// A redux store. Dispatching actions on the store will make the action pass through the
/// middleware and finally the state will be reduced via the `Reduce` trait.
///
/// All middleware may return a value that is eventually returned from the dispatch function.
pub struct Store<State, Action> {
    state_lock: RwLock<State>,
    dispatcher: Dispatcher<State, Action>,
}

impl<State, Action> Store<State, Action>
where
    State: Clone + Reduce<Action>,
{
    /// Create a new Redux store
    pub fn new(state: State) -> Self {
        Store {
            state_lock: RwLock::new(state),
            dispatcher: Box::new(|store, action| {
                let mut state = store.state_lock.write().unwrap();
                *state = state.clone().reduce(action);
            }),
        }
    }

    pub fn add_middleware(
        self,
        middleware: impl FnOnce(Dispatcher<State, Action>) -> Dispatcher<State, Action>,
    ) -> Self {
        let state_lock = self.state_lock;
        let dispatcher = middleware(self.dispatcher);
        Store {
            state_lock,
            dispatcher,
        }
    }

    /// Dispatch action through the middleware and eventualle reduce state with it!
    pub fn dispatch(&self, action: Action) {
        (self.dispatcher)(self, action);
    }

    /// Get a clone of the current state
    pub fn get_state(&self) -> State {
        self.state_lock.read().unwrap().clone()
    }
}

impl<State, Action> Default for Store<State, Action>
where
    State: Default + Clone + Reduce<Action>,
{
    /// Create a new redux store, with a default state.
    fn default() -> Self {
        Store::new(State::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Copy, Clone)]
    enum LampAction {
        TurnOn,
        TurnOff,
        Switch,
    }

    #[derive(Default, Clone)]
    struct LampState {
        power: bool,
    }

    impl LampState {
        pub fn select_power(&self) -> bool {
            self.power
        }
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

    #[test]
    fn store_test() {
        let store: Store<LampState, _> = Store::default();

        let state = store.get_state();
        assert_eq!(state.power, false);

        store.dispatch(LampAction::TurnOn);
        let state = store.get_state();
        assert_eq!(state.power, true);

        store.dispatch(LampAction::TurnOff);
        let state = store.get_state();
        assert_eq!(state.power, false);
    }

    #[test]
    fn store_middleware_test() {
        let store: Store<LampState, _> = Store::default();
        let store = store.add_middleware(|next| {
            Box::new(move |store, action| {
                if let LampAction::Switch = action {
                    let state = store.get_state();
                    if state.select_power() {
                        store.dispatch(LampAction::TurnOff);
                    } else {
                        store.dispatch(LampAction::TurnOn);
                    }
                };
                next(store, action);
            })
        });

        let state = store.get_state();
        assert_eq!(state.power, false);

        store.dispatch(LampAction::TurnOn);
        let state = store.get_state();
        assert_eq!(state.power, true);

        store.dispatch(LampAction::TurnOff);
        let state = store.get_state();
        assert_eq!(state.power, false);

        store.dispatch(LampAction::Switch);
        let state = store.get_state();
        assert_eq!(state.power, true);

        store.dispatch(LampAction::Switch);
        let state = store.get_state();
        assert_eq!(state.power, false);
    }
}
