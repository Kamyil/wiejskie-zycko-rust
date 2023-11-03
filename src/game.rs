use crate::{logger::Logger, window::Window};
use glfw::{Action, Context, Key};

pub struct Game {}

impl Game {
    /// Initializes the game by running the famous "game loop"
    /// that renders things on screen in frames per second
    pub fn run_game_loop(window: &mut Window) {
        // Loop until the user closes the window
        while !window.window_instance.should_close() {
            // Swap front and back buffers
            window.window_instance.swap_buffers();

            // Poll for and process events
            window.glfw_instance.poll_events();

            Game::init_gl(window);

            for (_, event) in glfw::flush_messages(&window.events) {
                match event {
                    glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                        window.window_instance.set_should_close(true);
                        Logger::info("Window closed");
                    }
                    _ => {}
                }
            }
        }
    }

    /// Intializes Graphics Library (a.k.a gl)
    fn init_gl(window: &mut Window) {
        gl::load_with(|symbol| window.window_instance.get_proc_address(symbol));
    }

    fn draw_some_heck_on_the_screen() {}
}
