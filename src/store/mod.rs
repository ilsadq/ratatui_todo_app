use crate::actions::ActionType;

pub mod store;

pub trait Store {
    /// Store constructor
    fn new() -> Self;

    /// Actions handler
    fn handle_action(&mut self, action: ActionType) -> Result<(), String>;
}
