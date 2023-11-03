use std::sync::mpsc::Receiver;

use glfw;

pub struct Window {
    pub glfw_instance: glfw::Glfw,
    pub window_instance: Box<glfw::Window>,
    pub events: Receiver<(f64, glfw::WindowEvent)>,
}

pub struct WindowSettings<'a> {
    pub width: u32,
    pub height: u32,
    pub title: &'a str,
}

impl Window {
    /// Creates new empty window
    pub fn new(window_settings: WindowSettings) -> Window {
        use glfw::fail_on_errors;
        let mut glfw_instance = glfw::init(fail_on_errors!()).unwrap();

        // Create a windowed mode window and its OpenGL context
        let (window_instance, events) = glfw_instance
            .create_window(
                window_settings.width,
                window_settings.height,
                window_settings.title,
                glfw::WindowMode::Windowed,
            )
            .expect("Failed to create GLFW window.");

        return Window {
            window_instance,
            glfw_instance,
            events,
        };
    }
}
