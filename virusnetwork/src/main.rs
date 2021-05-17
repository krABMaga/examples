extern crate rust_ab;

use model::node::{self, NetNode};
use model::node::{NodeStatus, NodeStatus::*};
use model::state::EpidemicNetworkState;

use rust_ab::{bevy::{DefaultPlugins, prelude::{Commands, Msaa, OrthographicCameraBundle, Transform}}, engine::field::network::EdgeOptions::*, simulate};
use rust_ab::engine::field::network::*;
use rust_ab::engine::schedule::*;
use rust_ab::rand::Rng;
use rust_ab::{engine::agent::Agent, rand};
use rust_ab::{engine::location::Real2D, preferential_attachment_BA};
use std::{time::Instant, u128};

mod model;

static STEP: u128 = 50;
static NUM_NODES: u128 = 5;
static WIDTH: f64 = 200.0;
static HEIGTH: f64 = 208.0;
static DISCRETIZATION: f64 = 10.0 / 1.5;
static TOROIDAL: bool = false;

///Initial infected nodes
static INITIAL_INFECTED_PROB: f64 = 0.3;
static INIT_EDGE: u128 = 2;
static VIRUS_SPREAD_CHANCE: f64 = 0.3;
static VIRUS_CHECK_FREQUENCY: f64 = 0.2;
static RECOVERY_CHANCE: f64 = 0.30;
static GAIN_RESISTENCE_CHANCE: f64 = 0.20;

#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
fn main() {
    let mut schedule: Schedule<NetNode> = Schedule::new();
    let mut state = EpidemicNetworkState::new(WIDTH, HEIGTH, DISCRETIZATION, TOROIDAL);

    let nodes_set = gen_nodes(&mut state, &mut schedule);
    preferential_attachment_BA!(nodes_set, state.network, NetNode, String, INIT_EDGE);
    
    for node in state.network.edges.keys(){
        println!("Node {} has {} edges", node.id, state.network.getEdges(node).unwrap().len());
    }

    simulate!(STEP, schedule, NetNode, state);
}


fn gen_nodes(state: &mut EpidemicNetworkState, schedule: &mut Schedule<NetNode>) -> Vec<NetNode> {
    //Nodes Generation

    let mut nodes_set = Vec::new();
    let mut rng = rand::thread_rng();
    for node_id in 0..NUM_NODES {
        let r1: f64 = rng.gen();
        let r2: f64 = rng.gen();

        let init_status: NodeStatus = if rng.gen_bool(INITIAL_INFECTED_PROB) {
            Infected
        } else {
            Susceptible
        };

        let node = NetNode::new(
            node_id,
            Real2D {
                x: WIDTH * r1,
                y: HEIGTH * r2,
            },
            init_status,
        );

        state.field1.set_object_location(node, node.pos);
        state.network.addNode(&node);
        schedule.schedule_repeating(node, 0.0, 0);

        nodes_set.push(node);
    }
    let t = state.network.edges.len();
    println!("Nodes in the network:\t\t{}", t);
    nodes_set
}



// Visualization specific imports
#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
use {
    rust_ab::visualization::visualization::Visualization,
    crate::visualization::vis_state::VisState,
    rust_ab::bevy::prelude::Color,
    rust_ab::bevy::prelude::IntoSystem,
    rust_ab::visualization::field::number_grid_2d::BatchRender,
    bevy_prototype_lyon::prelude::*

};

#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
mod visualization;

// Main used when a visualization feature is applied.
#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
fn main() {
    // Initialize the simulation and its visualization here.
    let mut schedule: Schedule<NetNode> = Schedule::new();
    let mut state = EpidemicNetworkState::new(WIDTH, HEIGTH, DISCRETIZATION, TOROIDAL);

    
    let mut app = Visualization::default()
        .with_window_dimensions(800., 800.)
        .with_simulation_dimensions(500., 500.)
        .with_background_color(Color::rgb(0., 0., 0.))
        .setup::<NetNode, VisState>(VisState, state, schedule);

    app.add_plugin(ShapePlugin);

    app.add_startup_system(setup.system());
    app.run();

}

fn setup(mut commands: Commands) {
    let shape = shapes::RegularPolygon {
        sides: 6,
        feature: shapes::RegularPolygonFeature::Radius(20.0),
        ..shapes::RegularPolygon::default()
    };

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(GeometryBuilder::build_as(
        &shape,
        ShapeColors::outlined(Color::RED, Color::AZURE),
        DrawMode::Outlined {
            fill_options: FillOptions::default(),
            outline_options: StrokeOptions::default().with_line_width(10.0),
        },
        Transform::default(),
    ));
}