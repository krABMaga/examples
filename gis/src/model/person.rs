use crate::model::map::Map;
use core::fmt;
use krabmaga::bevy::ecs::system::In;
use krabmaga::engine::agent::Agent;
use krabmaga::engine::fields::dense_number_grid_2d::DenseNumberGrid2D;
use krabmaga::engine::fields::field_2d::Location2D;
use krabmaga::engine::location::{Int2D, Real2D};
use krabmaga::engine::state::State;
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

//struct to store each neighbour in the current location of the agent in the simulation
#[derive(Clone, Copy)]
pub struct Neighbour {
    pub loc: Int2D,
    pub value: i32,
}

//the agent
#[derive(Clone, Copy)]
pub struct Person {
    pub id: u32,
    pub loc: Real2D,
    pub dir_x: f32,
    pub dir_y: f32,
    pub direction: Option<Direction>,
}

impl Person {
    #[allow(dead_code)]
    pub fn as_agent(self) -> Box<dyn Agent> {
        Box::new(self)
    }

    pub fn get_neighbours(&self, map: &Map) -> Vec<Neighbour> {
        let mut neighbours: Vec<Neighbour> = Vec::new();

        //up-left
        neighbours.push(Neighbour {
            loc: Int2D {
                x: self.loc.x as i32 - 1,
                y: self.loc.y as i32 + 1,
            },
            value: map.gis_value(Real2D {
                x: self.loc.x - 1.,
                y: self.loc.y + 1.,
            }),
        });

        //up
        neighbours.push(Neighbour {
            loc: Int2D {
                x: self.loc.x as i32,
                y: self.loc.y as i32 + 1,
            },
            value: map.gis_value(Real2D {
                x: self.loc.x,
                y: self.loc.y + 1.,
            }),
        });

        //up-right
        neighbours.push(Neighbour {
            loc: Int2D {
                x: self.loc.x as i32 + 1,
                y: self.loc.y as i32 + 1,
            },
            value: map.gis_value(Real2D {
                x: self.loc.x + 1.,
                y: self.loc.y + 1.,
            }),
        });

        //left
        neighbours.push(Neighbour {
            loc: Int2D {
                x: self.loc.x as i32 - 1,
                y: self.loc.y as i32,
            },
            value: map.gis_value(Real2D {
                x: self.loc.x - 1.,
                y: self.loc.y,
            }),
        });

        //right
        neighbours.push(Neighbour {
            loc: Int2D {
                x: self.loc.x as i32 + 1,
                y: self.loc.y as i32,
            },
            value: map.gis_value(Real2D {
                x: self.loc.x + 1.,
                y: self.loc.y,
            }),
        });

        //down-left
        neighbours.push(Neighbour {
            loc: Int2D {
                x: self.loc.x as i32 - 1,
                y: self.loc.y as i32 - 1,
            },
            value: map.gis_value(Real2D {
                x: self.loc.x - 1.,
                y: self.loc.y - 1.,
            }),
        });

        //down
        neighbours.push(Neighbour {
            loc: Int2D {
                x: self.loc.x as i32,
                y: self.loc.y as i32 - 1,
            },
            value: map.gis_value(Real2D {
                x: self.loc.x,
                y: self.loc.y - 1.,
            }),
        });

        //down-right
        neighbours.push(Neighbour {
            loc: Int2D {
                x: self.loc.x as i32 + 1,
                y: self.loc.y as i32 - 1,
            },
            value: map.gis_value(Real2D {
                x: self.loc.x + 1.,
                y: self.loc.y - 1.,
            }),
        });

        return neighbours;
    }

    pub fn determine_direction(&self, map: &Map) -> Option<Direction> {
        let neighbours = self.get_neighbours(map);
        let curr_direction = self.direction;
        let mut new_direction = Direction::Left;

        for neighbour in neighbours {
            let x = neighbour.loc.x as f32;
            let y = neighbour.loc.y as f32;

            if neighbour.value == 1 {
                match curr_direction {
                    Some(Direction::Left) => {
                        if x == self.loc.x && y == self.loc.y + 1. {
                            new_direction = Direction::Up;
                        }
                        if x == self.loc.x && y == self.loc.y - 1. {
                            new_direction = Direction::Down;
                        }
                        if x == self.loc.x - 1. && y == self.loc.y - 1. {
                            new_direction = Direction::DownLeft;
                        }
                        if x == self.loc.x - 1. && y == self.loc.y + 1. {
                            new_direction = Direction::UpLeft;
                        }
                        if x == self.loc.x + 1. && y == self.loc.y - 1. {
                            new_direction = Direction::DownRight;
                        }
                        if x == self.loc.x + 1. && y == self.loc.y + 1. {
                            new_direction = Direction::UpRight;
                        }
                    }
                    Some(Direction::Right) => {
                        if x == self.loc.x && y == self.loc.y + 1. {
                            new_direction = Direction::Up;
                        }
                        if x == self.loc.x && y == self.loc.y - 1. {
                            new_direction = Direction::Down;
                        }
                        if x == self.loc.x - 1. && y == self.loc.y - 1. {
                            new_direction = Direction::DownLeft;
                        }
                        if x == self.loc.x - 1. && y == self.loc.y + 1. {
                            new_direction = Direction::UpLeft;
                        }
                        if x == self.loc.x + 1. && y == self.loc.y - 1. {
                            new_direction = Direction::DownRight;
                        }
                        if x == self.loc.x + 1. && y == self.loc.y + 1. {
                            new_direction = Direction::UpRight;
                        }
                    }
                    Some(Direction::Down) => {
                        if x == self.loc.x + 1. && y == self.loc.y {
                            new_direction = Direction::Right;
                        }
                        if x == self.loc.x - 1. && y == self.loc.y {
                            new_direction = Direction::Left;
                        }
                        if x == self.loc.x - 1. && y == self.loc.y - 1. {
                            new_direction = Direction::DownLeft;
                        }
                        if x == self.loc.x - 1. && y == self.loc.y + 1. {
                            new_direction = Direction::UpLeft;
                        }
                        if x == self.loc.x + 1. && y == self.loc.y - 1. {
                            new_direction = Direction::DownRight;
                        }
                        if x == self.loc.x + 1. && y == self.loc.y + 1. {
                            new_direction = Direction::UpRight;
                        }
                    }
                    Some(Direction::Up) => {
                        if x == self.loc.x + 1. && y == self.loc.y {
                            new_direction = Direction::Right;
                        } else if x == self.loc.x - 1. && y == self.loc.y {
                            new_direction = Direction::Left;
                        } else {
                            new_direction = Direction::Up;
                        }
                        if x == self.loc.x - 1. && y == self.loc.y - 1. {
                            new_direction = Direction::DownLeft;
                        }
                        if x == self.loc.x - 1. && y == self.loc.y + 1. {
                            new_direction = Direction::UpLeft;
                        }
                        if x == self.loc.x + 1. && y == self.loc.y - 1. {
                            new_direction = Direction::DownRight;
                        }
                        if x == self.loc.x + 1. && y == self.loc.y + 1. {
                            new_direction = Direction::UpRight;
                        }
                    }
                    Some(Direction::UpRight) => {
                        if x == self.loc.x + 1. && y == self.loc.y + 1. {
                            new_direction = Direction::Up;
                        }
                        if x == self.loc.x - 1. && y == self.loc.y + 1. {
                            new_direction = Direction::Up;
                        }
                        if x == self.loc.x - 1. && y == self.loc.y - 1. {
                            new_direction = Direction::DownLeft;
                        }
                        if x == self.loc.x - 1. && y == self.loc.y + 1. {
                            new_direction = Direction::UpLeft;
                        }
                        if x == self.loc.x + 1. && y == self.loc.y - 1. {
                            new_direction = Direction::DownRight;
                        }
                        if x == self.loc.x + 1. && y == self.loc.y + 1. {
                            new_direction = Direction::UpRight;
                        }
                    }
                    Some(Direction::UpLeft) => {
                        if x == self.loc.x + 1. && y == self.loc.y - 1. {
                            new_direction = Direction::Up;
                        }
                        if x == self.loc.x - 1. && y == self.loc.y - 1. {
                            new_direction = Direction::Up;
                        }
                        if x == self.loc.x - 1. && y == self.loc.y - 1. {
                            new_direction = Direction::DownLeft;
                        }
                        if x == self.loc.x - 1. && y == self.loc.y + 1. {
                            new_direction = Direction::UpLeft;
                        }
                        if x == self.loc.x + 1. && y == self.loc.y - 1. {
                            new_direction = Direction::DownRight;
                        }
                        if x == self.loc.x + 1. && y == self.loc.y + 1. {
                            new_direction = Direction::UpRight;
                        }
                        if x == self.loc.x - 1. && y == self.loc.y - 1. {
                            new_direction = Direction::DownLeft;
                        }
                        if x == self.loc.x - 1. && y == self.loc.y + 1. {
                            new_direction = Direction::UpLeft;
                        }
                        if x == self.loc.x + 1. && y == self.loc.y - 1. {
                            new_direction = Direction::DownRight;
                        }
                        if x == self.loc.x + 1. && y == self.loc.y + 1. {
                            new_direction = Direction::UpRight;
                        }
                    }
                    Some(Direction::DownRight) => {
                        if x == self.loc.x + 1. && y == self.loc.y - 1. {
                            new_direction = Direction::Down;
                        }
                        if x == self.loc.x - 1. && y == self.loc.y - 1. {
                            new_direction = Direction::Down;
                        }
                        if x == self.loc.x - 1. && y == self.loc.y - 1. {
                            new_direction = Direction::DownLeft;
                        }
                        if x == self.loc.x - 1. && y == self.loc.y + 1. {
                            new_direction = Direction::UpLeft;
                        }
                        if x == self.loc.x + 1. && y == self.loc.y - 1. {
                            new_direction = Direction::DownRight;
                        }
                        if x == self.loc.x + 1. && y == self.loc.y + 1. {
                            new_direction = Direction::UpRight;
                        }
                    }
                    Some(Direction::DownLeft) => {
                        if x == self.loc.x + 1. && y == self.loc.y + 1. {
                            new_direction = Direction::Down;
                        }
                        if x == self.loc.x - 1. && y == self.loc.y + 1. {
                            new_direction = Direction::Down;
                        }
                        if x == self.loc.x - 1. && y == self.loc.y - 1. {
                            new_direction = Direction::DownLeft;
                        }
                        if x == self.loc.x - 1. && y == self.loc.y + 1. {
                            new_direction = Direction::UpLeft;
                        }
                        if x == self.loc.x + 1. && y == self.loc.y - 1. {
                            new_direction = Direction::DownRight;
                        }
                        if x == self.loc.x + 1. && y == self.loc.y + 1. {
                            new_direction = Direction::UpRight;
                        }
                    }
                    None => {
                        return Some(Direction::Right);
                    }
                }
            }
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
                self.loc.x -= 1.;
                self.direction = Some(Direction::Left);
            }
            Some(Direction::Right) => {
                self.loc.x += 1.;
                self.direction = Some(Direction::Right);
            }
            Some(Direction::Up) => {
                self.loc.y += 1.;
                self.direction = Some(Direction::Up);
            }
            Some(Direction::Down) => {
                self.loc.y -= 1.;
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
            Some(Direction::DownRight) => {
                self.loc.x += 1.;
                self.loc.y -= 1.;
                self.direction = Some(Direction::DownRight);
            }
            Some(Direction::DownLeft) => {
                self.loc.x -= 1.;
                self.loc.y -= 1.;
                self.direction = Some(Direction::DownLeft);
            }
            None => {
                self.loc = self.loc;
                self.direction = None;
            }
        }
        map.field.set_object_location(*self, self.loc);
    }

    /// Put the code that decides if an agent should be removed or not
    /// for example in simulation where agents can die
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
