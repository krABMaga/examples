use krabmaga::bevy::prelude::Color;

// mol by 47 to have different colors especially for similar near ID
pub fn body_color(id: u32, alpha: f32) -> Color {
    Color::hsla((id as f32 * 47.0) % 360.0, 0.85, 0.65, alpha)
}
