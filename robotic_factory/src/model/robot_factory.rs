use std::cmp::min;
use std::hash::Hash;

use krabmaga::engine::agent::Agent;
use krabmaga::engine::location::Real2D;
use krabmaga::engine::state::State;

use crate::model::order::Order;
use crate::model::stations::*;

#[derive(Clone)]
struct RobotFactory {
    robots: Vec<Robot>,
    stichers: Vec<SticherStation>,
    cutters: Vec<CuttingStation>,
    finishers_standard: FinisherStation,
    finishers_luxury: FinisherStation,
    loading_docks: LoadingDock,
    storage_room: StorageRoom,
    robot_room: RobotRoom,
}

//----------------Robot----------------
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum RobotState {
    CarryingOrder,
    Moving,
    Charging,
    Idle,
}

#[derive(Clone)]
pub struct Robot {
    max_charge: u32,
    charge: i32,
    location: Real2D,
    destination: Real2D,
    order: Option<Order>,
    state: RobotState,
}

impl Robot {
    pub fn charge(&mut self, p0: i32) {
        self.charge = min(self.max_charge as i32, self.charge + p0);
    }
}

impl Robot {
    pub fn new() -> Robot {
        let default_max_charge = 350;
        Robot {
            max_charge: default_max_charge,
            charge: default_max_charge as i32,
            location: Real2D { x: 0.0, y: 0.0 },
            destination: Real2D { x: 0.0, y: 0.0 },
            order: None,
            state: RobotState::Idle,
        }
    }

    fn move_step_towards_destination(&mut self, _state: &mut RobotFactory) {
        let x = self.location.x;
        let y = self.location.y;
        let mut dx = self.destination.x - x;
        let mut dy = self.destination.y - y;
        let distance = (dx * dx + dy * dy).sqrt();

        //normalize movement vector
        dx /= distance;
        dy /= distance;

        let _new_position = Real2D {
            x: x + dx,
            y: y + dy,
        };
        //state.move_robot(self, new_position);
    }
}

impl Agent for Robot {
    fn step(&mut self, state: &mut dyn State) {
        let _robot_factory = state.as_any().downcast_ref::<RobotFactory>().unwrap();
    }
}
