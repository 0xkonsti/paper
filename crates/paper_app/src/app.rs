use paper_input::Event;

use crate::commands::Commands;

pub trait PaperApp: Default {
    fn setup(&mut self, cmd: Commands);
    fn update(&mut self, cmd: Commands);

    fn fixed_update(&mut self, _cmd: Commands) {}

    fn cleanup(&mut self, _cmd: Commands) {}

    fn event_handler(&mut self, _cmd: Commands, _events: &Vec<Event>) {}
}

pub struct EmptyApp;

impl Default for EmptyApp {
    fn default() -> Self {
        Self
    }
}

impl PaperApp for EmptyApp {
    fn setup(&mut self, _cmd: Commands) {}

    fn update(&mut self, _cmd: Commands) {}
}
