use crate::model::ant::Ant;
use crate::model::state::State;
use rust_ab::visualization::renderable::{Render, SpriteType};
use rust_ab::bevy::prelude::{Quat, Transform};
use rust_ab::engine::location::Int2D;

impl Render for Ant {
    fn sprite(&self) -> SpriteType {
        SpriteType::Emoji(String::from("ant"))
    }

    /// The position must always be fetched through the state, since that will be the one actually updated
    /// by the RustAB schedule. All objects will be rendered on the 0. z, except pheromones, which will be
    /// put on a lower z-axis.
    fn position(&self, state: &State) -> (f32, f32, f32) {
        let loc = state.get_ant_location(self);
        match loc {
            Some(pos) => (pos.x as f32, pos.y as f32, 0.),
            None => (self.loc.x as f32, self.loc.y as f32, 0.),
        }
    }

    /// Emojis are 64x64, way too big for our simulation
    fn scale(&self) -> (f32, f32) {
        (0.1, 0.1)
    }

    /// No rotation is needed for ants
    fn rotation(&self) -> f32 {
        let rotation = if let Some(Int2D{x, y}) = self.last {
            ((y - self.loc.y) as f32).atan2((x - self.loc.x) as f32)
        } else {
            0.
        };
        rotation
    }

    /// Simply update the transform based on the position chosen
    fn update(&mut self, transform: &mut Transform, state: &State) {

        let (pos_x, pos_y, z) = self.position(state);
        let (scale_x, scale_y) = self.scale();

        // Update our local ant copy positions to properly calculate rotation
        if let Some(scheduled_ant) = state.ants_grid.grid.get_object_at_location(&Int2D{x: pos_x as i64, y: pos_y as i64 }) {
            self.loc = scheduled_ant.loc;
            self.last = scheduled_ant.last;
        }

        let rotation = self.rotation();

        let translation = &mut transform.translation;
        translation.x = pos_x;
        translation.y = pos_y;
        translation.z = z;
        transform.scale.x = scale_x;
        transform.scale.y = scale_y;
        transform.rotation = Quat::from_rotation_z(rotation);
    }
}
