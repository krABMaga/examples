use crate::model::forest::*;
use rust_ab::engine::fields::dense_object_grid_2d::DenseGrid2D;
use rust_ab::engine::fields::sparse_object_grid_2d::SparseGrid2D;
use rust_ab::engine::location::Int2D;
use rust_ab::visualization::fields::object_grid_2d::RenderObjectGrid2D;

impl RenderObjectGrid2D<Forest, Tree> for DenseGrid2D<Tree> {
    fn get_emoji_obj(state: &Forest, obj: &Tree) -> String {
        let obj_real = state.field.get(obj).unwrap();
        match obj_real.status {
            Status::Green => return "evergreen_tree".to_string(),
            Status::Burning => return "fire".to_string(),
            Status::Burned => return "dust".to_string(),
        }
    }
    fn scale(_obj: &Tree) -> (f32, f32) {
        (0.03, 0.03)
    }
    fn get_pos_obj(state: &Forest, obj: &Tree) -> Option<Int2D> {
        state.field.get_location(*obj)
    }
    fn get_rotation_obj(_state: &Forest, _obj: &Tree) -> f32 {
        0.0
    }
    fn get_dense_grid(state: &Forest) -> Option<&DenseGrid2D<Tree>> {
        Some(&state.field)
    }
    fn get_sparse_grid(_state: &Forest) -> Option<&SparseGrid2D<Tree>> {
        None
    }
}
