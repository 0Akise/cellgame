use super::cell::{HighLife, Life34};
use rand::Rng;
use rayon::prelude::*;
use sfml::graphics::RenderWindow;
use std::collections::{HashMap, HashSet};

pub struct Game {
    pub cells: HashMap<(i32, i32), Life34>,
    pub width: u32,
    pub height: u32,
}

impl Game {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            cells: HashMap::new(),
            width,
            height,
        }
    }

    pub fn init(&mut self) {
        let mut rng = rand::thread_rng();

        for y in 0..self.height {
            for x in 0..self.width {
                if rng.gen_range(0..100) < 10 {
                    self.cells.insert(
                        (x as i32, y as i32),
                        Life34::new((x as i32, y as i32), true),
                    );
                }
            }
        }
    }

    pub fn update(&mut self) {
        let live_cells: &HashMap<(i32, i32), Life34> = &self.cells;
        let spots_to_check: HashSet<(i32, i32)> = self
            .cells
            .par_iter()
            .flat_map(|(_, cell)| {
                let mut spots = vec![cell.pos];
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        if dx != 0 || dy != 0 {
                            let neighbor_pos = (cell.pos.0 + dx, cell.pos.1 + dy);
                            spots.push(neighbor_pos);
                        }
                    }
                }
                spots
            })
            .collect();

        let new_cells: HashMap<(i32, i32), Life34> = spots_to_check
            .par_iter()
            .filter_map(|&spot| {
                let mut live_neighbors: usize = 0;

                for dx in -1..=1 {
                    for dy in -1..=1 {
                        if dx != 0 || dy != 0 {
                            let neighbor_pos = (spot.0 + dx, spot.1 + dy);

                            if neighbor_pos.0 >= 0
                                && neighbor_pos.0 < self.width as i32
                                && neighbor_pos.1 >= 0
                                && neighbor_pos.1 < self.height as i32
                            {
                                if live_cells.contains_key(&neighbor_pos) {
                                    live_neighbors += 1;
                                }
                            }
                        }
                    }
                }
                let cell = self
                    .cells
                    .get(&spot)
                    .cloned()
                    .unwrap_or(Life34::new(spot, false));
                let new_cell = cell.update(live_neighbors);

                if new_cell.alive {
                    Some((spot, new_cell))
                } else {
                    None
                }
            })
            .collect();

        self.cells = new_cells;
    }

    pub fn render(&self, window: &mut RenderWindow) {
        for cell in &self.cells {
            cell.1.render(window);
        }
    }
}
