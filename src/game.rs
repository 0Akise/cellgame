use super::cell::{HighLife, Life34};
use rand::Rng;
use rayon::prelude::*;
use sfml::graphics::RenderWindow;
use std::collections::{HashMap, HashSet};

#[derive(Clone)]
pub enum CellType {
    Life34(Life34),
    HighLife(HighLife),
}

pub struct Game {
    pub cells: HashMap<(i32, i32), CellType>,
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
                    let cell_type = if rng.gen_bool(0.2) {
                        CellType::HighLife(HighLife::new((x as i32, y as i32), true))
                    } else {
                        CellType::Life34(Life34::new((x as i32, y as i32), true))
                    };
                    self.cells.insert((x as i32, y as i32), cell_type);
                }
            }
        }
    }

    pub fn update(&mut self) {
        let live_cells: &HashMap<(i32, i32), CellType> = &self.cells;

        let spots_to_check: HashSet<(i32, i32)> = self
            .cells
            .par_iter()
            .flat_map(|(_, cell)| {
                let mut spots = vec![match cell {
                    CellType::HighLife(cell) => cell.pos,
                    CellType::Life34(cell) => cell.pos,
                    // Add more cell types here as needed
                }];
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        let neighbor_pos = (spots[0].0 + dx, spots[0].1 + dy);
                        spots.push(neighbor_pos);
                    }
                }
                spots
            })
            .collect();

        let new_cells: HashMap<(i32, i32), CellType> = spots_to_check
            .par_iter()
            .filter_map(|&spot| {
                let mut live_neighbors: usize = 0;
                let mut highlife_neighbors: usize = 0;
                let mut life34_neighbors: usize = 0;

                for dx in -1..=1 {
                    for dy in -1..=1 {
                        if dx != 0 || dy != 0 {
                            let neighbor_pos = (spot.0 + dx, spot.1 + dy);

                            if neighbor_pos.0 >= 0
                                && neighbor_pos.0 < self.width as i32
                                && neighbor_pos.1 >= 0
                                && neighbor_pos.1 < self.height as i32
                            {
                                if let Some(neighbor) = live_cells.get(&neighbor_pos) {
                                    live_neighbors += 1;
                                    match neighbor {
                                        CellType::HighLife(_) => highlife_neighbors += 1,
                                        CellType::Life34(_) => life34_neighbors += 1,
                                        // Add more cell types here as needed
                                    }
                                }
                            }
                        }
                    }
                }

                let cell = self.cells.get(&spot).cloned();
                match cell {
                    Some(CellType::HighLife(cell)) => {
                        let new_cell = cell.update(live_neighbors);
                        if new_cell.alive {
                            Some((spot, CellType::HighLife(new_cell)))
                        } else {
                            None
                        }
                    }
                    Some(CellType::Life34(cell)) => {
                        let new_cell = cell.update(live_neighbors);
                        if new_cell.alive {
                            Some((spot, CellType::Life34(new_cell)))
                        } else {
                            None
                        }
                    }
                    None if live_neighbors == 3 => {
                        // Create a new cell of the same type as the majority of the neighbors
                        if highlife_neighbors >= life34_neighbors {
                            Some((spot, CellType::HighLife(HighLife::new(spot, true))))
                        } else {
                            Some((spot, CellType::Life34(Life34::new(spot, true))))
                        }
                    }
                    _ => None,
                }
            })
            .collect();

        self.cells = new_cells;
    }

    pub fn render(&self, window: &mut RenderWindow) {
        for cell in &self.cells {
            match cell.1 {
                CellType::HighLife(cell) => cell.render(window),
                CellType::Life34(cell) => cell.render(window),
            }
        }
    }
}
