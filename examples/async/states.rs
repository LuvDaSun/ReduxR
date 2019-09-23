extern crate reduxr;

use super::*;

use reduxr::*;

#[derive(Default, Clone)]
pub struct AsyncExampleState {
    pub count: usize,
}

impl Reduce<AsyncExampleAction> for AsyncExampleState {
    fn reduce(self, _action: &AsyncExampleAction) -> Self {
        Self {
            count: self.count + 1,
        }
    }
}
