use crate::{EVAPORATION, FOOD_LOW_PHEROMONE};
use krabmaga::engine::fields::field::Field;
use krabmaga::engine::fields::grid_option::GridOption;
use krabmaga::engine::fields::sparse_number_grid_2d::SparseNumberGrid2D;

// Represents food pheromones. Higher f32 value means more concentrated pheromone.
pub struct ToFoodGrid {
    pub grid: SparseNumberGrid2D<f32>,
}

impl ToFoodGrid {
    pub fn new(width: i32, height: i32) -> ToFoodGrid {
        ToFoodGrid {
            grid: SparseNumberGrid2D::new(width, height),
        }
    }

    pub fn update(&mut self) {
        self.grid.update();
        self.grid.apply_to_all_values(
            |val| {
                let new_val = val * EVAPORATION;
                if new_val < FOOD_LOW_PHEROMONE {
                    0.
                } else {
                    new_val
                }
            },
            GridOption::READ,
        )
    }
}
