extern crate redux_rs;

use super::*;
use redux_rs::*;
use std::rc::Rc;

pub fn create_store() -> Store<Rc<TodoExampleState>, TodoExampleAction> {
    Store::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_store() {
        let store = create_store();
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
}
