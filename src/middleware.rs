use super::reduce::*;
use super::store::*;

pub struct MiddlewareContext<'a, State, Action> {
    pub index: usize,
    pub action: &'a Action,
    pub store: &'a mut Store<State, Action>,
}

impl<'a, State, Action> MiddlewareContext<'a, State, Action>
where
    State: Clone + Reduce<Action>,
{
    pub fn get_state(&self) -> State {
        self.store.get_state()
    }

    pub fn dispatch(&mut self, action: &Action) {
        self.store.dispatch_index(action, 0);
    }

    pub fn dispatch_next(&mut self, action: &Action) {
        self.store.dispatch_index(action, self.index + 1);
    }
}
