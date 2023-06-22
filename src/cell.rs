use sfml::graphics::{self, RenderTarget, Shape, Transformable};
use sfml::system::Vector2f;

#[derive(Clone)]
pub struct Life34 {
    pub pos: (i32, i32),
    pub alive: bool,
}

impl Life34 {
    pub fn new(pos: (i32, i32), alive: bool) -> Self {
        Self { pos, alive }
    }

    pub fn render(&self, window: &mut graphics::RenderWindow) {
        let mut cell = graphics::RectangleShape::new();

        cell.set_size(Vector2f::new(1.0, 1.0));
        cell.set_fill_color(graphics::Color::GREEN);
        cell.set_position(Vector2f::new(self.pos.0 as f32, self.pos.1 as f32));

        window.draw(&cell);
    }

    pub fn update(&self, live_neighbors: usize) -> Self {
        let alive = match (self.alive, live_neighbors) {
            (true, 3) | (true, 4) => true,
            (false, 3) | (false, 4) => true,
            _ => false,
        };

        Life34::new(self.pos, alive)
    }
}

#[derive(Clone)]
pub struct HighLife {
    pub pos: (i32, i32),
    pub alive: bool,
}

impl HighLife {
    pub fn new(pos: (i32, i32), alive: bool) -> Self {
        Self { pos, alive }
    }

    pub fn render(&self, window: &mut graphics::RenderWindow) {
        let mut cell = graphics::RectangleShape::new();

        cell.set_size(Vector2f::new(1.0, 1.0));
        cell.set_fill_color(graphics::Color::WHITE);
        cell.set_position(Vector2f::new(self.pos.0 as f32, self.pos.1 as f32));

        window.draw(&cell);
    }

    pub fn update(&self, live_neighbors: usize) -> Self {
        let alive = match (self.alive, live_neighbors) {
            (true, 3) | (true, 4) => true,
            (false, 3) | (false, 4) => true,
            _ => false,
        };

        HighLife::new(self.pos, alive)
    }
}
