use std::cmp::min;
use std::fmt;
use std::hash::Hash;

use krabmaga::{rand, Rng, thread_rng};
use krabmaga::engine::agent::Agent;
use krabmaga::engine::fields::field_2d::Location2D;
use krabmaga::engine::location::Real2D;
use krabmaga::engine::state::State;
use krabmaga::rand::seq::IteratorRandom;

use crate::model::robot_factory::RobotFactory;
use crate::model::stations::{Station, StationType};

//----------------Robot----------------
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum CarriedProduct {
    Nothing,
    Bolts,
    Cuttings,
    StichedStandard,
    StichedDeluxe,
    FinishedStandard,
    FinishedDeluxe,
}

impl CarriedProduct {
    fn get_destination_type_of_product(&mut self) -> StationType {
        match self {
            CarriedProduct::Nothing => StationType::LoadingDock,
            CarriedProduct::Bolts => StationType::Cutter,
            CarriedProduct::Cuttings => StationType::Sticher,
            CarriedProduct::StichedStandard | CarriedProduct::StichedDeluxe => StationType::Finisher,
            CarriedProduct::FinishedStandard | CarriedProduct::FinishedDeluxe => StationType::LoadingDock,
        }
    }
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

    pub fn charge(&mut self, amount: u32, state: &RobotFactory) {
        self.charge = min(self.max_charge as i32, self.charge + amount as i32);
        self.order = CarriedProduct::Nothing;

        if self.is_fully_charged() {
            if rand::thread_rng().gen_bool(0.5) {
                self.change_destination(state.get_random_station());
            } else {
                self.change_destination(state.get_random_station_with_type(StationType::LoadingDock));
            }
        }

        state.robot_grid.set_object_location(*self, self.location);
    }

    pub fn is_fully_charged(&self) -> bool {
        self.charge == self.max_charge as i32
    }


    fn move_step_towards_destination(&mut self, state: &RobotFactory) {
        let x = self.location.x;
        let y = self.location.y;
        //get direction vector
        let mut dx = self.destination.x - x;
        let mut dy = self.destination.y - y;

        //get length of direction vector

        //ensure max step disatance is 1 by normalization
        let distance = (dx * dx + dy * dy).sqrt();
        if distance > 1.0 {
            dx /= distance;
            dy /= distance;
        }

        //apply direction vector to current position
        self.location = Real2D { x: x + dx, y: y + dy };
        state.robot_grid.set_object_location(*self, self.location);

        if self.order == CarriedProduct::Nothing {
            self.charge -= 1;
        } else {
            self.charge -= 2;
        }
    }

    fn is_at_destination(&self, state: &RobotFactory) -> bool {
        let x = self.location.x;
        let y = self.location.y;
        //get direction vector
        let mut dx = self.destination.x - x;
        let mut dy = self.destination.y - y;

        let distance_sq = (dx * dx + dy * dy);
        return distance_sq < 0.01;
    }

    fn renew_destination(&mut self, state: &RobotFactory) {
        self.destination_type = self.order.get_destination_type_of_product();
        self.change_destination(state.get_random_station_with_type(self.destination_type));
    }

    fn drop_off_product(&mut self, station: &mut Station) {
        self.order = CarriedProduct::Nothing;
        station.material_management.increment_supply();
    }
}

impl Agent for Robot {
    fn step(&mut self, state: &mut dyn State) {
        //Set robot destination

        let mut robot_factory = state.as_any_mut().downcast_mut::<RobotFactory>().unwrap();

        if !self.is_at_destination(robot_factory) {
            self.move_step_towards_destination(robot_factory);
        } else {
            let mut neighbor_stations = robot_factory.station_grid.get_neighbors_within_distance(self.location, 0.01);
            let mut station = neighbor_stations.iter_mut()
                .filter(|station| station.get_station_type() == self.destination_type)
                .choose(&mut rand::thread_rng()).unwrap();

            match self.destination_type {
                StationType::LoadingDock => {
                    if station.has_product_available() {
                        self.order = station.take_product(&mut robot_factory);
                        self.renew_destination(robot_factory);
                    }
                }
                StationType::Sticher | StationType::Cutter | StationType::Finisher => {
                    self.drop_off_product(&mut station);
                    if station.has_product_available() { //todo: this should check all stations around the robot
                        self.order = station.take_product(&mut robot_factory);
                        self.renew_destination(robot_factory);
                    } else {
                        self.change_destination(robot_factory.get_random_station_with_type(StationType::LoadingDock));
                    }
                }
                StationType::StorageRoom => {
                    self.drop_off_product(&mut station);
                    self.order = station.take_product(&mut robot_factory); //storage room returns CarriedProduct::Nothing
                    self.renew_destination(robot_factory);
                }
                StationType::RobotRoom => {
                    //nothing to-do here
                }
            }
        }

        //return-home
        if self.charge < 2 {
            let loading_station = robot_factory.get_random_station_with_type(StationType::LoadingDock);
            self.change_destination(loading_station);
        }
    }
}
