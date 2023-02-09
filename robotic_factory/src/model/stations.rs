use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};

use krabmaga::{log, rand, Rng};
use krabmaga::engine::agent::Agent;
use krabmaga::engine::fields::field_2d::Location2D;
use krabmaga::engine::location::Real2D;
use krabmaga::engine::schedule::ScheduleOptions;
use krabmaga::engine::state::State;

use crate::{
    CHARGE_PER_STEP, DELUXE_FINISHER_CYCLES, JUST_IN_TIME_CHARGE, ORDER_GENEREATION_CHANCE,
    STANDARD_FINISHER_CYCLES,
};
use crate::model::robot::{CarriedProduct, Robot};
use crate::model::robot_factory::{RobotFactory, StationLocation};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum StationType {
    Sticher,
    Cutter,
    Finisher,
    LoadingDock,
    StorageRoom,
    RobotRoom,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct FinisherInformation {
    process_time: u32,
    progress: u32,
    is_deluxe: bool,
    is_processing: bool,
}

#[derive(Clone, Copy, Eq)]
pub struct Station {
    id: u32,
    location: Real2D,
    pub material_management: MaterialManagement,
    station_type: StationType,

    finisher_information: FinisherInformation,
}

impl Station {
    pub fn new(
        id: u32,
        location: Real2D,
        station_type: StationType,
        mut is_delux_finisher: bool,
    ) -> Station {
        if station_type != StationType::Finisher {
            is_delux_finisher = false;
        }

        let mut station = Station {
            id,
            location,
            material_management: MaterialManagement::default(),
            station_type,
            finisher_information: FinisherInformation {
                process_time: if is_delux_finisher {
                    DELUXE_FINISHER_CYCLES
                } else {
                    STANDARD_FINISHER_CYCLES
                },
                progress: 0,
                is_deluxe: is_delux_finisher,
                is_processing: false,
            },
        };
        if station_type == StationType::LoadingDock {
            station.material_management.add_products(10);
        }
        station
    }

    pub fn get_station_type(&self) -> StationType {
        self.station_type
    }

    pub fn is_deluxe_finisher(&self) -> bool {
        self.finisher_information.is_deluxe
    }

    pub fn get_id(&self) -> u32 { self.id }

    pub fn to_station_location(&self) -> StationLocation {
        StationLocation {
            station_type: self.station_type,
            location: self.location,
        }
    }

    pub fn try_convert_one_supply(&mut self) {
        if self.material_management.has_supply() {
            self.material_management.decrement_supply();
            self.material_management.increment_products();
        }
    }

    pub fn take_product(&mut self, robot_factory: &mut RobotFactory) -> CarriedProduct {
        if !self.has_product_available() {
            panic!("No product available");
        }

        self.material_management.decrement_products();
        match self.station_type {
            StationType::LoadingDock => CarriedProduct::Bolts,
            StationType::Cutter => CarriedProduct::Cuttings,
            StationType::Sticher => {
                let order: Option<bool> = robot_factory.retrieve_order();
                //the implementation defaults to creating a deluxe garment if no order is available
                if order.is_none() || (order.is_some() && order.unwrap()) {
                    CarriedProduct::StichedDeluxe
                } else {
                    CarriedProduct::StichedStandard
                }
            }
            StationType::Finisher => {
                if self.finisher_information.is_deluxe {
                    CarriedProduct::FinishedDeluxe
                } else {
                    CarriedProduct::FinishedStandard
                }
            }
            _ => panic!("Wrong station type"),
        }
    }

    pub fn has_product_available(&self) -> bool {
        self.material_management.get_products_count() > 0
    }
}

impl Location2D<Real2D> for Station {
    fn get_location(self) -> Real2D {
        self.location
    }

    fn set_location(&mut self, loc: Real2D) {
        self.location = loc;
    }
}

impl Hash for Station {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for Station {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl fmt::Display for Station {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}({})", self.station_type, self.id)
    }
}

impl Agent for Station {
    fn step(&mut self, state: &mut dyn State) {
        let factory = state.as_any_mut().downcast_mut::<RobotFactory>().unwrap();

        match self.station_type {
            StationType::Sticher | StationType::Cutter => {
                //make-garments (except for finish call)
                self.try_convert_one_supply();
            }
            StationType::Finisher => {
                // finish

                if self.finisher_information.is_processing {
                    self.finisher_information.progress += 1;
                }

                if self.material_management.has_supply() && !self.finisher_information.is_processing
                {
                    self.finisher_information.progress += 1;
                    self.material_management.decrement_supply();
                    self.finisher_information.is_processing = true;
                }

                if self.finisher_information.progress >= self.finisher_information.process_time {
                    self.finisher_information.progress = 0;
                    self.material_management.increment_products();
                    self.finisher_information.is_processing = false;
                }
            }
            StationType::LoadingDock => {
                // deliver-more-material-sheets
                if rand::thread_rng().gen_bool(ORDER_GENEREATION_CHANCE)
                    && self.material_management.get_products_count() < 3
                {
                    self.material_management
                        .add_products(rand::thread_rng().gen_range(0..10));
                }

                if rand::thread_rng().gen_bool(0.03) {
                    for _ in 0..rand::thread_rng().gen_range(0..3) {
                        factory.bump_required_orders(rand::thread_rng().gen_bool(0.5));
                    }
                }
            }
            StationType::StorageRoom => {}
            StationType::RobotRoom => {
                //aka charging station
                //recharge

                let neighbors = factory
                    .robot_grid
                    .get_neighbors_within_distance(self.location, 1.4);

                if neighbors.len() != 0 {

                    let mut future_states: HashMap<usize, Robot> = HashMap::new();
                    for robot in neighbors.iter() {
                        factory
                            .robot_grid
                            .get_objects_unbuffered(robot.get_next_location())
                            .iter()
                            .for_each(|other| {
                                future_states.insert(other.get_id(), *other);
                            });
                    }

                    for robot in future_states.values_mut() {
                        robot.charge(CHARGE_PER_STEP, factory);
                    }

                    let loading_docks = factory.get_stations_of_type(StationType::LoadingDock);
                    if loading_docks
                        .iter()
                        .any(|dock| dock.material_management.has_products())
                    {
                        for robot in future_states.values_mut() {
                            if robot.charge >= JUST_IN_TIME_CHARGE {
                                log!(
                                LogType::Info,
                                format!("Just-in-time charging for Robot {}", robot),
                                true
                            );
                                robot.change_destination(
                                    factory.get_random_station_location_with_type(
                                        StationType::LoadingDock,
                                    ),
                                );
                            }
                        }
                    }

                    for robot in future_states.values() {
                        factory
                            .robot_grid
                            .remove_object_location(*robot, robot.get_location());
                        factory
                            .robot_grid
                            .set_object_location(*robot, robot.get_location());
                    }
                }
            }
        }

        factory
            .station_grid
            .set_object_location(*self, self.location);
    }

    fn before_step(
        &mut self,
        _state: &mut dyn State,
    ) -> Option<Vec<(Box<dyn Agent>, ScheduleOptions)>> {
        //load self state from factory state
        let factory = _state.as_any_mut().downcast_mut::<RobotFactory>().unwrap();

        let mut stations = factory.station_grid.get_objects_unbuffered(self.location);
        for station in stations.iter() {
            if station.id == self.id {
                self.material_management = station.material_management;
                self.finisher_information = station.finisher_information;
                return None; //if we find an updated state of the station, we update ourselves
            }
        }

        stations = factory.station_grid.get_objects(self.location);
        for station in stations.iter() {
            if station.id == self.id {
                self.material_management = station.material_management;
                self.finisher_information = station.finisher_information;
                break;
            }
        }

        None
    }
}

//----------------OrderManagement----------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct MaterialManagement {
    supply: u32,
    products: u32,
}

impl MaterialManagement {
    pub fn has_supply(&self) -> bool {
        self.supply > 0
    }
    pub fn get_supply_count(&self) -> u32 {
        self.supply
    }
    pub fn get_products_count(&self) -> u32 {
        self.products
    }
    pub fn has_products(&self) -> bool {
        self.products > 0
    }

    pub fn increment_supply(&mut self) {
        self.supply += 1;
    }
    pub fn decrement_supply(&mut self) {
        self.supply -= 1;
    }
    pub fn increment_products(&mut self) {
        self.products += 1;
    }
    pub fn decrement_products(&mut self) {
        self.products -= 1;
    }

    pub fn add_supply(&mut self, amount: u32) {
        self.supply += amount;
    }

    pub fn add_products(&mut self, amount: u32) {
        self.products += amount;
    }
}

#[cfg(test)]
mod tests {
    use krabmaga::engine::schedule::Schedule;

    use crate::{INITIAL_LOADING_DOCK_PRODUCTS, JUST_IN_TIME_CHARGE};
    use crate::model::robot_factory::StationLocation;

    use super::*;

    #[test]
    fn stitcher_converts_supply_to_products() {
        let mut stitcher = Station::new(0, Real2D { x: 0.0, y: 0.0 }, StationType::Sticher, false);
        station_converts_supply_to_products(stitcher)
    }

    #[test]
    fn cutter_converts_supply_to_products() {
        let mut cutter = Station::new(0, Real2D { x: 0.0, y: 0.0 }, StationType::Cutter, false);
        station_converts_supply_to_products(cutter);
    }

    fn station_converts_supply_to_products(mut station: Station) {
        let mut factory = RobotFactory::new();

        assert_eq!(station.material_management.get_supply_count(), 0);
        assert_eq!(station.material_management.get_products_count(), 0);

        station.step(factory.as_state_mut());

        assert_eq!(station.material_management.get_supply_count(), 0);
        assert_eq!(station.material_management.get_products_count(), 0);

        station.material_management.add_supply(3);

        station.step(factory.as_state_mut());

        assert_eq!(station.material_management.get_products_count(), 1);
        assert_eq!(station.material_management.get_supply_count(), 2);
    }

    #[test]
    fn standard_finisher_converts_supply_to_products_after_steps() {
        let mut finisher = Station::new(0, Real2D { x: 0.0, y: 0.0 }, StationType::Finisher, false);
        test_finisher(finisher, STANDARD_FINISHER_CYCLES);
    }

    #[test]
    fn deluxe_finisher_converts_supply_to_products_after_steps() {
        let mut finisher = Station::new(0, Real2D { x: 0.0, y: 0.0 }, StationType::Finisher, true);
        test_finisher(finisher, DELUXE_FINISHER_CYCLES);
    }

    fn test_finisher(mut finisher_station: Station, expected_finishing_cycles: u32) {
        assert_eq!(finisher_station.get_station_type(), StationType::Finisher);

        let mut factory = RobotFactory::new();

        finisher_station.material_management.add_supply(3);
        assert_eq!(finisher_station.material_management.get_products_count(), 0);

        for _ in 0..expected_finishing_cycles - 1 {
            finisher_station.step(factory.as_state_mut());
        }
        assert_eq!(finisher_station.material_management.get_products_count(), 0);

        finisher_station.step(factory.as_state_mut());
        assert_eq!(finisher_station.material_management.get_products_count(), 1);

        for _ in 0..expected_finishing_cycles - 1 {
            finisher_station.step(factory.as_state_mut());
        }
        assert_eq!(finisher_station.material_management.get_products_count(), 1);

        finisher_station.step(factory.as_state_mut());
        assert_eq!(finisher_station.material_management.get_products_count(), 2);
    }

    #[test]
    fn loading_dock_creates_supply() {
        let mut factory = RobotFactory::new();
        let mut loading_dock = Station::new(
            0,
            Real2D { x: 0.0, y: 0.0 },
            StationType::LoadingDock,
            false,
        );

        for _ in 0..1000 {
            loading_dock.step(factory.as_state_mut());
        }

        println!(
            "Products available: {}",
            loading_dock.material_management.get_products_count()
        );
        assert!(loading_dock.material_management.get_products_count() > 0);
    }

    #[test]
    fn does_create_supply_during_simulation() {
        let mut factory = RobotFactory::new();
        let mut scheduler = Schedule::new();

        factory.init(&mut scheduler);
        factory.update(0);

        let robots = factory.get_robots();
        for robot in robots.iter() {
            scheduler.dequeue(Box::new(*robot), robot.get_id() as u32);
        }

        let mut loading_docks = factory.get_stations_of_type(StationType::LoadingDock);

        for dock in loading_docks.iter() {
            assert_eq!(dock.material_management.get_products_count(), 0);
        }

        for step in 0..1000 {
            scheduler.step(&mut factory);
            loading_docks = factory.get_stations_of_type(StationType::LoadingDock);
            if loading_docks
                .iter()
                .all(|dock| dock.material_management.get_products_count() > 0)
            {
                println!("Took {} steps to create supply", step);
                break;
            }
        }

        for dock in loading_docks.iter() {
            println!(
                "Products available: {} for loading dock {}",
                dock.material_management.get_products_count(),
                dock.id
            );
            assert!(dock.material_management.get_products_count() > 0);
        }
    }

    #[test]
    fn robot_room_charges_robots() {
        //given
        let mut factory = RobotFactory::new();
        let mut robot_room =
            Station::new(0, Real2D { x: 0.0, y: 0.0 }, StationType::RobotRoom, false);
        let mut loading_dock = Station::new(
            0,
            Real2D { x: 0.0, y: 0.0 },
            StationType::LoadingDock,
            false,
        );

        factory
            .station_grid
            .set_object_location(robot_room, robot_room.location);
        factory.station_locations.push(StationLocation {
            station_type: StationType::RobotRoom,
            location: robot_room.location,
        });
        factory
            .station_grid
            .set_object_location(loading_dock, loading_dock.location);
        factory.station_locations.push(StationLocation {
            station_type: StationType::LoadingDock,
            location: loading_dock.location,
        });

        let mut robot = Robot::new(0, Real2D { x: 0.0, y: 0.0 }, &mut factory);
        robot.charge = 0;
        robot.max_charge = CHARGE_PER_STEP.saturating_add(100) as u32;
        factory
            .robot_grid
            .set_object_location(robot, robot.get_location());

        factory.update(0);

        //when
        robot_room.step(factory.as_state_mut());
        factory.update(1);

        robot = *factory
            .robot_grid
            .get_objects(robot.get_location())
            .first()
            .unwrap();

        //then
        assert_eq!(robot.charge, CHARGE_PER_STEP);
    }

    #[test]
    fn loading_dock_starts_with_initial_products() {
        let mut loading_dock = Station::new(
            0,
            Real2D { x: 0.0, y: 0.0 },
            StationType::LoadingDock,
            false,
        );

        assert_eq!(
            loading_dock.material_management.get_products_count(),
            INITIAL_LOADING_DOCK_PRODUCTS
        );
    }
}
