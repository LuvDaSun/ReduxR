extern crate reduxr;

use super::*;
use reduxr::Store;

pub fn create_store() -> Store<State, Action> {
    Store::default().add_middleware(create_switch_middleware())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn test_store() {
        let store_arc = Arc::new(create_store());
        let store = store_arc.clone();

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

        for _ in 0..100 {
            let threads: Vec<_> = (0..10)
                .map(|_| store_arc.clone())
                .map(|store| std::thread::spawn(move || store.dispatch(Action::Switch)))
                .collect();

            threads
                .into_iter()
                .for_each(|thread| thread.join().unwrap());

            let store = store_arc.clone();
            let state = store.get_state();
            assert_eq!(state.select_power(), false);
        }
    }
}
