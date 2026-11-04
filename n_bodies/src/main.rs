mod model;
#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
mod visualization;

// Simulation area. The default window of Bevy is 1280x720 and krABMaga
// reserves 300 px on the left for its panel, so (1280-300) x 720 gives a
// 1:1 scale (circles stay circles).
const DIM: (f32, f32) = (980.0, 720.0);

#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
use krabmaga::*;

#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
fn main() {
    use crate::model::state::{NBodyState, SetupMode};

    let mut state = NBodyState::new(DIM); // starts from Params::default()

    // Change whatever you want here, like the menu does in the window...
    state.p.mode = SetupMode::Random;
    state.p.number = 100;
    state.p.g = 100.0;
    state.p.dt = 0.05;

    // ...then regenerate the bodies: setup() reads the parameters above.
    state.setup();

    // simulate!(state, steps, repetitions)
    simulate!(state, 1000, 1);
}

#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
fn main() {
    use bevy_egui::EguiPrimaryContextPass;
    use krabmaga::bevy::prelude::{Color, Update};
    use krabmaga::visualization::visualization::Visualization;

    use crate::model::state::NBodyState;
    use crate::visualization::{menu::menu_system, trails::draw_trails, vis_state::NBodyVis};

    let state = NBodyState::new(DIM);

    // `setup` (instead of `start`) returns the Bevy App, so we can add our own systems.
    let mut app = Visualization::default()
        .with_window_dimensions(1280., 720.)
        .with_simulation_dimensions(DIM.0, DIM.1)
        .with_background_color(Color::srgb(0.02, 0.02, 0.08))
        .with_name("N-Bodies")
        .setup::<NBodyVis, NBodyState>(NBodyVis, state);

    app.add_systems(Update, draw_trails)
        .add_systems(EguiPrimaryContextPass, menu_system);

    app.run();
}
