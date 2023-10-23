use crate::actions::action::ActionType;
use crate::actions::dispatcher::{AppDispatcher, Dispatcher};
use crate::store::store::{AppStore, Store};

mod store;
mod types;
mod actions;

fn main() {
    let mut store = AppStore::new();
    let mut provider = AppDispatcher::new(&mut store);

    // Типо ui компонент
    provider.register_listener(|action| {
        match action {
            ActionType::AddTodo(description) => {
                println!("Success add todo: {}", description);
            },
            ActionType::CompleteTodo(todo) => {
                println!("Success complete todo: {}", todo);
            }
        }
    });

    _ = provider.dispatch(ActionType::AddTodo("Купить пиццу".to_string()));
    _ = provider.dispatch(ActionType::AddTodo("Начать работать".to_string()));
    _ = provider.dispatch(ActionType::AddTodo("Приготовить ужин".to_string()));

    let result = provider.dispatch(ActionType::CompleteTodo(5));

    match result {
        Ok(_) => {},
        Err(err) => println!("{}", err),
    }

    _ = provider.dispatch(ActionType::CompleteTodo(0));

    for todo in &store.get_todos() {
        println!("{}", todo.description);
    }

    println!("Completed");

    for todo in &store.get_completed_todos() {
        println!("{}", todo.description);
    }
}
