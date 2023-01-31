use std::cmp::min;
use std::fmt;
use std::hash::Hash;

use krabmaga::{rand, Rng};
use krabmaga::engine::agent::Agent;
use krabmaga::engine::fields::field_2d::Location2D;
use krabmaga::engine::location::Real2D;
use krabmaga::engine::state::State;

use crate::model::robot_factory::RobotFactory;
use crate::model::stations::{Station, StationType};

//----------------Robot----------------
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum CarriedProduct {
    Nothing,
    Bolts,
    Cuttings,
    Standard,
    Deluxe,
}


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Robot {
    id: u32,
    max_charge: u32,
    pub(crate) charge: i32,
    location: Real2D,
    destination: Real2D,
    destination_type: StationType,
    order: CarriedProduct,
}

impl Robot {
    pub fn change_destination(&mut self, target: (StationType, Real2D)) {
        self.destination = target.1;
        self.destination_type = target.0;
    }

    pub fn get_charge(&self) -> i32 {
        self.charge
    }

    pub fn get_max_charge(&self) -> u32 {
        self.max_charge
    }

    pub fn set_order(&mut self, order: CarriedProduct) {
        self.order = order;
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_id_mut(&mut self) -> u32 {
        self.id
    }
}

impl Location2D<Real2D> for Robot {
    fn get_location(self) -> Real2D {
        self.location
    }

    fn set_location(&mut self, location: Real2D) {
        self.location = location;
    }
}

impl Hash for Robot {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl fmt::Display for Robot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Robot(id: {} at {}, dest {}, carries {:?})", self.id, self.location, self.destination, self.order)
    }
}

impl Robot {
    pub fn new(id: u32, location: Real2D, state: &dyn State) -> Robot {
        let default_max_charge = 350;
        let robot_factory = state.as_any().downcast_ref::<RobotFactory>().unwrap();
        let initial_destination = robot_factory.get_random_station_with_type(StationType::LoadingDock).1;
        Robot {
            id,
            max_charge: default_max_charge,
            charge: default_max_charge as i32,
            location,
            destination: initial_destination,
            destination_type: StationType::LoadingDock,
            order: CarriedProduct::Nothing,
        }
    }

    pub fn charge(&mut self, amount: i32, state: &RobotFactory) {
        self.charge = min(self.max_charge as i32, self.charge + amount);
        self.order = CarriedProduct::Nothing;

        if self.is_fully_charged() {
            if rand::thread_rng().gen_bool(0.5) {
                self.change_destination(state.get_random_station());
            } else {
                self.change_destination(state.get_random_station_with_type(StationType::LoadingDock));
            }
        }
    }

    pub fn is_fully_charged(&self) -> bool {
        self.charge == self.max_charge as i32
    }


    fn move_step_towards_destination(&mut self, _state: &RobotFactory) {
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
        let robot_factory = state.as_any().downcast_ref::<RobotFactory>().unwrap();
    }
}
