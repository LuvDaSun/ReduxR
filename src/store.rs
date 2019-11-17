use crate::*;
use std::cell::RefCell;

type Dispatcher<State, Action> = Box<dyn Fn(&Store<State, Action>, Action)>;

/// A redux store. Dispatching actions on the store will make the action pass through the
/// middleware and finally the state will be reduced via the `Reduce` trait.
///
/// All middleware may return a value that is eventually returned from the dispatch function.
pub struct Store<State, Action> {
    state_cell: RefCell<State>,
    dispatcher: Dispatcher<State, Action>,
}

impl<State, Action> Store<State, Action>
where
    State: Clone + Reduce<Action>,
    Action: Clone,
{
    /// Create a new Redux store
    pub fn new(state: State) -> Self {
        Store {
            state_cell: RefCell::new(state),
            dispatcher: Box::new(|store, action| {
                let state = store.get_state();
                let state = state.reduce(action);
                store.state_cell.replace(state);
            }),
        }
    }

    pub fn enhance(
        self,
        enhancer: impl FnOnce(Dispatcher<State, Action>) -> Dispatcher<State, Action>,
    ) -> Self {
        let state_cell = self.state_cell;
        let dispatcher = enhancer(self.dispatcher);
        Store {
            state_cell,
            dispatcher,
        }
    }

    /// Dispatch action through the middleware and eventualle reduce state with it!
    pub fn dispatch(&self, action: Action) {
        (self.dispatcher)(self, action);
    }

    /// Get a clone of the current state
    pub fn get_state(&self) -> State {
        self.state_cell.borrow().clone()
    }
}

impl<State, Action> Default for Store<State, Action>
where
    State: Default + Clone + Reduce<Action>,
    Action: Clone,
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
    fn store_enhanced_test() {
        let store: Store<LampState, _> =
            Store::default().enhance(|next: Dispatcher<LampState, _>| {
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
