pub mod dispatcher;

#[derive(Clone)]
pub enum ActionType {
    AddTodo(String),
    CompleteTodo(usize)
}
