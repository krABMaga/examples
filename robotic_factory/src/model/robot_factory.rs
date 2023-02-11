use std::any::Any;
use std::cell::RefCell;
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
use crate::model::robot::Robot;
use crate::model::stations::*;
use crate::model::stations::StationType::*;

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

    pub step: u64,
}

impl RobotFactory {
    pub fn new() -> RobotFactory {
        //values set here do not matter much, as `reset()` will be called immediately after anyways
        let mut robot_factory = RobotFactory {
            station_locations: vec![],
            robot_grid: Field2D::new(FACTORY_WIDTH, FACTORY_HEIGHT, 0.5, false),
            station_grid: Field2D::new(FACTORY_WIDTH, FACTORY_HEIGHT, 0.5, false),
            standard_order_count: 0,
            luxury_order_count: 0,
            step: 0,
        };
        robot_factory.reset();
        robot_factory
    }

    pub fn get_random_station_location(&self) -> StationLocation {
        self.station_locations
            .choose(&mut thread_rng())
            .unwrap()
            .clone()
    }

    pub fn get_locations_of(&self, station_type: StationType) -> Vec<StationLocation> {
        self.station_locations
            .iter()
            .filter(|station| station.station_type == station_type)
            .map(|station| station.clone())
            .collect()
    }

    pub fn get_random_station_location_with_type(
        &self,
        destination_type: StationType,
    ) -> StationLocation {
        let binding = self.get_locations_of(destination_type);
        binding.choose(&mut thread_rng()).unwrap().clone()
    }

    pub fn get_robots(&mut self) -> Vec<Robot> {
        let too_many_robots = self.robot_grid.get_neighbors_within_distance(
            Real2D { x: (FACTORY_WIDTH / 2.0), y: (FACTORY_HEIGHT / 2.0) },
            ((FACTORY_WIDTH / 2.0).powi(2) * (FACTORY_HEIGHT / 2.0).powi(2)).sqrt(),
        );


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
            let stations_at_location = self.station_grid.get_objects(station_location.location);
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
                let stations_at_location = self.station_grid.get_objects(station_location.location);
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

        let a_random_u32 =
            thread_rng().gen_range(0..(self.standard_order_count + self.luxury_order_count));
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

    fn create_station(
        &mut self,
        schedule: &mut Schedule,
        location: Real2D,
        station_type: StationType,
        is_deluxe_finisher: bool,
    ) {
        //adjust location for the non cartesian coordinate system used in krabmaga
        let location_adjusted = Real2D {
            x: location.x + 13.0,
            y: location.y + 13.0,
        };
        let station = Station::new(
            self.station_locations.len() as u32,
            location_adjusted,
            station_type,
            is_deluxe_finisher,
        );
        self.station_locations.push(station.to_station_location());
        self.station_grid
            .set_object_location(station, station.get_location());
        schedule.schedule_repeating(
            Box::new(station),
            0.0,
            RobotFactory::get_station_priority(station_type),
        );
    }

    fn get_station_priority(station_type: StationType) -> i32 {
        //scheduling order is
        //1. Cutters and Stitchers
        //2. Finishers
        //3. Robots
        //4. Robot Room
        //5. Loading Dock
        //6. Shift Control
        match station_type {
            Cutter | Sticher => 1,
            Finisher => 2,
            RobotRoom => 4,
            LoadingDock => 5,
            _ => 0,
        }
    }
}

impl State for RobotFactory {
    fn init(&mut self, schedule: &mut Schedule) {
        self.reset();

        //prepare the shift control
        schedule.schedule_repeating(Box::new(ShiftControl {}), 0.0, 6);

        //spawn 1 loading dock, 2 stichers, 2 cutters, 2 finishers, 1 storage room, 1 robot room
        //we input the locations given by the netlogo implementation and adjust them during creation
        self.create_station(schedule, Real2D { x: 10.0, y: -6.0 }, Cutter, false);
        self.create_station(schedule, Real2D { x: 10.0, y: -11.0 }, Cutter, false);
        self.create_station(schedule, Real2D { x: 4.0, y: 8.0 }, Sticher, false);
        self.create_station(schedule, Real2D { x: 4.0, y: 3.0 }, Sticher, false);
        self.create_station(schedule, Real2D { x: 0.0, y: -4.0 }, Finisher, false);
        self.create_station(schedule, Real2D { x: 2.0, y: -8.0 }, Finisher, true);
        self.create_station(schedule, Real2D { x: -6.0, y: 4.0 }, StorageRoom, false);
        self.create_station(schedule, Real2D { x: -11.0, y: -7.0 }, RobotRoom, false);
        self.create_station(schedule, Real2D { x: 13.0, y: 4.0 }, LoadingDock, false);

        //spawn robots
        for i in 0..ROBOT_COUNT {
            let robot = Robot::new(
                self.station_locations.len() + i,
                Real2D { x: 0.0, y: 0.0 },
                self,
            );
            self.robot_grid
                .set_object_location(robot, robot.get_location());
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

        addplot!(
            String::from("Products Ready for Van"),
            String::from("Time"),
            String::from("Count"),
            true
        );
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
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

        self.robot_grid = Field2D::new(FACTORY_WIDTH, FACTORY_HEIGHT, 0.1, false);
        self.station_grid = Field2D::new(FACTORY_WIDTH, FACTORY_HEIGHT, 0.5, false);
    }

    fn update(&mut self, step: u64) {
        log!(LogType::Info, format!("Step #{}", step), true);

        self.step = step;

        self.station_grid.lazy_update();
        self.robot_grid.lazy_update();
    }

    fn before_step(&mut self, schedule: &mut Schedule) {
        #[cfg(debug_assertions)]{
            let bags = self.robot_grid.bags.clone();
            let mut bag1: &RefCell<Vec<Vec<Robot>>> = bags.get(0).unwrap();
            let mut bag2: &RefCell<Vec<Vec<Robot>>> = bags.get(1).unwrap();

            let bag1_borrow = bag1.borrow();
            let bag2_borrow = bag2.borrow();
            let bag1_filtered = bag1_borrow.iter()
                .filter(|bag| bag.len() > 0)
                .map(|bag| bag.iter().collect::<Vec<&Robot>>())
                .collect::<Vec<Vec<&Robot>>>();

            let bag2_filtered = bag2_borrow.iter()
                .filter(|bag| bag.len() > 0)
                .map(|bag| bag.iter().collect::<Vec<&Robot>>())
                .collect::<Vec<Vec<&Robot>>>();


            let count1 = bag1_filtered.iter().map(|bag| bag.len()).sum::<usize>();
            let count2 = bag2_filtered.iter().map(|bag| bag.len()).sum::<usize>();


            assert!(count1 == ROBOT_COUNT || count2 == ROBOT_COUNT);
        }
    }


    fn after_step(&mut self, _schedule: &mut Schedule) {
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

        plot!(
            String::from("Outstanding Orders"),
            String::from("Total Orders"),
            self.step as f64,
            self.luxury_order_count as f64 + self.standard_order_count as f64
        );

        //filter out robot rooms and Storage rooms
        let types_to_ignore: HashSet<StationType> =
            HashSet::from_iter(vec![RobotRoom, StorageRoom].into_iter());
        let mut supply_counts = HashMap::new();
        self.get_stations()
            .iter()
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
            log!(
                LogType::Info,
                format!("{:?}s supply: {}", station_type, supply),
                true
            );
            plot!(
                String::from("Machine Supply"),
                format!("{:?}s", station_type),
                self.step as f64,
                supply as f64
            );
        }

        //Report Storage Rooms
        let storage_rooms = self.get_stations_of_type(StorageRoom);
        let sum = storage_rooms.iter()
            .fold(0, |sum, station| {
                sum + station.material_management.get_supply_count()
            });
        for station in storage_rooms {
            log!(
                LogType::Info,
                format!(
                    "Storage Room #{} has {} products",
                    station.get_id(),
                    station.material_management.get_supply_count()
                ),
                true
            );
            plot!(
                String::from("Products Ready for Van"),
                format!("Storage Room ({})", station.get_id()),
                self.step as f64,
                station.material_management.get_supply_count() as f64
            );
        }

        log!(
            LogType::Info,
            format!("Total products ready for van: {}", sum),
            true
        );
        plot!(
            String::from("Products Ready for Van"),
            String::from("Total"),
            self.step as f64,
            sum as f64
        );
    }
}

#[derive(Clone, Copy, Debug)]
struct ShiftControl {}

impl Agent for ShiftControl {
    fn step(&mut self, state: &mut dyn State) {
        if thread_rng().gen_bool(0.02) {
            let factory = state.as_any_mut().downcast_mut::<RobotFactory>().unwrap();

            let mut robots = factory.get_robots();
            let robots_to_reschedule = robots.iter_mut().choose_multiple(&mut thread_rng(), 3);

            let ids: HashSet<usize> = robots_to_reschedule.iter().map(|robot| robot.get_id()).collect();

            let robot_future_state: HashSet<Robot> = robots_to_reschedule.iter()
                .flat_map(|robot| factory.robot_grid.get_objects_unbuffered(robot.get_next_location()))
                .filter(|robot| ids.contains(&robot.get_id()))
                .fold(HashSet::new(), |mut set, robot| {
                    set.insert(robot);
                    set
                });


            for mut robot in robot_future_state {
                let old_location = robot.clone().get_location();

                let loading_dock = factory.get_random_station_location_with_type(LoadingDock);
                robot.change_destination(loading_dock); //this will update the next location

                log!(
                    LogType::Info,
                    format!("Robot {} starts a new shift.", robot.get_id()),
                    true
                );

                factory.robot_grid.remove_object_location(robot, old_location);
                factory.robot_grid.set_object_location(robot, robot.get_location());
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

        assert_eq!(
            factory.standard_order_count + factory.luxury_order_count,
            total_orders
        );

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
        println!("Stations: {}", stations.len());
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
