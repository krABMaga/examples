use krabmaga::engine::agent::Agent;
use krabmaga::engine::location::Real2D;
use krabmaga::engine::state::State;

use crate::model::state::NBodyState;

//Physical Data of one body -> it lives in the state so that it can be modified and read by the menu and visualization
#[derive(Clone, Copy)]
pub struct BodyData {
    pub mass: f32,
    pub radius: f32,
    pub pos: Real2D,
    pub vel: Real2D,
    pub is_star: bool,
}

/// The agent only knows its index: it reads the snapshot `state.bodies`
/// (positions at the beginning of the step) and writes its new data into
/// `state.next`. `NBodyState::after_step` then publishes `next` -> `bodies`,
/// so every body sees the same instant (synchronous update).
#[derive(Clone, Copy)]
pub struct Body {
    pub id: u32,
}

impl Agent for Body {
    fn step(&mut self, state: &mut dyn State) {
        let s = state
            .as_any_mut()
            .downcast_mut::<NBodyState>()
            .expect("state is not NBodyState");

        let i = self.id as usize;
        let me = s.bodies[i];
        let eps2 = s.p.softening * s.p.softening;

        // Acceleration = sum_j G * m_j * (r_j - r_i) / (|r|^2 + eps^2)^(3/2)
        // (my own mass cancels out: F = m*a)
        let mut ax = 0.0f32;
        let mut ay = 0.0f32;

        for (j, other) in s.bodies.iter().enumerate() {
            if j == i {
                continue;
            }
            let dx = other.pos.x - me.pos.x;
            let dy = other.pos.y - me.pos.y;
            let d2 = dx * dx + dy * dy + eps2;
            if d2 < 1e-9 {
                continue;
            }
            let k = s.p.g * other.mass / (d2 * d2.sqrt());
            ax += k * dx;
            ay += k * dy;
        }

        // Semi-implicit Euler: v first, then x with the NEW v.
        let dt = s.p.dt;
        let mut new = me;
        new.vel.x += ax * dt;
        new.vel.y += ay * dt;
        new.pos.x += new.vel.x * dt;
        new.pos.y += new.vel.y * dt;

        s.next[i] = new;
    }
}
