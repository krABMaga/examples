use rust_ab::{
    bevy::prelude::{Commands, ResMut},
};
use rust_ab::{engine::field::network::EdgeOptions::*};
use rust_ab::engine::location::Real2D;
use rust_ab::engine::schedule::*;
use rust_ab::rand;
use rust_ab::rand::Rng;
use rust_ab::visualization::on_state_init::OnStateInit;
use rust_ab::visualization::renderable::{Render, SpriteType};
use rust_ab::visualization::simulation_descriptor::SimulationDescriptor;
use rust_ab::visualization::sprite_render_factory::SpriteFactoryResource;

use crate::{HEIGTH, INITIAL_INFECTED_PROB, NUM_NODES, WIDTH};
use crate::model::{node::*, state::EpidemicNetworkState};

pub struct VisState;

impl OnStateInit<NetNode> for VisState {
    fn on_init(
        &self,
        mut commands: Commands,
        mut sprite_render_factory: SpriteFactoryResource,
        mut state: ResMut<EpidemicNetworkState>,
        mut schedule: ResMut<Schedule<NetNode>>,
        mut sim: ResMut<SimulationDescriptor>,
    ) {
        let nodes_set = gen_nodes(&mut state, &mut schedule, &mut sprite_render_factory, &mut commands);
        let n_nodes = nodes_set.len();
        state.network.network.remove_all_edges();

        if n_nodes == 0 {
            return;
        }
        state.network.network.add_node(&nodes_set[0]);
        state.network.network.edges.update();
        if n_nodes == 1 {
            return;
        }
        state.network.network.add_node(&nodes_set[1]);

        state.network.network.add_edge(&nodes_set[0], &nodes_set[1], Simple);
        state.network.network.edges.update();

        let init_edge: usize = 1;

        for i in 2..n_nodes {
            let node = nodes_set[i];

            state.network.network.add_prob_edge(&node, &init_edge);
            state.network.network.edges.update();
        }
    }
}

fn gen_nodes(state: &mut EpidemicNetworkState,
             schedule: &mut Schedule<NetNode>,
             sprite_render_factory: &mut SpriteFactoryResource,
             commands: &mut Commands,
) -> Vec<NetNode> {
    //Nodes Generation

    let mut nodes_set = Vec::new();
    let mut rng = rand::thread_rng();
    for node_id in 0..NUM_NODES {
        let r1: f64 = rng.gen();
        let r2: f64 = rng.gen();

        let init_status: NodeStatus = if rng.gen_bool(INITIAL_INFECTED_PROB) {
            NodeStatus::Infected
        } else {
            NodeStatus::Susceptible
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
        state.network.network.add_node(&node);
        schedule.schedule_repeating(node, 0.0, 0);

        let SpriteType::Emoji(emoji_code) = node.sprite();
        let sprite_render = sprite_render_factory.get_emoji_loader(emoji_code);


        node.setup_graphics(sprite_render, commands, &state);

        nodes_set.push(node);
    }
    let t = state.network.network.edges.len();
    println!("Nodes in the network:\t\t{}", t);
    nodes_set
}
