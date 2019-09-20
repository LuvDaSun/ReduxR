use super::action::*;
use super::state::*;
use crate::*;
use std::rc::Rc;

#[test]
fn todo_example_test() {
    let store: Store<Rc<TodoExampleState>, TodoExampleAction, ()> = Store::default();
    store.dispatch(&TodoExampleAction::TodoAdd(TodoAddPayload {
        id: String::from("a"),
        name: String::from("do a thing"),
    }));
    store.dispatch(&TodoExampleAction::TodoResolve(TodoResolvePayload {
        id: String::from("a"),
    }));
    store.dispatch(&TodoExampleAction::TodoRemove(TodoRemovePayload {
        id: String::from("a"),
    }));
}
