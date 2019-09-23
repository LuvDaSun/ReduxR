extern crate reduxr;

use super::*;
use reduxr::*;

pub fn create_store() -> Store<AsyncExampleState, AsyncExampleAction> {
    Store::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_store() {
        let store = create_store();
    }
}
