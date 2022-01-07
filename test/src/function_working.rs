use crate::model::state::Flocker;

mod model;

use {
    rust_ab::engine::schedule::Schedule, rust_ab::engine::state::State, rust_ab::simulate,
    rust_ab::Info, rust_ab::ProgressBar, std::time::Duration,
};

// AWS specific import
use lambda_runtime::{handler_fn, Context, Error};
use serde_json::{json, Value};

pub static COHESION: f32 = 0.8;
pub static AVOIDANCE: f32 = 1.0;
pub static RANDOMNESS: f32 = 1.1;
pub static CONSISTENCY: f32 = 0.7;
pub static MOMENTUM: f32 = 1.0;
pub static JUMP: f32 = 0.7;
pub static DISCRETIZATION: f32 = 10.0 / 1.5;
pub static TOROIDAL: bool = true;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = handler_fn(func);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn func(event: Value, _: Context) -> Result<Value, Error> {
    let step = 10;

    let dim = (200., 200.);
    let num_agents = 100;
  
    let state = Flocker::new(dim, num_agents);
    simulate!(step, state, 1, Info::Normal);
    let check = event["check"].as_str().unwrap_or("unsuccess");

    Ok(json!({ "message": format!("The function was executed with {}!", check) }))
}
