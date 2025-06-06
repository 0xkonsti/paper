#[derive(Debug)]
pub enum WindowMode {
    Windowed,
    Fullscreen,
}

#[derive(Debug)]
pub struct WindowConfig<'a> {
    pub width: u32,
    pub height: u32,
    pub title: &'a str,
    pub resizable: bool,
    pub vsync: bool,
    pub samples: u32,
    pub mode: WindowMode,
}

impl Default for WindowConfig<'_> {
    fn default() -> Self {
        Self {
            width: 800,
            height: 600,
            title: "Paper Window",
            resizable: false,
            vsync: true,
            samples: 0,
            mode: WindowMode::Windowed,
        }
    }
}
