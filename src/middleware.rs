use super::reduce::*;
use super::store::*;

pub type Middleware<State, Action> = Box<dyn Fn(MiddlewareContext<State, Action>) -> ()>;

pub struct MiddlewareContext<'a, State, Action> {
    pub index: usize,
    pub action: &'a Action,
    pub store: &'a Store<State, Action>,
}

impl<'a, State, Action> MiddlewareContext<'a, State, Action>
where
    State: Clone + Reduce<Action>,
{
    pub fn new(store: &'a Store<State, Action>, action: &'a Action, index: usize) -> Self {
        Self {
            store,
            action,
            index,
        }
    }

    pub fn get_state(&self) -> State {
        self.store.get_state()
    }

    pub fn dispatch(&self, action: &Action) {
        self.store.dispatch_index(action, 0);
    }

    pub fn dispatch_next(&self, action: &Action) {
        self.store.dispatch_index(action, self.index + 1);
    }
}
