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
    }
    else {
        fn main() {}
    }
}
