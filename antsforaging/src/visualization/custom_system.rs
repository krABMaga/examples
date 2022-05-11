use crate::model::state::ModelState;
use crate::model::state::*;
use crate::model::to_food_grid::ToFoodGrid;
use crate::model::to_home_grid::ToHomeGrid;
use krabmaga::bevy::prelude::Image;
use krabmaga::engine::location::Int2D;
use krabmaga::visualization::fields::number_grid_2d::BatchRender;

use krabmaga::engine::fields::dense_object_grid_2d::DenseGrid2D;
use krabmaga::engine::fields::sparse_object_grid_2d::SparseGrid2D;
use krabmaga::visualization::fields::object_grid_2d::RenderObjectGrid2D;

impl BatchRender<ModelState> for ToHomeGrid {
    fn get_pixel(&self, loc: &Int2D) -> [u8; 4] {
        match self.grid.get_value(loc) {
            Some(val) => {
                let cell = val;

                let alpha = if cell < 0.01 {
                    50u8
                } else if cell < 0.1 {
                    80u8
                } else {
                    255u8
                };
                [0u8, 255u8, 0u8, alpha]
            }
            None => [0u8, 255u8, 0u8, 0u8],
        }
    }

    fn get_dimensions(&self) -> (u32, u32) {
        (self.grid.width as u32, self.grid.height as u32)
    }

    fn get_layer(&self) -> f32 {
        0.
    }

    fn get_texture_from_state(state: &ModelState) -> Image {
        state.to_home_grid.texture()
    }
}

impl BatchRender<ModelState> for ToFoodGrid {
    fn get_pixel(&self, loc: &Int2D) -> [u8; 4] {
        match self.grid.get_value(loc) {
            Some(val) => {
                let cell = val;

                let alpha = if cell < 0.01 {
                    50u8
                } else if cell < 0.1 {
                    80u8
                } else {
                    255u8
                };
                [0u8, 0u8, 255u8, alpha]
            }
            None => [0u8, 0u8, 255u8, 0u8],
        }
    }

    fn get_dimensions(&self) -> (u32, u32) {
        (self.grid.width as u32, self.grid.height as u32)
    }

    fn get_layer(&self) -> f32 {
        0.
    }

    fn get_texture_from_state(state: &ModelState) -> Image {
        state.to_food_grid.texture()
    }
}

impl RenderObjectGrid2D<ModelState, Item> for SparseGrid2D<Item> {
    fn fetch_sparse_grid(state: &ModelState) -> Option<&SparseGrid2D<Item>> {
        Some(&state.obstacles_grid)
    }

    fn fetch_dense_grid(_state: &ModelState) -> Option<&DenseGrid2D<Item>> {
        None
    }

    fn fetch_emoji(_state: &ModelState, obj: &Item) -> String {
        match obj.value {
            ItemType::Home => "house".to_string(),
            ItemType::Food => "candy".to_string(),
            ItemType::Obstacle => "no_entry_sign".to_string(),
            //_ => panic!("Object not recognized."),
        }
    }

    fn fetch_loc(state: &ModelState, obj: &Item) -> Option<Int2D> {
        state.obstacles_grid.get_location(*obj)
    }

    fn fetch_rotation(_state: &ModelState, _obj: &Item) -> f32 {
        0.0
    }

    fn scale(obj: &Item) -> (f32, f32) {
        match obj.value {
            ItemType::Home => (0.1, 0.1),
            ItemType::Food => (0.1, 0.1),
            ItemType::Obstacle => (0.05, 0.05),
            //_ => panic!("Object not recognized."),
        }
    }
}
