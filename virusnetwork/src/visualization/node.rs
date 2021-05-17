use crate::model::node::*;
use crate::model::state::EpidemicNetworkState;
use rust_ab::{engine::location::Real2D, visualization::{renderable::{Render, SpriteType}, sprite_render_factory::{SpriteFactoryResource, SpriteRenderFactory}}};
use rust_ab::bevy::prelude::{Quat, Transform};
use rust_ab::engine::location::Int2D;


impl Render for NetNode {
    fn sprite(&self) -> SpriteType {
        match self.status{
            NodeStatus::Susceptible => {SpriteType::Emoji(String::from("white_circle"))}
            NodeStatus::Infected => {SpriteType::Emoji(String::from("red_circle"))}
            NodeStatus::Resistent => {SpriteType::Emoji(String::from("large_blue_circle"))}
        }
    }

    /// The position must always be fetched through the state, since that will be the one actually updated
    /// by the RustAB schedule. All objects will be rendered on the 0. z, except pheromones, which will be
    /// put on a lower z-axis.
    fn position(&self, state: &EpidemicNetworkState) -> (f32, f32, f32) {
        let loc = state.field1.get_object_location(*self);
        match loc {
            Some(pos) => (pos.x as f32, pos.y as f32, 0.),
            None => (self.pos.x as f32, self.pos.y as f32, 0.),
        }
        
    }

    /// Emojis are 64x64, way too big for our simulation
    fn scale(&self) -> (f32, f32) {
        (0.3, 0.3)
    }

    /// No rotation is needed for ants
    fn rotation(&self) -> f32 {
        0.
    }

    /// Simply update the transform based on the position chosen
    fn update(&mut self, transform: &mut Transform, state: &EpidemicNetworkState) {
        
        let (pos_x, pos_y, z) = self.position(state);
        let model_pos = Real2D {
            x: pos_x as f64,
            y: pos_y as f64,
        };

        let node = state.field1.get_objects_at_location(model_pos);
        let node = node.first();

        if let Some(current_node)  = node{
            self.status  = current_node.status;
        }

    }
}