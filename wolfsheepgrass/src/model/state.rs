use crate::model::animals::*;
use crate::model::grass::*;
use crate::{model::animals_grid::AnimalsGrid, NUM_SHEEPS, NUM_WOLVES};
use rust_ab::engine::{location::Int2D, schedule::Schedule};
use std::{
    sync::{Arc, Mutex},
    u128,
};

pub struct State {
    pub wolves_grid: AnimalsGrid,
    pub sheeps_grid: AnimalsGrid,
    pub grass_field: GrassField,
    pub step: usize,
    pub next_id: Arc<Mutex<u128>>,
}

impl rust_ab::engine::state::State for State {
    fn update(&mut self, step: usize) {
        self.grass_field.update();
        self.sheeps_grid.update();
        self.wolves_grid.update();
        self.step = step;
    }
}

impl State {
    pub fn new(width: i64, height: i64) -> State {
        State {
            wolves_grid: AnimalsGrid::new(width, height),
            sheeps_grid: AnimalsGrid::new(width, height),
            grass_field: GrassField::new(width, height),
            step: 0,
            next_id: Arc::new(Mutex::new(NUM_SHEEPS + NUM_WOLVES)),
        }
    }

    pub fn set_wolf_location(&self, wolf: &Animal, loc: &Int2D) {
        self.wolves_grid.grid.set_object_location(*wolf, loc);
    }

    pub fn get_wolf_location(&self, wolf: &Animal) -> Option<&Int2D> {
        self.wolves_grid.grid.get_object_location(*wolf)
    }

    pub fn set_sheep_location(&self, sheep: &Animal, loc: &Int2D) {
        self.sheeps_grid.grid.set_object_location(*sheep, loc);
    }

    pub fn get_sheep_location(&self, sheep: &Animal) -> Option<&Int2D> {
        self.sheeps_grid.grid.get_object_location(*sheep)
    }

    pub fn get_sheep(&self, sheep: &Animal) -> Option<&Animal> {
        self.sheeps_grid.grid.get_object(sheep)
    }

    pub fn get_wolf(&self, wolf: &Animal) -> Option<&Animal> {
        self.wolves_grid.grid.get_object(wolf)
    }

    pub fn get_wolf_at_location(&self, loc: &Int2D) -> Option<&Animal> {
        if let Some(vec) = self.wolves_grid.grid.get_object_at_location(loc) {
            vec.first()
        } else {
            None
        }
    }

    pub fn get_sheep_at_location(&self, loc: &Int2D) -> Option<&Animal> {
        if let Some(vec) = self.sheeps_grid.grid.get_object_at_location(loc) {
            vec.first()
        } else {
            None
        }
    }

    pub fn remove_animal(&self, animal: Animal) {
        match animal.species {
            AnimalSpecies::Wolf => {
                self.wolves_grid.grid.remove_object(&animal);
                //self.wolves_grid.grid.update();
            }

            AnimalSpecies::Sheep => {
                self.sheeps_grid.grid.remove_object(&animal);
                //self.sheeps_grid.update();

                /* assert!(self.get_sheep_location(&animal) == None);
                assert!(self.get_sheep_at_location(&animal.loc) == None);

                if let Some(a)  = self.get_sheep_at_location(&animal.loc){
                    println!("\nNot removed from reverse Sheep {}  loc: {} {}, loc in agent {} {}", a.id, a.loc.x, a.loc.y, animal.loc.x, animal.loc.y);
                    if let Some(_p) = self.get_sheep_location(a){
                        println!("Not removed: loc {} {}", _p.x, _p.y );
                    }
                    else {
                        println!("removed only from one grid");
                    }


                    println!("Errore nella rimozione");
                }*/
            }
        }
    }

    pub fn reproduce_animal(&self, animal: &mut Animal) -> Animal {
        animal.energy /= 2.0;
        let mut new_animal = animal.clone();

        {
            let mut new_id = self.next_id.lock().unwrap();
            new_animal.id = *new_id;
            *new_id += 1;
        }

        match new_animal.species {
            AnimalSpecies::Wolf => {
                self.set_wolf_location(&mut new_animal, &animal.loc);
            }
            AnimalSpecies::Sheep => {
                self.set_sheep_location(&mut new_animal, &animal.loc);
            }
        }

        return new_animal;
    }

    pub fn set_grass_at_location(&self, loc: &Int2D, value: u16) {
        self.grass_field.grid.set_value_at_pos(loc, value);
    }

    pub fn get_grass_at_location(&self, loc: &Int2D) -> Option<&u16> {
        self.grass_field.grid.get_value_at_pos(loc)
    }
}