use crate::*;
use std::cell::RefCell;

/// A redux store. Dispatching actions on the store will make the action pass through the
/// middleware and finally the state will be reduced via the `Reduce` trait.
///
/// All middleware may return a value that is eventually returned from the dispatch function.
pub struct Store<State, Action, DispatchResult = ()> {
    state: RefCell<State>,
    initial_result_factory: fn() -> DispatchResult,
    middleware: Vec<Middleware<State, Action, DispatchResult>>,
}

impl<State, Action, DispatchResult> Store<State, Action, DispatchResult>
where
    State: Clone + Reduce<Action>,
{
    /// Create a new Redux store
    pub fn new(state: State, initial_result_factory: fn() -> DispatchResult) -> Self {
        let middleware = Vec::default();
        Self {
            state: RefCell::new(state),
            initial_result_factory,
            middleware,
        }
    }

    /// Adds new middleware to the store
    pub fn add_middleware<Middleware>(mut self, middleware: Middleware) -> Self
    where
        Middleware:
            'static + Fn(MiddlewareContext<State, Action, DispatchResult>) -> DispatchResult,
    {
        self.middleware.push(Box::new(middleware));
        self
    }

    /// Dispatch action through the middleware and eventualle reduce state with it!
    pub fn dispatch(&self, action: &Action) -> DispatchResult {
        self.dispatch_index(action, 0)
    }

    pub(crate) fn dispatch_index(&self, action: &Action, index: usize) -> DispatchResult {
        let middleware = self.middleware.get(index);

        match middleware {
            Option::None => {
                let state = self.get_state();
                let state = state.reduce(action);
                self.state.replace(state);
                (self.initial_result_factory)()
            }
            Option::Some(middleware) => {
                let context = MiddlewareContext::new(self, action, index);
                middleware(context)
            }
        }
    }

    /// Get a clone of the current state
    pub fn get_state(&self) -> State {
        self.state.borrow().clone()
    }
}

impl<State, Action, DispatchResult> Store<State, Action, DispatchResult>
where
    State: Default + Clone + Reduce<Action>,
{
    pub fn new_with_result(initial_result_factory: fn() -> DispatchResult) -> Self {
        Store::new(State::default(), initial_result_factory)
    }
}

impl<State, Action, DispatchResult> Default for Store<State, Action, DispatchResult>
where
    State: Default + Clone + Reduce<Action>,
    DispatchResult: Default,
{
    /// Create a new redux store, with a default state and a default result factory.
    fn default() -> Self {
        Store::new(State::default(), DispatchResult::default)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    enum LampAction {
        TurnOn,
        TurnOff,
        Switch,
    }

    #[derive(Default, Clone)]
    struct LampState {
        power: bool,
    }

    impl Reduce<LampAction> for LampState {
        fn reduce(self, action: &LampAction) -> Self {
            match action {
                LampAction::TurnOn => LampState { power: true },
                LampAction::TurnOff => LampState { power: false },
                _ => self,
            }
        }
    }

    #[test]
    fn store_test() {
        let store: Store<LampState, LampAction> = Store::default();

        let state = store.get_state();
        assert_eq!(state.power, false);

        store.dispatch(&LampAction::TurnOn);
        let state = store.get_state();
        assert_eq!(state.power, true);

        store.dispatch(&LampAction::TurnOff);
        let state = store.get_state();
        assert_eq!(state.power, false);
    }

    #[test]
    fn store_middleware_test() {
        let store: Store<LampState, LampAction> =
            Store::default().add_middleware(|context: MiddlewareContext<LampState, LampAction>| {
                context.dispatch_next(context.action);

                if let LampAction::Switch = context.action {
                    let state = context.get_state();
                    if state.power {
                        context.dispatch(&LampAction::TurnOff);
                    } else {
                        context.dispatch(&LampAction::TurnOn);
                    }
                }
            });

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
    fn store_result_test() {
        let store: Store<LampState, LampAction, usize> = Store::default().add_middleware(
            |context: MiddlewareContext<LampState, LampAction, usize>| {
                let count = context.dispatch_next(context.action);

                count + 1
            },
        );

        let state = store.get_state();
        assert_eq!(state.power, false);

        store.dispatch(&LampAction::TurnOn);
        let state = store.get_state();
        assert_eq!(state.power, true);

        store.dispatch(&LampAction::TurnOff);
        let state = store.get_state();
        assert_eq!(state.power, false);
    }
}
