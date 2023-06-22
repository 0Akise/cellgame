pub mod cell;
pub mod game;
use sfml::graphics::{Color, RenderTarget, RenderWindow};
use sfml::window::{ContextSettings, Event, Style};

fn main() {
    let window_width: u32 = 800;
    let window_height: u32 = 500;
    let mut settings: ContextSettings = ContextSettings::default();
    settings.antialiasing_level = 1;

    let mut window = RenderWindow::new(
        (window_width, window_height),
        "SFML Conway's Game of Life",
        Style::CLOSE,
        &settings,
    );
    window.set_framerate_limit(30);

    let mut game = game::Game::new(window_width, window_height);
    game.init();
    println!("cells: {}", game.cells.len());

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => window.close(),
                _ => {}
            }
        }

        window.clear(Color::BLACK);
        game.update();
        game.render(&mut window);
        window.display();
    }
}
