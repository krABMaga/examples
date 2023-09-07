use cfg_if::cfg_if;
use krabmaga::cfg_if;
cfg_if! {
    if #[cfg(any(feature = "distributed_mpi"))]
    {
        use crate::model::state::Flocker;
        use krabmaga::UNIVERSE;

        mod model;

        // No visualization specific imports
        #[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
        use {
            krabmaga::engine::schedule::Schedule, krabmaga::engine::state::State,
            krabmaga::simulate_mpi, krabmaga::Info, /* krabmaga::ProgressBar, */ krabmaga::*,
            std::time::Duration,
        };

        //use krabmaga::*;

        // Visualization specific imports
        #[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
        use {
            crate::visualization::vis_state::VisState, krabmaga::bevy::prelude::Color,
            krabmaga::visualization::visualization::Visualization,
        };

        #[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
        mod visualization;

        pub static COHESION: f32 = 0.8;
        pub static AVOIDANCE: f32 = 1.0;
        pub static RANDOMNESS: f32 = 1.1;
        pub static CONSISTENCY: f32 = 0.7;
        pub static MOMENTUM: f32 = 1.0;
        pub static JUMP: f32 = 0.7;
        pub static DISCRETIZATION: f32 = 10.0 / 1.5;
        pub static TOROIDAL: bool = true;

        // Main used when only the simulation should run, without any visualization.
        #[cfg(not(any(feature = "visualization", feature = "visualization_wasm", feature = "distributed_mpi")))]
        fn main() {
            println!("Exiting");
        }

        #[cfg(any(feature = "distributed_mpi"))]
        fn main() {
            let step = 200;

            let dim = (1131., 1131.);
            let num_agents = 128000;

            let state = Flocker::new(dim, num_agents);
            let _ = simulate_mpi!(state, step, 1, Info::Normal);
        }

        // Main used when a visualization feature is applied.
        #[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
        fn main() {
            let dim = (200., 200.);
            let num_agents = 100;
            let state = Flocker::new(dim, num_agents);
            Visualization::default()
                .with_window_dimensions(1000., 700.)
                .with_simulation_dimensions(dim.0 as f32, dim.1 as f32)
                .with_background_color(Color::rgb(0., 0., 0.))
                .with_name("Flockers")
                .start::<VisState, Flocker>(VisState, state);
        }
    }
    else {
        fn main() {}
    }
}
