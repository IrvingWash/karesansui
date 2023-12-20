pub struct Application {
    is_running: bool,
}

impl Application {
    pub fn new() -> Self {
        Application {
            is_running: false,
        }
    }

    pub fn is_running(&self) -> bool {
        self.is_running
    }

    pub fn input(&mut self) {}

    pub fn update(&mut self) {}

    pub fn render(&mut self) {}
}
