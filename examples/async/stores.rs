extern crate futures;
extern crate reduxr;

use super::*;
use reduxr::*;
use std::future::*;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;

#[derive(Default)]
pub struct ReadyFuture;

impl Future for ReadyFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(())
    }
}

pub fn create_store() -> Store<AsyncExampleState, (), ReadyFuture> {
    Store::default().add_middleware(|context| context.dispatch_next(context.action))
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
