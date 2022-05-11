use crate::model::forest::*;
use krabmaga::bevy::ecs::component::TableStorage;
use krabmaga::bevy::prelude::Component;
use krabmaga::engine::fields::dense_object_grid_2d::DenseGrid2D;
use krabmaga::engine::fields::sparse_object_grid_2d::SparseGrid2D;
use krabmaga::engine::location::Int2D;
use krabmaga::visualization::fields::object_grid_2d::RenderObjectGrid2D;

impl Component for Tree {
    type Storage = TableStorage;
}

impl RenderObjectGrid2D<Forest, Tree> for DenseGrid2D<Tree> {
    fn fetch_sparse_grid(_state: &Forest) -> Option<&SparseGrid2D<Tree>> {
        None
    }
    fn fetch_dense_grid(state: &Forest) -> Option<&DenseGrid2D<Tree>> {
        Some(&state.field)
    }
    fn fetch_emoji(state: &Forest, obj: &Tree) -> String {
        let obj_real = state.field.get(obj).unwrap();
        return match obj_real.status {
            Status::Green => "evergreen_tree".to_string(),
            Status::Burning => "fire".to_string(),
            Status::Burned => "dust".to_string(),
        };
    }
    fn fetch_loc(state: &Forest, obj: &Tree) -> Option<Int2D> {
        state.field.get_location(*obj)
    }
    fn fetch_rotation(_state: &Forest, _obj: &Tree) -> f32 {
        0.0
    }
    fn scale(_obj: &Tree) -> (f32, f32) {
        (0.03, 0.03)
    }
}
