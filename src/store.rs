use crate::*;
use std::sync::RwLock;

pub type Dispatch<State, Action> = Box<dyn Send + Sync + Fn(&Store<State, Action>, Action)>;

/// A redux store. Dispatching actions on the store will make the action pass through the
/// middleware and finally the state will be reduced via the `Reduce` trait.
///
/// All middleware may return a value that is eventually returned from the dispatch function.
pub struct Store<State, Action> {
    state_lock: RwLock<State>,
    dispatch_handler: Dispatch<State, Action>,
}

impl<State, Action> Store<State, Action>
where
    State: Clone + Reduce<Action>,
{
    /// Create a new Redux store
    pub fn new(state: State) -> Self {
        Store {
            state_lock: RwLock::new(state),
            dispatch_handler: Box::new(|store, action| {
                let mut state = store.state_lock.write().unwrap();
                *state = state.clone().reduce(action);
            }),
        }
    }

    pub fn add_middleware(
        mut self,
        middleware: impl FnOnce(Dispatch<State, Action>) -> Dispatch<State, Action>,
    ) -> Self {
        self.dispatch_handler = middleware(self.dispatch_handler);
        self
    }

    /// Dispatch action through the middleware and eventually reduce state with it!
    pub fn dispatch(&self, action: Action) {
        (self.dispatch_handler)(self, action);
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
