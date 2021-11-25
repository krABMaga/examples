use crate::model::state::WsgState;
use crate::FULL_GROWN;
use rust_ab::engine::fields::dense_number_grid_2d::DenseNumberGrid2D;
use rust_ab::engine::location::Int2D;
use rust_ab::{bevy::prelude::Texture, visualization::fields::number_grid_2d::BatchRender};

impl BatchRender<WsgState> for DenseNumberGrid2D<u16> {
    fn get_pixel(&self, pos: &Int2D) -> [u8; 4] {
        match self.get_value(pos) {
            Some(val) => {
                let growth = val;
                if (growth as f32 / FULL_GROWN as f32) < 0.5 {
                    [139u8, 69u8, 19u8, 180u8]
                } else if (growth as f32 / FULL_GROWN as f32) < 0.7 {
                    [128u8, 128u8, 0u8, 150u8]
                } else if growth == FULL_GROWN {
                    [0u8, 128u8, 0u8, 255u8]
                } else {
                    [0u8, 255u8, 0u8, 255u8]
                }
            }
            None => [0u8, 255u8, 0u8, 0u8],
        }
    }

    fn get_dimensions(&self) -> (u32, u32) {
        (self.width as u32, self.height as u32)
    }

    fn get_layer(&self) -> f32 {
        0.
    }

    fn get_texture_from_state(state: &WsgState) -> Texture {
        state.grass_field.texture()
    }
}
