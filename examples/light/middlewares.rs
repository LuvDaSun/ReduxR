extern crate reduxr;

use super::*;
use reduxr::Dispatch;
use std::sync::Mutex;

pub fn create_switch_middleware() -> impl FnOnce(Dispatch<State, Action>) -> Dispatch<State, Action>
{
    let mutex = Mutex::new(());

    |next| {
        Box::new(move |store, action| {
            next(store, action);

            if let Action::Switch = action {
                let _lock = mutex.lock();
                let state = store.get_state();
                if state.select_power() {
                    store.dispatch(Action::TurnOff);
                } else {
                    store.dispatch(Action::TurnOn);
                }
            }
        })
    }
}
