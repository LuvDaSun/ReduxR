use std::time::Duration;

pub struct WaitPayload {
    pub time: Duration,
}

pub enum AsyncExampleAction {
    WaitPayload(WaitPayload),
}
