use glad_gl::gl;
use glfw::Context;
use log::{debug, error, info};
use paper_math::{UVec2, Vec2};

use crate::{Samples, WindowConfig, WindowMode};

#[derive(Debug)]
pub struct Window {
    pub glfw: glfw::Glfw,
    pub window: glfw::PWindow,
    pub events: glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,

    pub size: UVec2,
}

impl Window {
    pub fn new(config: &WindowConfig) -> Option<Self> {
        debug!("Creating window with config: {config:?}");
        debug!("Initializing GLFW");

        let result = glfw::init(glfw::fail_on_errors);
        if let Err(e) = result {
            error!("Failed to initialize GLFW: {e}");
            return None;
        }
        let mut glfw = result.unwrap();

        glfw.window_hint(glfw::WindowHint::ContextVersion(4, 6));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        glfw.window_hint(glfw::WindowHint::Resizable(config.resizable));
        glfw.window_hint(glfw::WindowHint::Samples(config.samples.to_glfw_samples()));

        debug!("Creating GLFW window");

        let Some((mut window, events)) = glfw.with_primary_monitor(|glfw, m| {
            glfw.create_window(
                config.width,
                config.height,
                config.title,
                m.map_or(glfw::WindowMode::Windowed, |m| match config.mode {
                    WindowMode::Windowed => glfw::WindowMode::Windowed,
                    WindowMode::Fullscreen => glfw::WindowMode::FullScreen(m),
                }),
            )
        }) else {
            error!("Failed to get primary monitor");
            return None;
        };

        // Polling
        window.set_key_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_scroll_polling(true);
        window.set_mouse_button_polling(true);
        window.set_cursor_enter_polling(true);

        debug!("Loading OpenGL function pointers");

        gl::load(|symbol| window.get_proc_address(symbol));

        unsafe {
            let data = [
                ("OpenGL version: ", gl::GetString(gl::VERSION)),
                ("Vendor: ", gl::GetString(gl::VENDOR)),
                ("Renderer: ", gl::GetString(gl::RENDERER)),
            ];
            data.iter().for_each(|(msg, data)| {
                info!("{:<20} {}", msg, std::ffi::CStr::from_ptr(*data as *const i8).to_str().unwrap());
            });
        };

        // Global GL configuration
        glfw.set_swap_interval(if config.vsync { glfw::SwapInterval::Sync(1) } else { glfw::SwapInterval::None });

        unsafe {
            // gl::Enable(gl::DEPTH_TEST);
            // gl::Enable(gl::CULL_FACE);
            // gl::CullFace(gl::BACK);
            gl::Enable(gl::MULTISAMPLE);
        }

        info!("Window created successfully");
        Some(Self { glfw, window, events, size: (config.width, config.height).into() })
    }

    pub fn poll_events(&mut self) {
        self.glfw.poll_events();
    }

    pub fn swap_buffers(&mut self) {
        self.window.swap_buffers();
    }

    pub fn should_close(&self) -> bool {
        self.window.should_close()
    }

    pub fn get_cursor_pos(&self) -> Vec2 {
        let (x, y) = self.window.get_cursor_pos();
        Vec2::new(x as f32, y as f32)
    }

    pub fn close(&mut self) {
        self.window.set_should_close(true);
    }

    pub fn clear(&self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn set_samples(&mut self, samples: Samples) {
        glfw::WindowHint::Samples(samples.to_glfw_samples());
    }
}
