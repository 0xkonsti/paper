use paper_input::Event;

use crate::commands::Commands;

pub trait PaperApp {
    fn new(cmd: Commands) -> Self
    where
        Self: Sized;

    fn setup(&mut self, cmd: Commands) {}

    fn update(&mut self, cmd: Commands);

    fn fixed_update(&mut self, _cmd: Commands) {}

    fn cleanup(&mut self, _cmd: Commands) {}

    fn event_handler(&mut self, _cmd: Commands, _events: &[Event]) {}
}

pub struct EmptyApp;

impl PaperApp for EmptyApp {
    fn new(_cmd: Commands) -> Self {
        Self {}
    }

    fn update(&mut self, _cmd: Commands) {}
}
