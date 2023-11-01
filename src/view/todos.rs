use crate::actions::dispatcher::AppDispatcher;
use crate::store::store::AppStore;
use crate::view::View;

struct TodosPage {

}

impl View for TodosPage {
    fn new(&mut self, provider: &AppDispatcher<AppStore>) {
        todo!()
    }

    fn render(&self) {
        todo!()
    }
}