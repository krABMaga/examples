use std::time::Instant;

use krabmaga::engine::{Entity, Query, Res};
use krabmaga::engine::agent::AgentFactory;
use krabmaga::engine::components::double_buffer::{DBRead, DBWrite};
use krabmaga::engine::components::position::Real2DTranslation;
use krabmaga::engine::fields::field_2d::{Field2D, toroidal_distance, toroidal_transform};
use krabmaga::engine::location::Real2D;
use krabmaga::engine::resources::engine_configuration::EngineConfiguration;
use krabmaga::engine::rng::RNG;
use krabmaga::engine::simulation::Simulation;

use crate::model::bird::{Bird, LastReal2D};

mod model;

pub static COHESION: f32 = 0.8;
pub static AVOIDANCE: f32 = 1.0;
pub static RANDOMNESS: f32 = 1.1;
pub static CONSISTENCY: f32 = 0.7;
pub static MOMENTUM: f32 = 1.0;
pub static JUMP: f32 = 0.7;
pub static DISCRETIZATION: f32 = 10.0 / 1.5;
pub static TOROIDAL: bool = true;
pub static STEPS: u32 = 200;
pub static DIM_X: f32 = 800.;
pub static DIM_Y: f32 = 800.;
pub static NUM_AGENTS: u32 = 200;
pub static SEED: u64 = 1337;


// Main used when only the simulation should run, without any visualization.
#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
fn main() {
    let mut simulation = build_simulation(Simulation::build().with_steps(STEPS));
    let now = Instant::now();
    simulation.run();
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}, steps per second: {}", elapsed, STEPS as f64 / elapsed.as_secs_f64());
}

fn build_simulation(simulation: Simulation) -> Simulation {
    let field: Field2D<Entity> = Field2D::new(DIM_X, DIM_Y, DISCRETIZATION, TOROIDAL);

    let mut simulation = simulation
        .register_double_buffer::<Real2DTranslation>()
        .register_double_buffer::<LastReal2D>()
        .register_step_handler(step_system)
        //.with_num_threads(20)
        // .with_rng(SEED) // We cannot use this during parallel iteration due to mutable access being required for RNG.
        .with_engine_configuration(EngineConfiguration::new(Real2D { x: DIM_X, y: DIM_Y }, SEED)); // TODO abstract
    // TODO figure out how configs should work. Either split engine config and simulation config, requiring the latter to be registered, or...?
    init_world(&mut simulation, field);

    simulation
}

// TODO: remove this. The user should specify a bundle representing the agent (AgentBundle) with the component it requires.
// TODO: there needs to be a way to specify initialization logic too though.
// TODO: this bundle prototype must be passed to the simulation so that, along with NUM_AGENTS, the simulation
// TODO: can be programmatically restarted.
fn init_world(simulation: &mut Simulation, field: Field2D<Entity>) {
    for bird_id in 0..NUM_AGENTS {
        let mut rng = RNG::new(SEED, bird_id as u64);
        let r1: f32 = rng.gen();
        let r2: f32 = rng.gen();

        let position = Real2D { x: DIM_X * r1, y: DIM_Y * r2 };
        let current_pos = Real2DTranslation(position);

        let mut agent = AgentFactory::new(simulation);

        agent
            .insert_data(Bird { id: bird_id })
            .insert_double_buffered(LastReal2D::new(Real2D { x: 0., y: 0. }))
            .insert_double_buffered(current_pos);
        //println!("Bird #{} init: ({}, {}), {} - {}", bird_id, position.x, position.y, r1, r2);
    }

    simulation.add_field(field);
}

// TODO couple DBRead and DBWrite queries in a single systemparam
// TODO assume step systems will always query all the components added to an entity and make a systemparam grouping all of them automatically? Splitting up will hardly matter since inner parallelism with par queries will always be better
// TODO compare with 2024 flockers step code
fn step_system(mut query: Query<(Entity, &Bird, &DBRead<Real2DTranslation>, &DBRead<LastReal2D>, &mut DBWrite<Real2DTranslation>, &mut DBWrite<LastReal2D>)>,
               neighbour_query: Query<(&DBRead<Real2DTranslation>, &DBRead<LastReal2D>)>,
               field_query: Query<&Field2D<Entity>>,
               config: Res<EngineConfiguration>) {
    let field = field_query.single();
    //println!("Step #{}", config.current_step);
    let now = Instant::now();
    query.par_iter_mut().for_each(|(entity, bird, cur_pos, last_pos, mut w_cur_pos, mut w_last_pos)| {
        let cur_pos = cur_pos.0.0;
        let last_pos = last_pos.0.0;

        let mut neighbours = field.get_neighbors_within_relax_distance(cur_pos, 10.);
        neighbours.retain(|x| *x != entity);

        let (mut x_avoidance, mut y_avoidance) = (0., 0.);
        let (mut x_cohesion, mut y_cohesion) = (0., 0.);
        let (mut x_consistency, mut y_consistency) = (0., 0.);
        let (mut x_randomness, mut y_randomness) = (0., 0.);
        let (x_momentum, y_momentum) = (last_pos.x, last_pos.y);
        // Previously we had a check for neighbours being empty, but the check was actually pointless since the vec always contained at least {bird}.
        let mut count = 0;
        for (elem_loc, last_elem_loc) in neighbour_query.iter_many(neighbours) {
            let elem_loc = elem_loc.0.0;
            let last_elem_loc = last_elem_loc.0.0;

            let dx = toroidal_distance(cur_pos.x, elem_loc.x, DIM_X);
            let dy = toroidal_distance(cur_pos.y, elem_loc.y, DIM_Y);
            count += 1;

            //avoidance calculation
            let square = dx * dx + dy * dy;
            x_avoidance += dx / (square * square + 1.0);
            y_avoidance += dy / (square * square + 1.0);

            //cohesion calculation
            x_cohesion += dx;
            y_cohesion += dy;

            //consistency calculation
            x_consistency += last_elem_loc.x;
            y_consistency += last_elem_loc.y;
        }
        //println!("Elapsed 2 agent {}: {:?}", bird.id, now.elapsed());
        if count > 0 {
            x_avoidance /= count as f32;
            y_avoidance /= count as f32;
            x_cohesion /= count as f32;
            y_cohesion /= count as f32;
            x_consistency /= count as f32;
            y_consistency /= count as f32;

            x_consistency /= count as f32;
            y_consistency /= count as f32; // Old code did this division twice
        }

        x_avoidance *= 400.;
        y_avoidance *= 400.;
        x_cohesion = -x_cohesion / 10.;
        y_cohesion = -y_cohesion / 10.;
        // We cannot cache the RNG during parallel iteration since generating a number requires mutating the RNG itself.
        // Finding a way to assign one rng per thread generated by par_iter is probably overkill considering the little gain it provides.
        let mut rng = RNG::new(config.rand_seed, bird.id as u64 + config.current_step as u64);
        rng.set_stream(bird.id as u64 + config.current_step as u64);
        let r1 = rng.gen() * 2. - 1.;
        let r2 = rng.gen() * 2. - 1.;
        let square = (r1 * r1 + r2 * r2).sqrt();
        x_randomness = 0.05 * r1 / square;
        y_randomness = 0.05 * r2 / square;
        //println!("Elapsed 3 agent {}: {:?}", bird.id, now.elapsed());
        let mut dx = COHESION * x_cohesion
            + AVOIDANCE * x_avoidance
            + CONSISTENCY * x_consistency
            + RANDOMNESS * x_randomness
            + MOMENTUM * x_momentum;
        let mut dy = COHESION * y_cohesion
            + AVOIDANCE * y_avoidance
            + CONSISTENCY * y_consistency
            + RANDOMNESS * y_randomness
            + MOMENTUM * y_momentum;

        let dis = (dx * dx + dy * dy).sqrt();
        if dis > 0.0 {
            dx = dx / dis * JUMP;
            dy = dy / dis * JUMP;
        }

        let loc_x = toroidal_transform(cur_pos.x + dx, DIM_X);
        let loc_y = toroidal_transform(cur_pos.y + dy, DIM_Y);
        /* if config.current_step == 200 {
            println!("Bird {} - Step {}: - cohesion {:?}, avoidance {:?}, consistency {:?}, randomness {:?}, mom {:?}, loc {:?}",
                     bird.id, config.current_step, (x_cohesion, y_cohesion), (x_avoidance,y_avoidance), (x_consistency,y_consistency), (x_randomness, y_randomness),
                     (x_momentum, y_momentum), (loc_x, loc_y));
        } */


        // TODO this is ugly, but if we unify read and write buffers we'll end up querying both all the time even when it's not needed
        // TODO perhaps give the user a way to query only read or both read and write, and proxy methods accordingly
        w_last_pos.0 = LastReal2D::new(Real2D { x: dx, y: dy });
        w_cur_pos.0 = Real2DTranslation(Real2D { x: loc_x, y: loc_y });
        //println!("Elapsed 4 agent {}: {:?}", bird.id, now.elapsed());
    });
    //println!("Elapsed: {:?}", now.elapsed());
}
//
// // Main used when a visualization feature is applied.
// #[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
// fn main() {
//     let dim = (200., 200.);
//     let num_agents = 100;
//     let state = Flocker::new(dim, num_agents);
//     Visualization::default()
//         .with_window_dimensions(1000., 700.)
//         .with_simulation_dimensions(dim.0, dim.1)
//         .with_background_color(Color::rgb(0., 0., 0.))
//         .with_name("Flockers")
//         .start::<VisState, Flocker>(VisState, state);
// }
