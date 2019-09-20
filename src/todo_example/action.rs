pub struct TodoAddPayload {
    pub id: String,
    pub name: String,
}

pub struct TodoRemovePayload {
    pub id: String,
}

pub struct TodoResolvePayload {
    pub id: String,
}

pub enum TodoExampleAction {
    TodoAdd(TodoAddPayload),
    TodoRemove(TodoRemovePayload),
    TodoResolve(TodoResolvePayload),
}
