extern crate futures;
extern crate reduxr;

use super::*;
use reduxr::*;
use std::future::*;

pub type AsyncExampleDisatchResult = Box<dyn Future<Output = ()> + Unpin>;

pub fn create_store() -> Store<AsyncExampleState, (), AsyncExampleDisatchResult> {
    Store::new_with_result(|| Box::new(futures::future::ready({})))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_store() {
        futures::executor::block_on(async {
            let store = create_store();

            let state = store.get_state();
            assert_eq!(state.select_count(), 0);

            store.dispatch(&()).await;
            store.dispatch(&()).await;
            store.dispatch(&()).await;

            let state = store.get_state();
            assert_eq!(state.select_count(), 3);
        })
    }
}
