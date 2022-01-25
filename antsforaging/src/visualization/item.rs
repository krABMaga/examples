use rust_ab::bevy::ecs::component::TableStorage;
use rust_ab::bevy::prelude::Component;

use crate::model::state::Item;

impl Component for Item { type Storage = TableStorage; }