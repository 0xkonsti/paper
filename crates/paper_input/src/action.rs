#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Action {
    Release,
    Press,
    Repeat,
}

impl From<glfw::Action> for Action {
    fn from(action: glfw::Action) -> Self {
        match action {
            glfw::Action::Release => Self::Release,
            glfw::Action::Press => Self::Press,
            glfw::Action::Repeat => Self::Repeat,
        }
    }
}
