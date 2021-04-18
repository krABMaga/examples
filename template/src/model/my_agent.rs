use rust_ab::engine::agent::Agent;
use crate::model::my_state::MyState;
use std::hash::{Hash, Hasher};

/// The most basic agent should implement Clone, Copy and Agent to be able to be inserted in a Schedule.
#[derive(Clone, Copy)]
pub struct MyAgent {
    pub id: u128
}

impl Agent for MyAgent {
    type SimState = MyState;

    /// Put the code that should happen for each step, for each agent here.
    fn step(&mut self, _state: &MyState) {
        println!("Hi!");
    }
}

impl Hash for MyAgent {
    fn hash<H>(&self, state: &mut H)
        where
            H: Hasher,
    {
        self.id.hash(state);
    }
}

impl Eq for MyAgent {}

impl PartialEq for MyAgent {
    fn eq(&self, other: &MyAgent) -> bool {
        self.id == other.id
    }
}