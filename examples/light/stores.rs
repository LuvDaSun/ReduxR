extern crate reduxr;

use super::*;
use reduxr::*;

pub fn create_store() -> Store<LampState, LampAction> {
    Store::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_store() {
        let store: Store<LampState, _> = Store::default();
        let store = store.add_middleware(|next| {
            Box::new(move |store, action| {
                if let LampAction::Switch = action {
                    let state = store.get_state();
                    if state.select_power() {
                        store.dispatch(LampAction::TurnOff);
                    } else {
                        store.dispatch(LampAction::TurnOn);
                    }
                };
                next(store, action);
            })
        });

        let state = store.get_state();
        assert_eq!(state.select_power(), false);

        store.dispatch(LampAction::TurnOn);
        let state = store.get_state();
        assert_eq!(state.select_power(), true);

        store.dispatch(LampAction::TurnOff);
        let state = store.get_state();
        assert_eq!(state.select_power(), false);

        store.dispatch(LampAction::Switch);
        let state = store.get_state();
        assert_eq!(state.select_power(), true);

        store.dispatch(LampAction::Switch);
        let state = store.get_state();
        assert_eq!(state.select_power(), false);
    }
}
