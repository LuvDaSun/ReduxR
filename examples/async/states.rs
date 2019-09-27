extern crate reduxr;

use reduxr::*;

#[derive(Default, Clone)]
pub struct AsyncExampleState {
    pub count: usize,
}

impl Reduce<()> for AsyncExampleState {
    fn reduce(self, _action: &()) -> Self {
        Self {
            count: self.count + 1,
        }
    }
}
