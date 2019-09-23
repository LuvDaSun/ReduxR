extern crate reduxr;

use super::*;
use reduxr::*;
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

        let state = store.get_state();
        assert_eq!(state.select_todo_count(),0);

        store.dispatch(&TodoExampleAction::TodoAdd(TodoAddPayload {
            id: String::from("a"),
            name: String::from("do a thing"),
        }));

        let state = store.get_state();
        assert_eq!(state.select_todo_count(),1);

        store.dispatch(&TodoExampleAction::TodoResolve(TodoResolvePayload {
            id: String::from("a"),
        }));

        let state = store.get_state();
        assert_eq!(state.select_todo_count(),1);

        store.dispatch(&TodoExampleAction::TodoRemove(TodoRemovePayload {
            id: String::from("a"),
        }));

        let state = store.get_state();
        assert_eq!(state.select_todo_count(),0);
    }
}
