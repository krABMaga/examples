use std::any::Any;
use std::cell::RefCell;
use std::cmp::min;
use std::collections::HashMap;
use std::hash::Hash;
use std::panic::Location;

use krabmaga::{rand, Rng};
use krabmaga::engine::agent::Agent;
use krabmaga::engine::fields::field::Field;
use krabmaga::engine::fields::field_2d::{Field2D, Location2D};
use krabmaga::engine::location::Real2D;
use krabmaga::engine::schedule::Schedule;
use krabmaga::engine::state::State;
use krabmaga::rand::seq::{IteratorRandom, SliceRandom};

use crate::model::robot::{CarriedProduct, Robot};
use crate::model::stations::*;

pub struct RobotFactory {
    stations: Vec<Station>, //copy used for finding the locations of random stations

    pub robot_grid: Field2D<Robot>,
    pub station_grid: Field2D<Station>,

    pub standard_order_count: u32,
    pub luxury_order_count: u32,
}

impl RobotFactory {
    pub fn new() -> RobotFactory {
        RobotFactory {
            stations: vec![],
            robot_grid: Field2D::new(100.0, 100.0, 0.1, false),
            station_grid: Field2D::new(100.0, 100.0, 0.1, false),
            standard_order_count: 0,
            luxury_order_count: 0,
        }
    }

    pub fn get_random_station(&self) -> (StationType, Real2D) {
        let station = self.stations.choose(&mut rand::thread_rng()).unwrap();
        return (station.get_station_type(), station.get_location());
    }

    pub fn get_stations_of_type(&self, station_type: StationType) -> Vec<&Station> {
        self.stations.iter().filter(|x| { x.get_station_type() == station_type }).collect()
    }

    pub fn get_random_station_with_type(&self, destination_type: StationType) -> (StationType, Real2D) {
        let binding = self.get_stations_of_type(destination_type);
        let station = binding.choose(&mut rand::thread_rng()).unwrap();
        return (station.get_station_type(), station.get_location());
    }

    pub fn get_robots(&mut self) -> Vec<Robot> {
        self.robot_grid.get_neighbors_within_relax_distance(Real2D { x: 0.0, y: 0.0 }, 100.0)
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

    pub fn do_initial_scheduling(&mut self, schedule: &mut Schedule) {
        //scheduling order is
        //1. Cutters and Stitchers
        //2. Finishers
        //3. Robots
        //4. Robot Room
        //5. Loading Dock
        //6. Shift Control

        fn get_station_priority(station_type: StationType) -> i32 {
            match station_type {
                StationType::Cutter => 1,
                StationType::Sticher => 1,
                StationType::Finisher => 2,
                StationType::RobotRoom => 4,
                StationType::LoadingDock => 5,
                _ => 0
            }
        }

        for station in self.stations.iter() {
            let station_type = station.get_station_type();
            let priority = get_station_priority(station_type);
            schedule.schedule_repeating(Box::new(*station), 0.0, priority);
        }

        schedule.schedule_repeating(Box::new(ShiftControl {}), 0.0, 6);
    }
}


impl State for RobotFactory {
    fn init(&mut self, schedule: &mut Schedule) {
        let mut stations: Vec<Station> = Vec::new();

        //spawn 1 loading dock, 2 stichers, 2 cutters, 2 finishers, 1 storage room, 1 robot room
        stations.push(Station::new(stations.len() as u32, Real2D { x: 0.0, y: 0.0 }, StationType::LoadingDock, false));
        stations.push(Station::new(stations.len() as u32, Real2D { x: 0.0, y: 0.0 }, StationType::Sticher, false));
        stations.push(Station::new(stations.len() as u32, Real2D { x: 0.0, y: 0.0 }, StationType::Sticher, false));
        stations.push(Station::new(stations.len() as u32, Real2D { x: 0.0, y: 0.0 }, StationType::Cutter, false));
        stations.push(Station::new(stations.len() as u32, Real2D { x: 0.0, y: 0.0 }, StationType::Cutter, false));
        stations.push(Station::new(stations.len() as u32, Real2D { x: 0.0, y: 0.0 }, StationType::Finisher, false));
        stations.push(Station::new(stations.len() as u32, Real2D { x: 0.0, y: 0.0 }, StationType::Finisher, true));
        stations.push(Station::new(stations.len() as u32, Real2D { x: 0.0, y: 0.0 }, StationType::StorageRoom, false));
        stations.push(Station::new(stations.len() as u32, Real2D { x: 0.0, y: 0.0 }, StationType::RobotRoom, false));

        let station_count = stations.len();

        for station in stations {
            self.stations.push(station);
            self.station_grid.set_object_location(station, station.get_location());
        }

        //spawn 3 robots
        for i in 0..3 {
            let robot = Robot::new((station_count + i) as u32, Real2D { x: 0.0, y: 0.0 }, self);
            self.robot_grid.set_object_location(robot, robot.get_location());
            schedule.schedule_repeating(Box::new(robot), 0.0, 3);
        }

        self.do_initial_scheduling(schedule);
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

        self.station_grid = Field2D::new(100.0, 100.0, 0.01, false);
        self.station_grid = Field2D::new(100.0, 100.0, 0.01, false);
    }

    fn update(&mut self, step: u64) {
        self.station_grid.lazy_update();
        self.robot_grid.lazy_update();
    }
}

#[derive(Clone, Copy, Debug)]
struct ShiftControl {}

impl Agent for ShiftControl {
    fn step(&mut self, state: &mut dyn State) {
        if rand::thread_rng().gen_bool(0.02) {
            let mut factory = state.as_any_mut().downcast_mut::<RobotFactory>().unwrap();

            let mut robots = factory.get_robots();
            let mut robots_to_reschedule = robots.iter_mut().choose_multiple(&mut rand::thread_rng(), 3);

            for mut robot in robots_to_reschedule {
                let loading_dock = factory.get_random_station_with_type(StationType::LoadingDock);
                robot.change_destination(loading_dock);
            }
        }
    }
}
