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

        let state1 = store.get_state();

        store.dispatch(TodoExampleAction::TodoAdd(TodoAddPayload {
            id: String::from("a"),
            name: String::from("do a thing"),
        }));

        let state2 = store.get_state();

        store.dispatch(TodoExampleAction::TodoResolve(TodoResolvePayload {
            id: String::from("a"),
        }));

        let state3 = store.get_state();

        std::thread::spawn(|| {
            store.dispatch(TodoExampleAction::TodoRemove(TodoRemovePayload {
                id: String::from("a"),
            }))
        });

        let state4 = std::thread::spawn(|| store.get_state()).join().unwrap();

        assert_eq!(state1.select_todo_count(), 0);

        assert_eq!(state2.select_todo_count(), 1);
        assert_eq!(
            state2.select_todo_item(SelectTodoItemArg {
                id: String::from("a")
            }),
            TodoSelectItem {
                id: String::from("a"),
                name: String::from("do a thing"),
                done: false,
            }
        );

        assert_eq!(state3.select_todo_count(), 1);
        assert_eq!(
            state3.select_todo_item(SelectTodoItemArg {
                id: String::from("a")
            }),
            TodoSelectItem {
                id: String::from("a"),
                name: String::from("do a thing"),
                done: true,
            }
        );

        assert_eq!(state4.select_todo_count(), 0);
    }
}
