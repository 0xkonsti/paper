use paper_input::Event;

pub trait Commandable {
    fn close(&mut self);

    fn events(&self) -> &Vec<Event>;
}

pub struct Commands<'a> {
    ca: &'a mut dyn Commandable,
}

impl<'a> Commands<'a> {
    pub(crate) fn new(ca: &'a mut dyn Commandable) -> Self {
        Self { ca }
    }

    pub fn close(&mut self) {
        self.ca.close();
    }

    pub fn events(&self) -> &Vec<Event> {
        self.ca.events()
    }
}
