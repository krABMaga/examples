use crate::{ENERGY_CONSUME, model::state::State};

use rust_ab::{engine::{agent::Agent, location::{Int2D, Location2D}}, rand::{self, Rng}};

use std::hash::{Hasher, Hash};


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

#[derive(Clone, Copy, Debug)]
pub enum LifeState {
    Alive,
    Dead,
}


pub trait AnimalActions {
    fn consume_energy(&mut self) -> LifeState;
    fn act(&mut self, state: &State);
    fn reproduce(&mut self, state: &State);
    fn eat(&mut self, state: &State);
    fn die(&mut self, state: &State);
}

impl Agent for Animal {
    type SimState = State;

    fn step(&mut self, state: &Self::SimState) {
        match self.animal_state {
            LifeState::Alive => {
                self.act(state);
                self.eat(state);
                let life_state = self.consume_energy();

                match life_state {
                    LifeState::Alive => {
                        self.reproduce(state);
                    }
                    LifeState::Dead => self.die(state),
                }
            }

            LifeState::Dead => {
                return;
            }
        }
    }
}

impl AnimalActions for Animal {

    fn consume_energy(&mut self) -> LifeState {
        self.energy = self.energy - ENERGY_CONSUME;
        if self.energy <= 0.0 {
            LifeState::Dead
        } else {
            LifeState::Alive
        }    }

    fn act(&mut self, state: &State) {
        match self.species{
            AnimalSpecies::Wolf => { self.wolf_act(state);}
            AnimalSpecies::Sheep => { self.sheep_act(state);}
        }
    }

    fn reproduce(&mut self, state: &State) {
        //waiting for Scheduler operation

        let mut rng = rand::thread_rng();
        if rng.gen_bool(self.prob_reproduction) {}
    }

    fn eat(&mut self, state: &State) {
        match self.species{
            AnimalSpecies::Wolf => { self.wolf_eat(state);}
            AnimalSpecies::Sheep => { self.sheep_eat(state);}
        }    }

    fn die(&mut self, state: &State) {
        println!("Animal {:?}{} dies at step {}", self.species, self.id, state.step);
        self.animal_state = LifeState::Dead;
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
