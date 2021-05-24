use rust_ab::engine::state::State;

/// Expand the state definition according to your model, for example by having a grid struct field to
/// store the agents' positions.
pub struct MyState{ pub step:u128 }

impl MyState{
    pub fn new()->MyState{
        MyState{step:0}
    }
}

impl State for MyState {
    /// Put the code that should be executed for each state update here. The state is updated once for each
    /// schedule step.
    fn update(&mut self, _step: usize) {

    }
}