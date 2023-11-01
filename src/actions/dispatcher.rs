use crate::store::store::AppStore;
use crate::store::Store;

use super::ActionType;

pub trait Dispatcher<'a, T> where T: Store {
    /// Dispatcher constructor
    fn new(store: &'a mut T) -> Self;

    /// Register listener
    fn register_listener(&mut self, listener: fn(action: ActionType));

    /// Store handle actions
    fn dispatch(&mut self, action: ActionType) -> Result<(), String>;
}

pub struct AppDispatcher<'a, T> where T: Store {
    listeners: Vec<fn(action: ActionType)>,
    store: &'a mut T,
}

impl<'a> Dispatcher<'a, AppStore> for AppDispatcher<'a, AppStore> {
    fn new(store: &'a mut AppStore) -> Self {
        Self {
            listeners: vec![],
            store,
        }
    }

    fn register_listener(&mut self, listener: fn(ActionType)) {
        self.listeners.push(listener);
    }

    fn dispatch(&mut self, action: ActionType) -> Result<(), String> {
        _ = &self.store.handle_action(action.clone())?;
        Ok(())
    }
}
