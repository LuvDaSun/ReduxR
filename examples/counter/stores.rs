extern crate reduxr;

use super::*;
use reduxr::*;

pub fn create_store() -> Store<State, Action> {
    Store::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_store() {
        let store = create_store();

        let state = store.get_state();
        assert_eq!(state.select_counter(), 0);

        store.dispatch(Action::Increment);
        let state = store.get_state();
        assert_eq!(state.select_counter(), 1);

        store.dispatch(Action::Increment);
        let state = store.get_state();
        assert_eq!(state.select_counter(), 2);

        store.dispatch(Action::Decrement);
        let state = store.get_state();
        assert_eq!(state.select_counter(), 1);
    }
}
