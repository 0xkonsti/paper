#[derive(Debug)]
pub enum WindowMode {
    Windowed,
    Fullscreen,
}

#[derive(Debug, Default)]
pub enum Samples {
    #[default]
    None,
    X2,
    X4,
    X8,
    X16,
}

impl Samples {
    pub fn to_glfw_samples(&self) -> Option<u32> {
        match self {
            Samples::None => None,
            Samples::X2 => Some(2),
            Samples::X4 => Some(4),
            Samples::X8 => Some(8),
            Samples::X16 => Some(16),
        }
    }

    pub fn raw_value(&self) -> f32 {
        match self {
            Samples::None => 0.0,
            Samples::X2 => 2.0,
            Samples::X4 => 4.0,
            Samples::X8 => 8.0,
            Samples::X16 => 16.0,
        }
    }
}

#[derive(Debug)]
pub struct WindowConfig<'a> {
    pub width: u32,
    pub height: u32,
    pub title: &'a str,
    pub resizable: bool,
    pub vsync: bool,
    pub samples: Samples,
    pub mode: WindowMode,
}

impl<'a> WindowConfig<'a> {
    pub fn with_width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }

    pub fn with_height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }

    pub fn with_size(mut self, size: impl Into<(u32, u32)>) -> Self {
        let (width, height) = size.into();
        self.width = width;
        self.height = height;
        self
    }

    pub fn with_title(mut self, title: &'a str) -> Self {
        self.title = title;
        self
    }

    pub fn with_resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }

    pub fn with_vsync(mut self, vsync: bool) -> Self {
        self.vsync = vsync;
        self
    }

    pub fn with_samples(mut self, samples: Samples) -> Self {
        self.samples = samples;
        self
    }

    pub fn with_mode(mut self, mode: WindowMode) -> Self {
        self.mode = mode;
        self
    }
}

impl Default for WindowConfig<'_> {
    fn default() -> Self {
        Self {
            width: 800,
            height: 600,
            title: "Paper Window",
            resizable: false,
            vsync: true,
            samples: Samples::X16,
            mode: WindowMode::Windowed,
        }
    }
}
