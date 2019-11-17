extern crate reduxr;

use super::*;
use std::collections::hash_map::RandomState;

use reduxr::*;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Default, Clone)]
pub struct TodoItem {
    pub name: String,
    pub done: bool,
}

#[derive(Default, Clone)]
pub struct TodoExampleState {
    pub todos: Rc<HashMap<String, Rc<TodoItem>>>,
}

impl Reduce<TodoExampleAction> for Rc<TodoExampleState> {
    fn reduce(mut self, action: TodoExampleAction) -> Self {
        let self_mut = Rc::make_mut(&mut self);

        self_mut.todos = self_mut.todos.clone().reduce(action);

        self
    }
}

impl Reduce<TodoExampleAction> for Rc<HashMap<String, Rc<TodoItem>, RandomState>> {
    fn reduce(mut self, action: TodoExampleAction) -> Self {
        match action {
            TodoExampleAction::TodoAdd(add_item) => {
                let self_mut = Rc::make_mut(&mut self);
                self_mut.insert(
                    add_item.id.clone(),
                    Rc::new(TodoItem {
                        name: add_item.name,
                        done: false,
                    }),
                );
            }

            TodoExampleAction::TodoRemove(remove_item) => {
                let self_mut = Rc::make_mut(&mut self);
                self_mut.remove(&remove_item.id);
            }

            TodoExampleAction::TodoResolve(resolve_item) => {
                let self_mut = Rc::make_mut(&mut self);
                let mut_item = Rc::make_mut(self_mut.get_mut(&resolve_item.id).unwrap());

                mut_item.done = true;
            }
        }

        self
    }
}
