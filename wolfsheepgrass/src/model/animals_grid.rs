use rust_ab::engine::field::object_grid_2d::Grid2D;

use crate::model::animals::Animal;

/// Represents the main grid containing ants and their location.
/// As for now it serves more of a logging purpose than anything,
/// in future it can be used to run operations on all the ants of the simulation,
/// for example to disable their sprites to be able to focus on the pheromones.
pub struct AnimalsGrid {
    pub grid: Grid2D<Animal>,
}

impl AnimalsGrid {
    pub fn new(width: i64, height: i64) -> AnimalsGrid {
        AnimalsGrid {
            grid: Grid2D::new(width, height),
        }
    }

    pub(crate) fn update(&mut self) {
        self.grid.update();
    }
}
