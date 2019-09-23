use super::*;

pub struct TodoSelectItem {
    pub id: String,
    pub name: String,
    pub done: bool,
}

pub struct SelectTodoItemArg {
    id: String,
}

impl TodoExampleState {
    pub fn select_todo_count(&self) -> usize {
        self.todos.len()
    }

    // pub fn select_todo_list(&self) -> impl Iterator{
    //     self.todos.iter().map(|(id,item)|TodoSelectItem{
    //         id: id.clone(),
    //         name: item.name.clone(),
    //         done: item.done,
    //     }).collect()
    // }

    pub fn select_todo_item(&self, SelectTodoItemArg { id }: SelectTodoItemArg) -> TodoSelectItem {
        let item = self.todos.get(&id).unwrap();

        TodoSelectItem {
            id,
            name: item.name.clone(),
            done: item.done,
        }
    }
}
