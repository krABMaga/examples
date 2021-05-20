use crate::{model::state::State, ENERGY_CONSUME};

use rust_ab::{
    engine::{
        agent::Agent,
        location::{Int2D, Location2D},
    },
    rand::{self, Rng},
};

use rust_ab::engine::schedule::ScheduleOptions;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::option::Option::None;

pub const FOOD_CHOICE: f64 = 0.5;

#[derive(Copy, Clone, Debug)]
pub enum AnimalSpecies {
    Wolf,
    Sheep,
}

#[derive(Copy, Clone)]
pub struct Animal {
    /// An unique id.
    pub id: u128,
    /// Animal species.
    pub species: AnimalSpecies,
    /// Field to represent lifestate of animal
    pub animal_state: LifeState,
    /// The position of the agent.
    pub loc: Int2D,
    /// Last position of the agent, starts as None.
    pub last: Option<Int2D>,
    /// Animal's energy, that they consume each step. They can restore it by eating.
    /// If an animal runs out of energy, it will die.
    pub energy: f64,
    ///The amount of energy that the animal can gain by eating
    pub gain_energy: f64,
    ///The probablity of reproduction. Through this parameter, is possible the "birth"
    ///of new animal istance
    pub prob_reproduction: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LifeState {
    Alive,
    Dead,
}

pub trait AnimalActions {
    fn consume_energy(&mut self) -> LifeState;
    fn act(&mut self, state: &State);
    fn eat(&mut self, state: &State);
}

impl Agent for Animal {
    type SimState = State;

    fn step(&mut self, state: &Self::SimState) {
        println!(
            "{:?}{} i'm scheduled at step {}, energy: {}, loc: {} {}",
            self.species, self.id, state.step, self.energy, self.loc.x, self.loc.y
        );

        self.act(state);
        self.eat(state);
        self.animal_state = self.consume_energy();
        if let LifeState::Dead = self.animal_state {
            //state.remove_animal(*self); // this breaks the visualization since it cannot fetch the animal from the state anymore
            if let AnimalSpecies::Sheep = self.species {
                state.set_sheep_location(self, &self.loc);
            } else {
                state.set_wolf_location(self, &self.loc);
            }
            return;
        }
    }

    fn should_remove(&mut self, state: &Self::SimState) -> bool {
        if self.animal_state == LifeState::Dead {
            println!(
                "Animal {:?}{} dies at step {}",
                self.species, self.id, state.step
            );
            true
        } else {
            false
        }
    }

    fn should_reproduce(
        &mut self,
        state: &Self::SimState,
    ) -> Option<HashMap<Box<Self>, ScheduleOptions>> {
        let mut rng = rand::thread_rng();
        if let LifeState::Alive = self.animal_state {
            if rng.gen_bool(self.prob_reproduction) {
                let mut map = HashMap::new();
                let new_animal = state.reproduce_animal(self);
                let ordering = if let AnimalSpecies::Wolf = new_animal.species {
                    1
                } else {
                    0
                };
                map.insert(
                    Box::new(new_animal),
                    ScheduleOptions {
                        ordering,
                        repeating: true,
                    },
                );
                println!(
                    "-----\n{:?}{} is a mum: {:?}{} is born\n-----",
                    self.species, self.id, new_animal.species, new_animal.id
                );
                return Some(map);
            }
        }
        None
    }
}

impl AnimalActions for Animal {
    fn consume_energy(&mut self) -> LifeState {
        self.energy = self.energy - ENERGY_CONSUME;
        if self.energy <= 0.0 {
            LifeState::Dead
        } else {
            LifeState::Alive
        }
    }

    fn act(&mut self, state: &State) {
        match self.species {
            AnimalSpecies::Wolf => {
                self.wolf_act(state);
            }
            AnimalSpecies::Sheep => {
                self.sheep_act(state);
                /*println!("Sheep moved to {} {}", self.loc.x, self.loc.y);
                if self.last.is_some() {
                    if let Some(_a) = state.get_sheep_at_location(&self.last.unwrap()) {
                        println!("Same agent in previous position");
                    }
                } */
            }
        }
    }

    fn eat(&mut self, state: &State) {
        match self.species {
            AnimalSpecies::Wolf => {
                self.wolf_eat(state);
            }
            AnimalSpecies::Sheep => {
                self.sheep_eat(state);
            }
        }
    }
}

//----------------------------------------------------------------
impl Eq for Animal {}

impl PartialEq for Animal {
    fn eq(&self, other: &Animal) -> bool {
        self.id == other.id
    }
}

impl Location2D<Int2D> for Animal {
    fn get_location(self) -> Int2D {
        self.loc
    }

    fn set_location(&mut self, loc: Int2D) {
        self.loc = loc;
    }
}

impl Hash for Animal {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.id.hash(state);
    }
}
