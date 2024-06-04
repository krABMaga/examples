use crate::model::map::Map;
use core::fmt;
use krabmaga::engine::agent::Agent;
use krabmaga::engine::fields::field_2d::Location2D;
use krabmaga::engine::location::{Int2D, Real2D};
use krabmaga::engine::state::State;
use rand::Rng;
use std::hash::{Hash, Hasher};

// The Direction of the person
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
}

//the agent
#[derive(Clone, Copy)]
pub struct Person {
    pub id: u32,
    pub loc: Real2D,
    pub direction: Option<Direction>,
}

impl Person {
    #[allow(dead_code)]
    pub fn as_agent(self) -> Box<dyn Agent> {
        Box::new(self)
    }

    pub fn determine_direction(&self, map: &Map) -> Option<Direction> {
        let x = self.loc.x as i32;
        let y = self.loc.y as i32;
        let curr_direction = self.direction;
        let mut new_direction = Direction::Left;
        let mut possible_direction: Vec<Direction> = Vec::new();

        for dx in -1..2 {
            for dy in -1..2 {
                
                let new_x = dx + x;
                let new_y = dy + y;
                if map.gis_value(Int2D { x: new_x, y: new_y }) == 1 {
                    if dx == 0 && dy == -1 {
                        possible_direction.push(Direction::Up);
                    }
                    if dx == 1 && dy == 0 {
                        possible_direction.push(Direction::Left);
                    }
                    if dx == -1 && dy == 0 {
                        possible_direction.push(Direction::Right);
                    }
                    if dx == 0 && dy == 1 {
                        possible_direction.push(Direction::Down);
                    }
                }
            }
        }
        
        match curr_direction {
            Some(Direction::Left) => {
                if possible_direction.contains(&Direction::Left) {
                    new_direction = Direction::Left;
                } else {
                    new_direction = Direction::Up;
                }
            }
            Some(Direction::Right) => {
                if possible_direction.contains(&Direction::Right) {
                    new_direction = Direction::Right;
                } else {
                    new_direction = Direction::Down;
                }
            }
            Some(Direction::Up) => {
                if possible_direction.contains(&Direction::Up) {
                    new_direction = Direction::Up;
                } else {
                    new_direction = Direction::Right;
                }
            }
            Some(Direction::Down) => {
                if possible_direction.contains(&Direction::Down) {
                    new_direction = Direction::Down;
                } else {
                    new_direction = Direction::Left;
                }
            }
            Some(Direction::UpRight) => {
                if possible_direction.contains(&Direction::UpRight) {
                    new_direction = Direction::UpRight;
                } else {
                    new_direction = Direction::DownLeft;
                }
            }
            Some(Direction::UpLeft) => {
                if possible_direction.contains(&Direction::UpLeft) {
                    new_direction = Direction::UpLeft;
                } else {
                    new_direction = Direction::DownRight;
                }
            }
            Some(Direction::DownLeft) => {
                if possible_direction.contains(&Direction::DownLeft) {
                    new_direction = Direction::DownLeft;
                } else {
                    new_direction = Direction::UpRight;
                }
            }
            Some(Direction::DownRight) => {
                if possible_direction.contains(&Direction::DownRight) {
                    new_direction = Direction::DownRight;
                } else {
                    new_direction = Direction::UpLeft;
                }
            }
            None => new_direction = Direction::Left,
        }

        Some(new_direction)
    }
}

impl Agent for Person {
    /// Put the code that should happen for each step, for each agent here.
    fn step(&mut self, state: &mut dyn State) {
        let map = state.as_any().downcast_ref::<Map>().unwrap();
        let direction = self.determine_direction(map);

        match direction {
            Some(Direction::Left) => {
                self.loc.x += 1.;
                self.direction = Some(Direction::Left);
            }
            Some(Direction::Right) => {
                self.loc.x -= 1.;
                self.direction = Some(Direction::Right);
            }
            Some(Direction::Up) => {
                self.loc.y -= 1.;
                self.direction = Some(Direction::Up);
            }
            Some(Direction::Down) => {
                self.loc.y += 1.;
                self.direction = Some(Direction::Down);
            }
            Some(Direction::UpRight) => {
                self.loc.x += 1.;
                self.loc.y += 1.;
                self.direction = Some(Direction::UpRight);
            }
            Some(Direction::UpLeft) => {
                self.loc.x -= 1.;
                self.loc.y += 1.;
                self.direction = Some(Direction::UpLeft);
            }
            Some(Direction::DownLeft) => {
                self.loc.x -= 1.;
                self.loc.y -= 1.;
                self.direction = Some(Direction::DownLeft);
            }
            Some(Direction::DownRight) => {
                self.loc.x += 1.;
                self.loc.y -= 1.;
                self.direction = Some(Direction::DownRight);
            }
            None => {
                self.loc = self.loc;
                self.direction = None;
            }
        }
        map.field.set_object_location(*self, self.loc);
    }

    fn is_stopped(&mut self, _state: &mut dyn State) -> bool {
        false
    }
}

impl Hash for Person {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.id.hash(state);
    }
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl Eq for Person {}

impl PartialEq for Person {
    fn eq(&self, other: &Person) -> bool {
        self.id == other.id
    }
}

impl Location2D<Real2D> for Person {
    fn get_location(self) -> Real2D {
        self.loc
    }

    fn set_location(&mut self, loc: Real2D) {
        self.loc = loc;
    }
}
