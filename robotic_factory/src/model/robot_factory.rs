use std::any::Any;
use std::cell::RefCell;
use std::cmp::min;
use std::hash::Hash;
use std::panic::Location;

use krabmaga::{rand, Rng};
use krabmaga::engine::agent::Agent;
use krabmaga::engine::fields::dense_object_grid_2d::DenseGrid2D;
use krabmaga::engine::fields::field_2d::{Field2D, Location2D};
use krabmaga::engine::location::Real2D;
use krabmaga::engine::schedule::Schedule;
use krabmaga::engine::state::State;

use crate::model::stations::*;

pub struct RobotFactory {
    robots: Vec<RefCell<Robot>>,
    stations: Vec<Station>,

    pub standard_order_count: u32,
    pub luxury_order_count: u32,

    sticher_grid: Field2D<Station>,
}

impl RobotFactory {
    ///
    /// Finds a random station and **returns a copy** of it.
    ///
    pub(crate) fn get_random_station(&self) -> &Station {
        let stations = self.stations.clone();
        return self.get_random_entry(stations);
    }


    pub(crate) fn get_robots(&self) -> Vec<RefCell<Robot>> {
        return Vec::from(&self.robots[..]);
    }

    pub fn get_location_of_any_station_with_type(&self, destination_type: StationType) -> Real2D {
        let stations_of_type = self.stations.iter().filter(|x| { x.get_station_type() == destination_type }).collect();
        return self.get_random_entry(stations_of_type).get_location();
    }

    fn get_random_entry<T>(&self, list: Vec<T>) -> &T {
        let index = rand::thread_rng().gen_range(0..list.len());
        &list[index]
    }

    /// Picks a random order, deletes it and returns whether it was a luxury order or not.
    ///
    /// _Returns_: None if there are no orders Else returns Some(true) if it was a luxury order
    /// or Some(false) if it was a standard order
    ///
    pub fn retrieve_order(&mut self) -> Option<bool> {
        if self.standard_order_count == 0 && self.luxury_order_count == 0 {
            return None;
        }

        let a_random_u32 = rand::thread_rng().gen_range(0..(self.standard_order_count + self.luxury_order_count));
        return if a_random_u32 < self.standard_order_count {
            self.standard_order_count -= 1;
            Some(false)
        } else {
            self.luxury_order_count -= 1;
            Some(true)
        };
    }

    pub fn bump_required_orders(&mut self, is_deluxe: bool) {
        if is_deluxe {
            self.luxury_order_count += 1;
        } else {
            self.standard_order_count += 1;
        }
    }
}


impl State for RobotFactory {
    fn init(&mut self, schedule: &mut Schedule) {
        //spawn 1 loading dock, 2 stichers, 2 cutters, 2 finishers, 1 storage room, 1 robot room
        self.stations.push(Station::new(self.stations.len() as u32, Real2D { x: 0.0, y: 0.0 }, StationType::LoadingDock, false));
        self.stations.push(Station::new(self.stations.len() as u32, Real2D { x: 0.0, y: 0.0 }, StationType::Sticher, false));
        self.stations.push(Station::new(self.stations.len() as u32, Real2D { x: 0.0, y: 0.0 }, StationType::Sticher, false));
        self.stations.push(Station::new(self.stations.len() as u32, Real2D { x: 0.0, y: 0.0 }, StationType::Cutter, false));
        self.stations.push(Station::new(self.stations.len() as u32, Real2D { x: 0.0, y: 0.0 }, StationType::Cutter, false));
        self.stations.push(Station::new(self.stations.len() as u32, Real2D { x: 0.0, y: 0.0 }, StationType::Finisher, false));
        self.stations.push(Station::new(self.stations.len() as u32, Real2D { x: 0.0, y: 0.0 }, StationType::Finisher, true));
        self.stations.push(Station::new(self.stations.len() as u32, Real2D { x: 0.0, y: 0.0 }, StationType::StorageRoom, false));
        self.stations.push(Station::new(self.stations.len() as u32, Real2D { x: 0.0, y: 0.0 }, StationType::RobotRoom, false));

        for station in self.stations {
            self.sticher_grid.set_object_location(station, station.get_location());
        }

        //spawn 3 robots
        for _ in 0..3 {
            self.robots.push(RefCell::from(Robot::new(Real2D { x: 0.0, y: 0.0 }, self)));
        }
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_state_mut(&mut self) -> &mut dyn State {
        self
    }

    fn as_state(&self) -> &dyn State {
        self
    }

    fn reset(&mut self) {
        self.standard_order_count = 0;
        self.luxury_order_count = 0;

        self.stations.clear();
        self.robots.clear();
    }

    fn update(&mut self, step: u64) {
        todo!()
    }
}

//----------------Robot----------------
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum CarriedProduct {
    Nothing,
    Bolts,
    Cuttings,
    Standard,
    Deluxe,
}


#[derive(Clone)]
pub struct Robot {
    max_charge: u32,
    charge: i32,
    location: Real2D,
    destination: Real2D,
    destination_type: StationType,
    order: CarriedProduct,
}

impl Location2D<Real2D> for Robot {
    fn get_location(self) -> Real2D {
        self.location
    }

    fn set_location(&mut self, location: Real2D) {
        self.location = location;
    }
}

impl Robot {
    pub fn new(location: Real2D, state: &dyn State) -> Robot {
        let default_max_charge = 350;
        let robot_factory = state.as_any().downcast_ref::<RobotFactory>().unwrap();
        let initial_destination = robot_factory.get_location_of_any_station_with_type(StationType::LoadingDock);
        Robot {
            max_charge: default_max_charge,
            charge: default_max_charge as i32,
            location,
            destination: initial_destination,
            destination_type: StationType::LoadingDock,
            order: CarriedProduct::Nothing,
        }
    }

    pub fn charge(&mut self, p0: i32, state: &RobotFactory) {
        self.charge = min(self.max_charge as i32, self.charge + p0);
        self.order = CarriedProduct::Nothing;

        if self.is_fully_charged() {
            if rand::thread_rng().gen_bool(0.5) {
                let destination: &Station = state.get_random_station();
                self.destination = destination.get_location();
                self.destination_type = destination.get_station_type();
            } else {
                self.destination = state.get_location_of_any_station_with_type(StationType::LoadingDock);
                self.destination_type = StationType::LoadingDock;
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
