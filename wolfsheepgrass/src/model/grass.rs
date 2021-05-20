use rust_ab::engine::field::number_grid_2d::NumberGrid2D;
pub const FULL_GROWN: u16 = 1000;

/// Represents food pheromones. Higher f64 value means more concentrated pheromone.

pub struct GrassField {
    pub grid: NumberGrid2D<u16>,
}

impl GrassField {
    pub fn new(width: i64, height: i64) -> GrassField {
        GrassField {
            grid: NumberGrid2D::new(width, height),
        }
    }

    pub fn update(&mut self) {
        self.grid.update();
        self.grid.locs.apply_to_all_values(|grass| {
            let growth = *grass;
            if growth < FULL_GROWN {
                growth + 1
            } else {
                growth
            }
        });
    }
}
