use glad_gl::gl;
use log::{debug, error, info};
use paper_color::Srgba;

use crate::config::{WindowConfig, WindowMode};

#[derive(Debug)]
#[cfg(feature = "internal")]
pub struct Window {
    pub glfw:     glfw::Glfw,
    pub p_window: glfw::PWindow,
    pub events:   glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
}

#[derive(Debug)]
#[cfg(not(feature = "internal"))]
pub struct Window {
    glfw:     glfw::Glfw,
    p_window: glfw::PWindow,
    events:   glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
}

impl Window {
    pub fn new(config: &WindowConfig) -> Option<Self> {
        debug!("Creating window with config: {config:#?}");
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

        let Some((mut p_window, events)) = glfw.with_primary_monitor(|glfw, m| {
            glfw.create_window(
                config.width,
                config.height,
                &config.title,
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
        p_window.set_key_polling(true);
        p_window.set_cursor_pos_polling(true);
        p_window.set_scroll_polling(true);
        p_window.set_mouse_button_polling(true);
        p_window.set_cursor_enter_polling(true);

        debug!("Loading OpenGL function pointers");

        gl::load(|symbol| p_window.get_proc_address(symbol));

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

        Some(Self { glfw, p_window, events })
    }

    pub fn set_clear_color(&self, color: Srgba) {
        unsafe {
            gl::ClearColor(color.r, color.g, color.b, color.a);
        }
    }

    pub fn clear(&self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }
}
