use crate::*;

pub type Middleware<State, Action, DispatchResult> =
    Box<dyn Fn(MiddlewareContext<State, Action, DispatchResult>) -> DispatchResult>;

pub struct MiddlewareContext<'a, State, Action, DispatchResult> {
    pub index: usize,
    pub action: &'a Action,
    pub store: &'a Store<State, Action, DispatchResult>,
}

impl<'a, State, Action, DispatchResult> MiddlewareContext<'a, State, Action, DispatchResult>
where
    State: Clone + Reduce<Action>,
{
    pub fn new(
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

    pub fn get_state(&self) -> State {
        self.store.get_state()
    }

    pub fn dispatch(&self, action: &Action) -> DispatchResult {
        self.store.dispatch_index(action, 0)
    }

    pub fn dispatch_next(&self, action: &Action) -> DispatchResult {
        self.store.dispatch_index(action, self.index + 1)
    }
}
