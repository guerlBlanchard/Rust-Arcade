mod pong;

use coffee::graphics::{Color, Frame, Window, WindowSettings};
use coffee::load::Task;
use coffee::{Game, Timer};

use pong::*;

fn main() {
    Pong_player();
    // Arcade::run(WindowSettings{
    //     title: String::from("Arcade"),
    //     size: (1280, 1024),
    //     resizable: false,
    //     maximized: false,
    //     fullscreen: false
    // })
    // .expect("an error has occured while loading the window");
}

struct Arcade {
     
}

impl Game for Arcade {
    type Input = (); // No input data
    type LoadingScreen = (); // No loading screen

    fn load(_window: &Window) -> Task<Arcade> {
        // Load your game assets here. Check out the `load` module!
        Task::succeed(|| Arcade { /* ... */ })
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        // Clear the current frame
        frame.clear(Color::BLACK);

        // Draw your game here. Check out the `graphics` module!
    }
}