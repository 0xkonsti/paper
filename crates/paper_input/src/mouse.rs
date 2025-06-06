#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Button4,
    Button5,
    Button6,
    Button7,
    Button8,
}

impl From<glfw::MouseButton> for MouseButton {
    fn from(button: glfw::MouseButton) -> Self {
        match button {
            glfw::MouseButton::Button1 => Self::Left,
            glfw::MouseButton::Button2 => Self::Right,
            glfw::MouseButton::Button3 => Self::Middle,
            glfw::MouseButton::Button4 => Self::Button4,
            glfw::MouseButton::Button5 => Self::Button5,
            glfw::MouseButton::Button6 => Self::Button6,
            glfw::MouseButton::Button7 => Self::Button7,
            glfw::MouseButton::Button8 => Self::Button8,
        }
    }
}
