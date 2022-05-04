use krABMaga::bevy::ecs::component::TableStorage;
use krABMaga::bevy::prelude::Component;

use crate::model::state::Item;

impl Component for Item { type Storage = TableStorage; }