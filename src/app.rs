use crate::Args;

pub struct AppState {
    pub url: String,
    pub should_quit: bool,
    pub text: String,
}

impl AppState {
    pub fn new(config: Args) -> Self {
        Self {
            url: config.url,
            text: "".to_string(),
            should_quit: false,
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set should_quit to true to quit the application.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn increment_counter(&self) {
        todo!()
    }

    pub fn decrement_counter(&self) {
        todo!()
    }
}
