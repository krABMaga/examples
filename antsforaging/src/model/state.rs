use crate::model::ant::Ant;
use crate::model::to_food_grid::ToFoodGrid;
use crate::model::to_home_grid::ToHomeGrid;
use crate::{
    FOOD_XMAX, FOOD_XMIN, FOOD_YMAX, FOOD_YMIN, HEIGHT, HOME_XMAX, HOME_XMIN, HOME_YMAX, HOME_YMIN,
    NUM_AGENT, WIDTH,
};
use core::fmt;
use core::hash::{Hash, Hasher};
use krabmaga::engine::fields::field::Field;
use krabmaga::engine::fields::sparse_object_grid_2d::SparseGrid2D;
use krabmaga::engine::location::Int2D;
use krabmaga::engine::schedule::Schedule;
use krabmaga::engine::state::State;
use krabmaga::rand;
use krabmaga::rand::Rng;
use std::any::Any;
use std::sync::RwLock;
// Objects within the field
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum ItemType {
    Food,
    Home,
    Obstacle,
}

impl fmt::Display for ItemType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ItemType::Food => write!(f, "Food"),
            ItemType::Home => write!(f, "Home"),
            ItemType::Obstacle => write!(f, "Obstacle"),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Item {
    pub id: u32,
    pub value: ItemType,
}

impl Hash for Item {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.id.hash(state);
    }
}

impl Eq for Item {}

impl PartialEq for Item {
    fn eq(&self, other: &Item) -> bool {
        self.id == other.id
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} value {}", self.id, self.value)
    }
}

// The global simulation state. This holds the various grids used for movement, exposing setter methods
// so that the state itself will worry about ownership rules by mutating its own fields.
pub struct ModelState {
    pub ants_grid: SparseGrid2D<Ant>,
    pub obstacles_grid: SparseGrid2D<Item>,
    pub to_food_grid: ToFoodGrid,
    pub to_home_grid: ToHomeGrid,
    pub food_source_found: RwLock<bool>,
    pub food_returned_home: RwLock<bool>,
    pub step: u64,
}

impl State for ModelState {
    fn reset(&mut self) {
        self.step = 0;
        self.ants_grid = SparseGrid2D::new(WIDTH, HEIGHT);
        self.obstacles_grid = SparseGrid2D::new(WIDTH, HEIGHT);
        self.to_food_grid = ToFoodGrid::new(WIDTH, HEIGHT);
        self.to_home_grid = ToHomeGrid::new(WIDTH, HEIGHT);
        self.food_source_found = RwLock::new(false);
        self.food_returned_home = RwLock::new(false);
    }

    fn init(&mut self, schedule: &mut Schedule) {
        self.step = 0;
        self.ants_grid = SparseGrid2D::new(WIDTH, HEIGHT);
        self.obstacles_grid = SparseGrid2D::new(WIDTH, HEIGHT);
        self.to_food_grid = ToFoodGrid::new(WIDTH, HEIGHT);
        self.to_home_grid = ToHomeGrid::new(WIDTH, HEIGHT);
        self.food_source_found = RwLock::new(false);
        self.food_returned_home = RwLock::new(false);

        let mut rng = rand::thread_rng();

        // Food generation
        let x: i32 = if FOOD_XMIN == FOOD_XMAX {
            FOOD_XMIN
        } else {
            rng.gen_range(FOOD_XMIN..FOOD_XMAX)
        };
        let y: i32 = if FOOD_YMIN == FOOD_YMAX {
            FOOD_YMIN
        } else {
            rng.gen_range(FOOD_YMIN..FOOD_YMAX)
        };
        let food_location = Int2D { x, y };

        self.obstacles_grid.set_object_location(
            Item {
                id: 888888888,
                value: ItemType::Food,
            },
            &food_location,
        );

        // Nest generation
        let x: i32 = if HOME_XMIN == HOME_XMAX {
            HOME_XMIN
        } else {
            rng.gen_range(HOME_XMIN..HOME_XMAX)
        };
        let y: i32 = if HOME_YMIN == HOME_YMAX {
            HOME_YMIN
        } else {
            rng.gen_range(HOME_YMIN..HOME_YMAX)
        };
        let nest_location = Int2D { x, y };
        self.obstacles_grid.set_object_location(
            Item {
                id: 99999999,
                value: ItemType::Home,
            },
            &nest_location,
        );

        // Obastacles generation
        /* General formula to calculate an ellipsis, used to draw obstacles.
           x and y define a specific cell
           horizontal and vertical define the ellipsis location (bottom left: 0,0)
           size defines the ellipsis' size (smaller value = bigger ellipsis)
        */
        let ellipsis = |x: f32, y: f32, horizontal: f32, vertical: f32, size: f32| -> bool {
            ((x - horizontal) * size + (y - vertical) * size)
                * ((x - horizontal) * size + (y - vertical) * size)
                / 36.
                + ((x - horizontal) * size - (y - vertical) * size)
                    * ((x - horizontal) * size - (y - vertical) * size)
                    / 1024.
                <= 1.
        };

        let mut obstacle_id = 0;
        for i in 0..WIDTH {
            for j in 0..HEIGHT {
                // Good obstacle placement for 500x500 simulations
                // if ellipsis(i as f32, j as f32, 300., 345., 0.407)
                //    || ellipsis(i as f32, j as f32, 190., 155., 0.407)
                if ellipsis(i as f32, j as f32, 100., 145., 0.407)
                    || ellipsis(i as f32, j as f32, 90., 55., 0.407)
                {
                    let obstacle_location = Int2D { x: i, y: j };
                    self.obstacles_grid.set_object_location(
                        Item {
                            id: obstacle_id,
                            value: ItemType::Obstacle,
                        },
                        &obstacle_location,
                    );
                }
                obstacle_id += 1;
            }
        }

        // Ants generation
        for ant_id in 0..NUM_AGENT {
            let x = (HOME_XMAX + HOME_XMIN) / 2;
            let y = (HOME_YMAX + HOME_YMIN) / 2;
            let ant_loc = Int2D { x, y };
            // Generate the ant with an initial reward of 1, so that it starts spreading home pheromones
            // around the nest, the initial spawn point.
            let ant = Ant::new(ant_id, ant_loc, false, 1.);
            self.ants_grid.set_object_location(ant, &ant_loc);
            schedule.schedule_repeating(Box::new(ant), 0., 0);
        }

        self.obstacles_grid.update();
    }

    fn update(&mut self, step: u64) {
        self.ants_grid.lazy_update();
        self.to_food_grid.update();
        self.to_home_grid.update();
        self.step = step;
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
}

impl ModelState {
    pub(crate) fn new() -> ModelState {
        ModelState {
            ants_grid: SparseGrid2D::new(WIDTH, HEIGHT),
            obstacles_grid: SparseGrid2D::new(WIDTH, HEIGHT),
            to_food_grid: ToFoodGrid::new(WIDTH, HEIGHT),
            to_home_grid: ToHomeGrid::new(WIDTH, HEIGHT),
            food_source_found: RwLock::new(false),
            food_returned_home: RwLock::new(false),
            step: 0,
        }
    }

    // Check if a particular grid cell has an obstacle or not. Will return None if the grid cell holds no obstacle.
    pub fn get_obstacle(&self, loc: &Int2D) -> Option<Vec<Item>> {
        self.obstacles_grid
            .get_objects(loc)
            .filter(|vec| vec.first().unwrap().value == ItemType::Obstacle)
    }
}
