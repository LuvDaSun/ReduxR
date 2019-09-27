extern crate reduxr;

use super::*;
use reduxr::*;

pub fn create_store() -> Store<AsyncExampleState, ()> {
    Store::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_store() {
        let store = create_store();

        let state = store.get_state();
        assert_eq!(state.select_count(), 0);

        store.dispatch(&());
        store.dispatch(&());
        store.dispatch(&());

        let state = store.get_state();
        assert_eq!(state.select_count(), 3);
    }
}
