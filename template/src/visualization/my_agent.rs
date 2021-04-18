use rust_ab::visualization::renderable::{Render, SpriteType};
use crate::model::my_agent::MyAgent;
use rust_ab::bevy::prelude::{Transform, Quat};

/// Define how your agent should be rendered here.
impl Render for MyAgent{
    /// Specify the assets to use. Swap "bird" with the file name of whatever emoji you want to use.
    /// Be sure to also copy the asset itself in the assets/emojis folder. In future, this limitation will
    /// be removed.
    fn sprite(&self) -> SpriteType {
        SpriteType::Emoji(String::from("bird"))
    }

    /// Specify where the agent should be rendered in the window.
    fn position(&self, _state: &Self::SimState) -> (f32, f32, f32) {
        (250.,250.,0.)
    }

    /// Specify how much the texture should be scaled by. A common scale is (0.1, 0.1).
    fn scale(&self) -> (f32, f32) {
        (1.,1.)
    }

    /// Define the degrees in radians to rotate the texture by.
    fn rotation(&self) -> f32 {
        0.
    }

    /// Specify the code to execute for each frame, for each agent.
    fn update(&mut self, transform: &mut Transform, state: &Self::SimState) {

        // This snippet updates the agent position, scale and rotation for each frame.
        let (pos_x, pos_y, pos_z) = self.position(state);
        let (scale_x, scale_y) = self.scale();
        let rotation = self.rotation();

        let translation = &mut transform.translation;
        translation.x = pos_x;
        translation.y = pos_y;
        translation.z = pos_z;
        transform.scale.x = scale_x;
        transform.scale.y = scale_y;
        transform.rotation = Quat::from_rotation_z(rotation);
    }
}