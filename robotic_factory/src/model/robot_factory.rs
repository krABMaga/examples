use std::any::Any;
use std::cmp::{max, min};
use std::collections::HashSet;

use krabmaga::*;
use krabmaga::engine::agent::Agent;
use krabmaga::engine::fields::field::Field;
use krabmaga::engine::fields::field_2d::{Field2D, Location2D};
use krabmaga::engine::location::Real2D;
use krabmaga::engine::schedule::Schedule;
use krabmaga::engine::state::State;
use krabmaga::rand::seq::{IteratorRandom, SliceRandom};

use StationType::{RobotRoom, StorageRoom};

use crate::{FACTORY_HEIGHT, FACTORY_WIDTH, ROBOT_COUNT};
use crate::model::robot::{CarriedProduct, Robot};
use crate::model::robot_factory;
use crate::model::stations::*;
use crate::model::stations::StationType::LoadingDock;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct StationLocation {
    pub station_type: StationType,
    pub location: Real2D,
}

pub struct RobotFactory {
    pub station_locations: Vec<StationLocation>, //holds the location information of all stations

    pub robot_grid: Field2D<Robot>,
    pub station_grid: Field2D<Station>,

    standard_order_count: u32,
    luxury_order_count: u32,

    step: u64,
}

impl RobotFactory {
    pub fn new() -> RobotFactory {
        //values set here do not matter much, as `reset()` will be called immediately after anyways
        let mut robot_factory = RobotFactory {
            station_locations: vec![],
            robot_grid: Field2D::new(FACTORY_WIDTH, FACTORY_HEIGHT, 0.5, false),
            station_grid: Field2D::new(FACTORY_WIDTH, FACTORY_HEIGHT, 1.0, false),
            standard_order_count: 0,
            luxury_order_count: 0,
            step: 0,
        };
        robot_factory.reset();
        robot_factory
    }

    pub fn get_random_station_location(&self) -> StationLocation {
        self.station_locations.choose(&mut rand::thread_rng()).unwrap().clone()
    }

    pub fn get_locations_of(&self, station_type: StationType) -> Vec<StationLocation> {
        self.station_locations.iter().filter(|station| station.station_type == station_type).map(|station| station.clone()).collect()
    }

    pub fn get_random_station_location_with_type(&self, destination_type: StationType) -> StationLocation {
        let binding = self.get_locations_of(destination_type);
        binding.choose(&mut rand::thread_rng()).unwrap().clone()
    }

    pub fn get_robots(&mut self) -> Vec<Robot> {
        let mut too_many_robots = self.robot_grid.get_neighbors_within_distance(Real2D { x: 0.0, y: 0.0 }, f32::max(FACTORY_WIDTH, FACTORY_HEIGHT));

        let mut found = HashSet::new();
        let mut reduced_robots = vec![];

        //find unique robots  by id (alternatively, one could use HashSet::from_iter, which is probably slower)
        for robot in too_many_robots {
            if found.contains(&robot.get_id()) {
                continue;
            } else {
                found.insert(robot.get_id());
                reduced_robots.push(robot);
                if reduced_robots.len() == ROBOT_COUNT {
                    break;
                }
            }
        }
        reduced_robots.into_iter().collect()
    }

    pub fn get_stations(&self) -> HashSet<Station> {
        let mut stations = HashSet::new();
        for station_location in &self.station_locations {
            let mut stations_at_location = self.station_grid.get_objects(station_location.location);
            for s in stations_at_location {
                stations.insert(s);
            }
        }
        stations
    }

    pub fn get_stations_of_type(&self, station_type: StationType) -> HashSet<Station> {
        let mut stations = HashSet::new();
        for station_location in &self.station_locations {
            if station_location.station_type == station_type {
                let mut stations_at_location = self.station_grid.get_objects(station_location.location);
                for s in stations_at_location {
                    stations.insert(s);
                }
            }
        }
        stations
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
        self.reset();

        //scheduling order is
        //1. Cutters and Stitchers
        //2. Finishers
        //3. Robots
        //4. Robot Room
        //5. Loading Dock
        //6. Shift Control

        fn get_station_priority(station_type: StationType) -> i32 {
            match station_type {
                StationType::Cutter | StationType::Sticher => 1,
                StationType::Finisher => 2,
                StationType::RobotRoom => 4,
                StationType::LoadingDock => 5,
                _ => 0
            }
        }

        //prepare the shift control
        schedule.schedule_repeating(Box::new(ShiftControl {}), 0.0, 6);

        //spawn 1 loading dock, 2 stichers, 2 cutters, 2 finishers, 1 storage room, 1 robot room
        let mut stations: Vec<Station> = Vec::new();
        stations.push(Station::new(stations.len() as u32, Real2D { x: 1.0, y: 0.0 }, StationType::LoadingDock, false));
        stations.push(Station::new(stations.len() as u32, Real2D { x: 2.0, y: 0.0 }, StationType::Sticher, false));
        stations.push(Station::new(stations.len() as u32, Real2D { x: 3.0, y: 0.0 }, StationType::Sticher, false));
        stations.push(Station::new(stations.len() as u32, Real2D { x: 4.0, y: 0.0 }, StationType::Cutter, false));
        stations.push(Station::new(stations.len() as u32, Real2D { x: 5.0, y: 0.0 }, StationType::Cutter, false));
        stations.push(Station::new(stations.len() as u32, Real2D { x: 6.0, y: 0.0 }, StationType::Finisher, false));
        stations.push(Station::new(stations.len() as u32, Real2D { x: 7.0, y: 0.0 }, StationType::Finisher, true));
        stations.push(Station::new(stations.len() as u32, Real2D { x: 8.0, y: 0.0 }, StorageRoom, false));
        stations.push(Station::new(stations.len() as u32, Real2D { x: 9.0, y: 0.0 }, RobotRoom, false));


        //spawn stations
        let station_count = stations.len(); //prepare for later
        for station in stations {
            self.station_locations.push(StationLocation { location: station.get_location(), station_type: station.get_station_type() });
            self.station_grid.set_object_location(station, station.get_location());
            schedule.schedule_repeating(Box::new(station), 0.0, get_station_priority(station.get_station_type()));
        }

        //spawn robots
        for i in 0..ROBOT_COUNT {
            let robot = Robot::new((station_count + i) as u32, Real2D { x: 0.0, y: 0.0 }, self);
            self.robot_grid.set_object_location(robot, robot.get_location());
            schedule.schedule_repeating(Box::new(robot), 0.0, 3);
        }

        addplot!(
            String::from("Outstanding Orders"),
            String::from("Time"),
            String::from("Count"),
            true
        );

        addplot!(
            String::from("Robots' Energy"),
            String::from("Time"),
            String::from("Energy (%)"),
            true
        );

        addplot!(
            String::from("Machine Supply"),
            String::from("Time"),
            String::from("Supply"),
            true
        );
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

        self.station_locations.clear();

        self.robot_grid = Field2D::new(FACTORY_WIDTH, FACTORY_HEIGHT, 0.5, false);
        self.station_grid = Field2D::new(FACTORY_WIDTH, FACTORY_HEIGHT, 1.0, false);
    }

    fn update(&mut self, step: u64) {
        self.step = step;

        self.station_grid.lazy_update();
        self.robot_grid.lazy_update();
    }

    fn after_step(&mut self, schedule: &mut Schedule) {
        plot!(
            String::from("Outstanding Orders"),
            String::from("Standard Orders"),
            self.step as f64,
            self.standard_order_count as f64
        );

        plot!(
            String::from("Outstanding Orders"),
            String::from("Deluxe Orders"),
            self.step as f64,
            self.luxury_order_count as f64
        );


        let robots = self.get_robots();
        for robot in robots {
            plot!(
                String::from("Robots' Energy"),
                format!("Robot {}", robot.get_id()),
                self.step as f64,
                (robot.charge as f64 / robot.max_charge as f64) * 100.0
            );
        }

        //filter out robot rooms and Storage rooms
        let types_to_ignore: HashSet<StationType> = HashSet::from_iter(
            vec![RobotRoom, StorageRoom].into_iter());
        let mut supply_counts = HashMap::new();
        self.get_stations().iter()
            .filter(|station| !types_to_ignore.contains(&station.get_station_type()))
            .for_each(|station| {
                let count = supply_counts.entry(station.get_station_type()).or_insert(0);

                *count += if station.get_station_type() == LoadingDock {
                    station.material_management.get_products_count()
                } else {
                    station.material_management.get_supply_count()
                };
            });

        for (station_type, supply) in supply_counts {
            log!(LogType::Info, format!("{:?}s supply: {}", station_type, supply));
            plot!(
                String::from("Machine Supply"),
                format!("{:?}s", station_type),
                self.step as f64,
                supply as f64
            );
        }
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
                let loading_dock = factory.get_random_station_location_with_type(StationType::LoadingDock);
                robot.change_destination(loading_dock);
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use krabmaga::thread_rng;

    use super::*;

    #[test]
    fn test_get_robots_count() {
        let mut factory = RobotFactory::new();
        let mut schedule = Schedule::new();

        factory.init(&mut schedule);
        factory.update(0);

        let mut robots = factory.get_robots();
        assert_eq!(robots.len(), ROBOT_COUNT);
    }

    #[test]
    fn test_get_stations_count() {
        let mut factory = RobotFactory::new();
        let mut schedule = Schedule::new();

        factory.init(&mut schedule);
        factory.update(0);

        let mut stations = factory.get_stations();
        assert_eq!(factory.station_locations.len(), stations.len());
    }

    #[test]
    fn places_and_retrieves_standard_order() {
        let mut factory = RobotFactory::new();

        factory.bump_required_orders(false);

        assert_eq!(factory.standard_order_count, 1);
        assert_eq!(factory.luxury_order_count, 0);

        let order = factory.retrieve_order();

        assert_eq!(order.is_some(), true);
        assert_eq!(order.unwrap(), false);
        assert_eq!(factory.standard_order_count, 0);
        assert_eq!(factory.luxury_order_count, 0);
    }

    #[test]
    fn places_and_retrieves_luxury_order() {
        let mut factory = RobotFactory::new();

        factory.bump_required_orders(true);

        assert_eq!(factory.standard_order_count, 0);
        assert_eq!(factory.luxury_order_count, 1);

        let order = factory.retrieve_order();

        assert_eq!(order.is_some(), true);
        assert_eq!(order.unwrap(), true);
        assert_eq!(factory.standard_order_count, 0);
        assert_eq!(factory.luxury_order_count, 0);
    }

    #[test]
    fn correctly_retrieves_order_type() {
        let mut factory = RobotFactory::new();

        let total_orders = 100;
        let deluxe_chance = 0.3;

        for _ in 0..total_orders {
            factory.bump_required_orders(thread_rng().gen_bool(deluxe_chance))
        }

        assert_eq!(factory.standard_order_count + factory.luxury_order_count, total_orders);

        let mut luxury_count = 0;
        let mut standard_count = 0;

        for _ in 0..5000000 {
            let order = factory.retrieve_order();
            if order.is_some() {
                if order.unwrap() {
                    luxury_count += 1;
                } else {
                    standard_count += 1;
                }
                factory.bump_required_orders(order.unwrap());
            }
        }

        let luxury_ratio = luxury_count as f64 / (luxury_count + standard_count) as f64;
        let standard_ratio = standard_count as f64 / (luxury_count + standard_count) as f64;

        println!("Luxury ratio: {}", luxury_ratio);
        println!("Standard ratio: {}", standard_ratio);

        assert!((luxury_ratio - deluxe_chance).abs() < 0.1);
        assert!((standard_ratio - (1.0 - deluxe_chance)).abs() < 0.1);
    }

    #[test]
    fn test_get_station_after_step() {
        let mut factory = RobotFactory::new();
        let mut schedule = Schedule::new();

        factory.init(&mut schedule);

        schedule.step(&mut factory);

        let mut stations = factory.get_stations();
        assert_eq!(factory.station_locations.len(), stations.len());
    }

    #[test]
    fn test_get_robots_after_step() {
        let mut factory = RobotFactory::new();
        let mut schedule = Schedule::new();

        factory.init(&mut schedule);

        schedule.step(&mut factory);

        let mut robots = factory.get_robots();
        assert_eq!(robots.len(), ROBOT_COUNT);
    }
}
