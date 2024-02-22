use std::hash::Hash;
use krabmaga::engine::Component;
use krabmaga::engine::bevy_ecs;

use krabmaga::engine::location::Real2D;

#[derive(Clone, Copy, Component)]
pub struct Bird {
    pub id: u32,
}

#[derive(Component, Copy, Clone)]
pub struct LastReal2D(pub Real2D);

impl LastReal2D {
    pub fn new(loc: Real2D) -> LastReal2D {
        LastReal2D(loc)
    }
}