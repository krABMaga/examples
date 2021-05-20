use crate::model::ant::Ant;
use crate::model::ants_grid::AntsGrid;
use crate::model::obstacles_grid::ObstaclesGrid;
use crate::model::sites_grid::SitesGrid;
use crate::model::static_objects::StaticObjectType;
use crate::model::to_food_grid::ToFoodGrid;
use crate::model::to_home_grid::ToHomeGrid;
use rust_ab::engine::location::Int2D;
use std::sync::RwLock;

/// The global simulation state. This holds the various grids used for movement, exposing setter methods
/// so that the state itself will worry about ownership rules by mutating its own fields.
pub struct State {
    pub ants_grid: AntsGrid,
    pub obstacles_grid: ObstaclesGrid,
    pub sites_grid: SitesGrid,
    pub to_food_grid: ToFoodGrid,
    pub to_home_grid: ToHomeGrid,
    pub food_source_found: RwLock<bool>,
    pub food_returned_home: RwLock<bool>,
    pub step: usize,
}

impl rust_ab::engine::state::State for State {
    fn update(&mut self, step: usize) {
        self.ants_grid.update();
        self.to_food_grid.update();
        self.to_home_grid.update();
        self.step = step;
    }
}

impl State {
    pub fn new(width: i64, height: i64) -> State {
        State {
            ants_grid: AntsGrid::new(width, height),
            obstacles_grid: ObstaclesGrid::new(width, height),
            sites_grid: SitesGrid::new(width, height),
            to_food_grid: ToFoodGrid::new(width, height),
            to_home_grid: ToHomeGrid::new(width, height),
            food_source_found: RwLock::new(false),
            food_returned_home: RwLock::new(false),
            step: 0,
        }
    }

    /// Fetch a food pheromone from a particular grid cell.
    pub fn get_food_pheromone(&self, loc: &Int2D) -> Option<&f64> {
        self.to_food_grid.grid.get_value_at_pos(loc)
    }

    /// Set the value of a food pheromone in a particular grid cell.
    pub fn set_food_pheromone(&self, loc: &Int2D, value: f64) {
        self.to_food_grid.grid.set_value_at_pos(loc, value);
    }

    /// Fetch a home pheromone from a particular grid cell.
    pub fn get_home_pheromone(&self, loc: &Int2D) -> Option<&f64> {
        self.to_home_grid.grid.get_value_at_pos(loc)
    }

    /// Set the value of a home pheromone in a particular grid cell.
    pub fn set_home_pheromone(&self, loc: &Int2D, value: f64) {
        self.to_home_grid.grid.set_value_at_pos(loc, value);
    }

    /// Check if a particular grid cell has an obstacle or not. Will return None if the grid cell
    /// holds no obstacle, Some(StaticObjectType::OBSTACLE) otherwise.
    pub fn get_obstacle(&self, loc: &Int2D) -> Option<&StaticObjectType> {
        match self.obstacles_grid.grid.get_object_at_location(loc) {
            Some(_vec) => Some(&StaticObjectType::OBSTACLE),
            None => None
        }
    }

    /// Set an obstacle in a particular grid cell.
    pub fn set_obstacle(&self, loc: &Int2D) {
        self.obstacles_grid
            .grid
            .set_object_location(StaticObjectType::OBSTACLE, loc);
    }

    /// Set the location of an ant to a particular cell.
    pub fn set_ant_location(&self, ant: &mut Ant, loc: &Int2D) {
        self.ants_grid.grid.set_object_location(*ant, loc);
    }

    pub fn get_ant_location(&self, ant: &Ant) -> Option<&Int2D> {
        self.ants_grid.grid.get_object_location(*ant)
    }

    pub fn get_ant(&self, ant: &Ant) -> Option<&Ant> {
        self.ants_grid.grid.get_object(ant)
    }

    /// Check if a particular grid cell has a site or not. Will return None if the grid cell
    /// holds no site, Some(StaticObjectType::FOOD) or Some(StaticObjectType::HOME) otherwise.
    pub fn get_site(&self, loc: &Int2D) -> Option<&StaticObjectType> {
        match self.sites_grid.grid.get_object_at_location(loc) {
            Some(vec) => {
                if vec.len() > 1 {
                    panic!("A grid cell contains more than a site, this should not happen!");
                }
                vec.first()
            },
            None => None
        }
    }

    /// Set a particular site in a grid cell.
    pub fn set_site(&self, loc: &Int2D, site: StaticObjectType) {
        self.sites_grid.grid.set_object_location(site, loc);
    }

    pub fn update_sites(&mut self) {
        self.sites_grid.update();
    }

    pub fn update_obstacles(&mut self) {
        self.obstacles_grid.update();
    }
}
