extern crate reduxr;

use super::*;
use std::collections::hash_map::RandomState;

use reduxr::*;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Default, Clone)]
pub struct TodoItem {
    pub name: String,
    pub done: bool,
}

#[derive(Default, Clone)]
pub struct TodoExampleState {
    pub todos: Arc<HashMap<String, Arc<TodoItem>>>,
}

impl Reduce<TodoExampleAction> for Arc<TodoExampleState> {
    fn reduce(mut self, action: TodoExampleAction) -> Self {
        let self_mut = Arc::make_mut(&mut self);

        self_mut.todos = self_mut.todos.clone().reduce(action);

        self
    }
}

impl Reduce<TodoExampleAction> for Arc<HashMap<String, Arc<TodoItem>, RandomState>> {
    fn reduce(mut self, action: TodoExampleAction) -> Self {
        match action {
            TodoExampleAction::TodoAdd(add_item) => {
                let self_mut = Arc::make_mut(&mut self);
                self_mut.insert(
                    add_item.id.clone(),
                    Arc::new(TodoItem {
                        name: add_item.name,
                        done: false,
                    }),
                );
            }

            TodoExampleAction::TodoRemove(remove_item) => {
                let self_mut = Arc::make_mut(&mut self);
                self_mut.remove(&remove_item.id);
            }

            TodoExampleAction::TodoResolve(resolve_item) => {
                let self_mut = Arc::make_mut(&mut self);
                let mut_item = Arc::make_mut(self_mut.get_mut(&resolve_item.id).unwrap());

                mut_item.done = true;
            }
        }

        self
    }
}
