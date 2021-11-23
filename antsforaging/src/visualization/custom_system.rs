use crate::model::state::ModelState;
use crate::model::state::*;
use crate::model::to_food_grid::ToFoodGrid;
use crate::model::to_home_grid::ToHomeGrid;
use rust_ab::bevy::prelude::Texture;
use rust_ab::engine::location::Int2D;
use rust_ab::visualization::fields::number_grid_2d::BatchRender;

use rust_ab::engine::fields::dense_object_grid_2d::DenseGrid2D;
use rust_ab::engine::fields::sparse_object_grid_2d::SparseGrid2D;
use rust_ab::visualization::fields::object_grid_2d::RenderObjectGrid2D;

impl BatchRender<ModelState> for ToHomeGrid {
    fn get_pixel(&self, pos: &Int2D) -> [u8; 4] {
        match self.grid.get_value(pos) {
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

    fn get_texture_from_state(state: &ModelState) -> Texture {
        state.to_home_grid.texture()
    }
}

impl BatchRender<ModelState> for ToFoodGrid {
    fn get_pixel(&self, pos: &Int2D) -> [u8; 4] {
        match self.grid.get_value(pos) {
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

    fn get_texture_from_state(state: &ModelState) -> Texture {
        state.to_food_grid.texture()
    }
}

impl RenderObjectGrid2D<ModelState, Item> for SparseGrid2D<Item> {
    fn get_sparse_grid(state: &ModelState) -> Option<&SparseGrid2D<Item>> {
        Some(&state.obstacles_grid)
    }

    fn get_dense_grid(_state: &ModelState) -> Option<&DenseGrid2D<Item>> {
        None
    }

    fn get_emoji_obj(_state: &ModelState, obj: &Item) -> String {
        match obj.value {
            ItemType::Home => "house".to_string(),
            ItemType::Food => "candy".to_string(),
            ItemType::Obstacle => "no_entry_sign".to_string(),
            //_ => panic!("Object not recognized."),
        }
    }

    fn scale(obj: &Item) -> (f32, f32) {
        match obj.value {
            ItemType::Home => (0.1, 0.1),
            ItemType::Food => (0.1, 0.1),
            ItemType::Obstacle => (0.05, 0.05),
            //_ => panic!("Object not recognized."),
        }
    }

    fn get_pos_obj(state: &ModelState, obj: &Item) -> Option<Int2D> {
        state.obstacles_grid.get_location(*obj)
    }

    fn get_rotation_obj(_state: &ModelState, _obj: &Item) -> f32 {
        0.0
    }
}
