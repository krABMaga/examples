use crate::model::animals_grid::AnimalsGrid;
use rust_ab::engine::location::Int2D;
use crate::model::grass::*;
use crate::model::animals::*;




pub struct State {
    pub wolves_grid: AnimalsGrid,
    pub sheeps_grid: AnimalsGrid,
    pub grass_field: GrassField,
    pub step: u128,
    pub num_sheep: u128,
    pub num_wolf: u128,
}

impl rust_ab::engine::state::State for State {
    fn update(&mut self) {

        self.grass_field.update();
        self.sheeps_grid.update();
        self.wolves_grid.update();
        
    }
}

impl State {
    pub fn new(width: i64, height: i64) -> State {
        State {
            wolves_grid: AnimalsGrid::new(width, height),
            sheeps_grid: AnimalsGrid::new(width, height),
            grass_field: GrassField::new(width, height),
            step: 0,
            num_sheep: 0,
            num_wolf: 0
        }
    }
 
    pub fn set_wolf_location(&self, wolf: &mut Animal, loc: &Int2D){
        self.wolves_grid.grid.set_object_location(*wolf, loc);
    }

    pub fn get_wolf_location(&self, wolf: &Animal) -> Option<&Int2D>{
        self.wolves_grid.grid.get_object_location(*wolf) 
    }

    pub fn set_sheep_location(&self, sheep: Animal, loc: &Int2D){
        self.sheeps_grid.grid.set_object_location(sheep, loc);
    }

    pub fn get_sheep_location(&self, sheep: &Animal) -> Option<&Int2D>{
        self.sheeps_grid.grid.get_object_location(*sheep) 
    }

    pub fn get_sheep_at_location(&self, loc: &Int2D) -> Option<&Animal>{
        self.sheeps_grid.grid.get_object_at_location(loc)
    }
  
    

    pub fn set_grass_at_location(&self, loc: &Int2D, value: u16){
        self.grass_field.grid.set_value_at_pos(loc, value);
    }

    pub fn get_grass_at_location(&self, loc: &Int2D) -> Option<&u16>{
        self.grass_field.grid.get_value_at_pos(loc)
    }
}