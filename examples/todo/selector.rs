use super::*;

impl TodoExampleState{
    pub fn select_todo_count(&self) -> usize{
        self.todos.len()
    }
}
