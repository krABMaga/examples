use rust_ab::bevy::prelude::Texture;
use rust_ab::visualization::field::number_grid_2d::{BatchRender};
use crate::model::to_food_grid::ToFoodGrid;
use crate::model::state::State;
use rust_ab::engine::location::Int2D;
use crate::model::to_home_grid::ToHomeGrid;
use crate::model::ant::Ant;

impl BatchRender<Ant> for ToHomeGrid {
    fn get_pixel(&self, pos: &Int2D) -> [u8; 4] {
        match self.grid.get_value_at_pos(pos) {
            Some(val) => {
                let cell = *val;

                let alpha = if cell < 0.01 {
                    50u8
                } else if cell < 0.1 {
                    80u8
                } else {
                    255u8
                };
                [0u8, 255u8, 0u8, alpha]
            }//Some((*val * 200.) as u8),
            None => [0u8, 255u8, 0u8, 0u8],
        }
    }

    fn get_dimensions(&self) -> (u32, u32) {
        (self.grid.width as u32, self.grid.height as u32)
    }

    fn get_layer(&self) -> f32 {
        0.
    }

    fn get_texture_from_state(state: &State) -> Texture {
        state.to_home_grid.texture()
    }
}

impl BatchRender<Ant> for ToFoodGrid {
    fn get_pixel(&self, pos: &Int2D) -> [u8; 4] {
        match self.grid.get_value_at_pos(pos) {
            Some(val) => {
                let cell = *val;

                let alpha = if cell < 0.01 {
                    50u8
                } else if cell < 0.1 {
                    80u8
                } else {
                    255u8
                };
                [0u8, 0u8, 255u8, alpha]
            }//Some((*val * 200.) as u8),
            None => [0u8, 0u8, 255u8, 0u8],
        }
    }

    fn get_dimensions(&self) -> (u32, u32) {
        (self.grid.width as u32, self.grid.height as u32)
    }

    fn get_layer(&self) -> f32 {
        0.
    }

    fn get_texture_from_state(state: &State) -> Texture {
        state.to_food_grid.texture()
    }
}