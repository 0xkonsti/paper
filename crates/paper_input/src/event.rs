use log::warn;

use crate::{Action, Key, MouseButton};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Event {
    // Basic Events
    Key(Key, Action),
    MouseMove(u64, u64),
    MouseButton(MouseButton, Action),
    MouseScroll(u64, u64),
    MouseEnter(bool),
    Resize(u32, u32),
    Focus(bool),
    Close,
    Refresh,
    Maximize(bool),
    Iconify(bool),
    ContentScale(u32, u32),
    FileDrop(Vec<std::path::PathBuf>),
    WindowPos(i32, i32),
    WindowSize(i32, i32),

    // Grouped Events
    AnyKey(Vec<Key>, Action),
    AnyMouseButton(Vec<MouseButton>, Action),
}

impl From<glfw::WindowEvent> for Event {
    fn from(event: glfw::WindowEvent) -> Self {
        match event {
            glfw::WindowEvent::Key(key, _, action, _) => {
                let key = key.into();
                let action = action.into();
                Self::Key(key, action)
            }
            glfw::WindowEvent::MouseButton(button, action, _) => {
                let button = button.into();
                let action = action.into();
                Self::MouseButton(button, action)
            }
            glfw::WindowEvent::CursorPos(x, y) => Self::MouseMove(x as u64, y as u64),
            glfw::WindowEvent::Scroll(x, y) => Self::MouseScroll(x as u64, y as u64),
            glfw::WindowEvent::CursorEnter(entered) => Self::MouseEnter(entered),
            glfw::WindowEvent::FramebufferSize(width, height) => Self::Resize(width as u32, height as u32),
            glfw::WindowEvent::Focus(focused) => Self::Focus(focused),
            glfw::WindowEvent::Close => Self::Close,
            glfw::WindowEvent::Refresh => Self::Refresh,
            glfw::WindowEvent::Maximize(maximized) => Self::Maximize(maximized),
            glfw::WindowEvent::Iconify(iconified) => Self::Iconify(iconified),
            glfw::WindowEvent::ContentScale(x, y) => Self::ContentScale(x as u32, y as u32),
            glfw::WindowEvent::FileDrop(paths) => Self::FileDrop(paths),
            glfw::WindowEvent::Pos(x, y) => Self::WindowPos(x, y),
            glfw::WindowEvent::Size(width, height) => Self::WindowSize(width, height),
            _ => {
                warn!("Unhandled window event: {event:?}");
                Self::Close
            }
        }
    }
}
