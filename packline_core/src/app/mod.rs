pub struct App;

pub mod channel;

impl App {
    pub fn new() -> App {
        App {}
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
