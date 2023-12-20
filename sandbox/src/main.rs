use application::Application;

mod application;
mod renderer;

fn main() {
    let mut app = Application::new();

    while app.is_running() {
        app.input();
        app.update();
        app.render();
    }
}
