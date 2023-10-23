use crate::actions::action::{ActionType};
use crate::store::state::AppState;
use crate::types::Todo;

pub trait Store {
    /// Store constructor
    fn new() -> Self;

    /// Actions handler
    fn handle_action(&mut self, action: ActionType) -> Result<(), String>;
}

pub struct AppStore {
    state: AppState,
}

impl Store for AppStore {
    fn new() -> Self {
        Self {
            state: AppState {
                todos: vec![]
            }
        }
    }

    fn handle_action(&mut self, action: ActionType) -> Result<(), String> {
        match action {
            ActionType::AddTodo(description) => self.state.todos.push(Todo {
                description,
                completed: false,
            }),
            ActionType::CompleteTodo(index) => {
                if let Some(value) = self.state.todos.get_mut(index) {
                    value.completed = true;
                } else {
                    return Err(format!("Todo by index {} not found", index));
                }
            }
        }

        Ok(())
    }
}

impl AppStore {
    pub fn get_todos(&self) -> Vec<Todo> {
        self.state.todos
            .iter()
            .filter(|todo| !todo.completed)
            .map(|todo| todo.clone())
            .collect()
    }

    pub fn get_completed_todos(&self) -> Vec<Todo> {
        self.state.todos
            .iter()
            .filter(|todo| todo.completed)
            .map(|todo| todo.clone())
            .collect()
    }
}
