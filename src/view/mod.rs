use crate::actions::dispatcher::AppDispatcher;
use crate::store::store::AppStore;

mod todos;

trait View {
    /// Create page
    fn new(&mut self, provider: &AppDispatcher<AppStore>);

    /// Render page
    fn render(&self);
}