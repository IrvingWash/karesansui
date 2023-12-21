use application::Application;

mod application;
mod canvas;

fn main() {
    let mut app = Application::new();

    app.init();

    while app.is_running() {
        app.input();
        app.update();
        app.render();
    }
}
