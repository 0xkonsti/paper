use log::info;

use crate::commands::Commands;

pub trait PaperApp: Default {
    fn setup(&mut self, cmd: Commands);
    fn update(&mut self, cmd: Commands);
    fn cleanup(&mut self, _cmd: Commands) {
        info!("Closing Paper App! No default cleanup defined.");
    }
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
