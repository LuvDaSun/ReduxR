use super::action::*;
use crate::*;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone)]
pub struct TodoItem {
    name: String,
}

#[derive(Default, Clone)]
pub struct TodoExampleState {
    pub todos: Rc<HashMap<String, Rc<TodoItem>>>,
}

impl Reduce<TodoExampleAction> for Rc<TodoExampleState> {
    fn reduce(&self, action: &TodoExampleAction) -> Self {
        let cloned = self.clone();
        Rc::new(TodoExampleState {
            todos: cloned.todos.reduce(action),
        })
    }
}

impl<S> Reduce<TodoExampleAction> for Rc<HashMap<String, Rc<TodoItem>, S>> {
    fn reduce(&self, action: &TodoExampleAction) -> Self {
        match action {
            TodoExampleAction::TodoAdd(item) => self.clone(),
            TodoExampleAction::TodoRemove(item) => self.clone(),
            TodoExampleAction::TodoResolve(item) => self.clone(),
        }
    }
}
