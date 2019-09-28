#[derive(Clone)]
pub struct TodoAddPayload {
    pub id: String,
    pub name: String,
}

#[derive(Clone)]
pub struct TodoRemovePayload {
    pub id: String,
}

#[derive(Clone)]
pub struct TodoResolvePayload {
    pub id: String,
}

#[derive(Clone)]
pub enum TodoExampleAction {
    TodoAdd(TodoAddPayload),
    TodoRemove(TodoRemovePayload),
    TodoResolve(TodoResolvePayload),
}
