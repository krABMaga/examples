use crate::model::world::*;
use krabmaga::bevy::ecs::component::TableStorage;
use krabmaga::bevy::prelude::Component;
use krabmaga::engine::fields::dense_object_grid_2d::DenseGrid2D;
use krabmaga::engine::fields::sparse_object_grid_2d::SparseGrid2D;
use krabmaga::engine::location::Int2D;
use krabmaga::visualization::fields::object_grid_2d::RenderObjectGrid2D;

impl Component for Patch {
    type Storage = TableStorage;
}

impl RenderObjectGrid2D<World, Patch> for SparseGrid2D<Patch> {
    fn fetch_sparse_grid(state: &World) -> Option<&SparseGrid2D<Patch>> {
        Some(&state.field)
    }

    fn fetch_dense_grid(_state: &World) -> Option<&DenseGrid2D<Patch>> {
        None
    }

    fn fetch_emoji(state: &World, obj: &Patch) -> String {
        let obj_real = state.field.get(obj).unwrap();
        match obj_real.value {
            Status::Red => "red_heart".to_string(),
            Status::Blue => "blue_heart".to_string(),
        }
    }
    fn fetch_loc(state: &World, obj: &Patch) -> Option<Int2D> {
        state.field.get_location(*obj)
    }
    fn fetch_rotation(_state: &World, _obj: &Patch) -> f32 {
        0.0
    }
    fn scale(_obj: &Patch) -> (f32, f32) {
        (0.01, 0.01)
    }
}
