extern crate reduxr;

use super::*;
use reduxr::*;

pub fn create_store() -> Store<State, Action> {
    let store: Store<State, _> = Store::default();
    store.add_middleware(|next| {
        Box::new(move |store, action| {
            if let Action::Switch = action {
                let state = store.get_state();
                if state.select_power() {
                    store.dispatch(Action::TurnOff);
                } else {
                    store.dispatch(Action::TurnOn);
                }
            };
            next(store, action);
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_store() {
        let store = create_store();

        let state = store.get_state();
        assert_eq!(state.select_power(), false);

        store.dispatch(Action::TurnOn);
        let state = store.get_state();
        assert_eq!(state.select_power(), true);

        store.dispatch(Action::TurnOff);
        let state = store.get_state();
        assert_eq!(state.select_power(), false);

        store.dispatch(Action::Switch);
        let state = store.get_state();
        assert_eq!(state.select_power(), true);

        store.dispatch(Action::Switch);
        let state = store.get_state();
        assert_eq!(state.select_power(), false);
    }
}
