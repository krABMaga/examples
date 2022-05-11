use krabmaga::bevy::ecs::component::TableStorage;
use krabmaga::bevy::prelude::Component;

use crate::model::state::Item;

impl Component for Item {
    type Storage = TableStorage;
}
