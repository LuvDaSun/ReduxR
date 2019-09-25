use crate::*;

/// Middleware is a function that takes a `MiddlewareContext` struct as an argument.
pub type Middleware<State, Action, DispatchResult = ()> =
    Box<dyn Fn(MiddlewareContext<State, Action, DispatchResult>) -> DispatchResult>;

/// The context in which middleware is executed.
pub struct MiddlewareContext<'a, State, Action, DispatchResult = ()> {
    store: &'a Store<State, Action, DispatchResult>,
    index: usize,

    /// The action being dispatched
    pub action: &'a Action,
}

impl<'a, State, Action, DispatchResult> MiddlewareContext<'a, State, Action, DispatchResult>
where
    State: Clone + Reduce<Action>,
{
    pub(crate) fn new(
        store: &'a Store<State, Action, DispatchResult>,
        action: &'a Action,
        index: usize,
    ) -> Self {
        Self {
            store,
            action,
            index,
        }
    }

    /// Get a clone of the current state in the store
    pub fn get_state(&self) -> State {
        self.store.get_state()
    }

    /// Dispatch an action to the store
    pub fn dispatch(&self, action: &Action) -> DispatchResult {
        self.store.dispatch_index(action, 0)
    }

    /// Dispatch action to the next middleware
    pub fn dispatch_next(&self, action: &Action) -> DispatchResult {
        self.store.dispatch_index(action, self.index + 1)
    }
}
