pub mod game;
pub mod logger;
pub mod window;

use game::Game;
use glfw::Context;
use window::{Window, WindowSettings};

/// This is the entrypoint of whole program
fn main() {
    let mut window = Window::new(WindowSettings {
        width: 500,
        height: 500,
        title: "Wiejskie Żyćko",
    });

    // Make the window's context current
    window.window_instance.make_current();
    window.window_instance.set_key_polling(true);

    Game::run_game_loop(&mut window);
}
