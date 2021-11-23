use crate::model::world::*;
use rust_ab::engine::fields::dense_object_grid_2d::DenseGrid2D;
use rust_ab::engine::fields::sparse_object_grid_2d::SparseGrid2D;
use rust_ab::engine::location::Int2D;
use rust_ab::visualization::fields::object_grid_2d::RenderObjectGrid2D;

impl RenderObjectGrid2D<World, Patch> for SparseGrid2D<Patch> {
    fn get_sparse_grid(state: &World) -> Option<&SparseGrid2D<Patch>> {
        Some(&state.field)
    }

    fn get_dense_grid(_state: &World) -> Option<&DenseGrid2D<Patch>> {
        None
    }

    fn get_emoji_obj(state: &World, obj: &Patch) -> String {
        let obj_real = state.field.get(obj).unwrap();
        match obj_real.value {
            Status::Red => "red_heart".to_string(),
            Status::Blue => "blue_heart".to_string(),
        }
    }
    fn scale(_obj: &Patch) -> (f32, f32) {
        (0.01, 0.01)
    }
    fn get_pos_obj(state: &World, obj: &Patch) -> Option<Int2D> {
        state.field.get_location(*obj)
    }
    fn get_rotation_obj(_state: &World, _obj: &Patch) -> f32 {
        0.0
    }
}
