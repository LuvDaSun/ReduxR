extern crate redux_rs;

use super::action::*;

use redux_rs::*;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Default)]
pub struct TodoItem {
    pub name: String,
    pub done: bool,
}

#[derive(Default)]
pub struct TodoExampleState {
    pub todos: HashMap<String, Rc<TodoItem>>,
}

impl Reduce<TodoExampleAction> for Rc<TodoExampleState> {
    fn reduce(self, action: &TodoExampleAction) -> Self {
        Rc::new(TodoExampleState {
            todos: self.todos.clone().reduce(action),
        })
    }
}

#[allow(clippy::implicit_hasher)]
impl Reduce<TodoExampleAction> for HashMap<String, Rc<TodoItem>> {
    fn reduce(self, action: &TodoExampleAction) -> Self {
        match action {
            TodoExampleAction::TodoAdd(add_item) => {
                let mut result: Self = self.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
                result.insert(
                    add_item.id.clone(),
                    Rc::new(TodoItem {
                        name: add_item.name.clone(),
                        done: false,
                    }),
                );
                result
            }

            TodoExampleAction::TodoRemove(remove_item) => {
                let mut result: Self = self.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
                result.remove(&remove_item.id);
                result
            }

            TodoExampleAction::TodoResolve(resolve_item) => {
                let mut result: Self = self.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
                let old_item = self.get(&resolve_item.id).unwrap().clone();

                result.insert(
                    resolve_item.id.clone(),
                    Rc::new(TodoItem {
                        name: old_item.name.clone(),
                        done: true,
                    }),
                );

                result
            }
        }
    }
}
