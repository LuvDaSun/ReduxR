extern crate futures;
extern crate reduxr;

use super::*;
use reduxr::*;
use std::future::*;
use std::pin::Pin;

pub type AsyncExampleDispatchResult = Pin<Box<dyn Future<Output = ()> + 'static>>;

pub fn create_store() -> Store<AsyncExampleState, (), AsyncExampleDispatchResult> {
    let store: Store<AsyncExampleState, (), AsyncExampleDispatchResult> =
        Store::new_with_result(|| Box::pin(futures::future::ready(())));

    let store = store.add_middleware(|context| context.dispatch_next(context.action));

    let store = store.add_middleware(|context| {
        let future = async {
            // context.dispatch_next(context.action).await;
        };
        let result: AsyncExampleDispatchResult = Box::pin(future);
        result
    });

    store
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
