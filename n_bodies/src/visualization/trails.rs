use krabmaga::bevy::prelude::{Gizmos, Res, Vec2};
use krabmaga::visualization::wrappers::ActiveState;

use crate::model::state::NBodyState;
use crate::visualization::colors::body_color;

// Bevy system (runs every frame): draws every trail as a polyline
// whose segments fade from transparent (oldest) to opaque (newest).
pub fn draw_trails(mut gizmos: Gizmos, state: Res<ActiveState<NBodyState>>) {
    let s = state.0.lock().expect("error on lock");
    if !s.p.show_trails {
        return;
    }

    for (k, trail) in s.trails.iter().enumerate() {
        let n = trail.len();
        if n < 2 {
            continue;
        }
        let id = (s.trail_first + k) as u32;

        let mut prev: Option<Vec2> = None;
        for (i, &(x, y)) in trail.iter().enumerate() {
            // +0.5: the framework draws sprites half a unit higher
            let point = Vec2::new(x, y + 0.5);
            if let Some(q) = prev {
                let alpha = 0.05 + 0.85 * (i as f32 / n as f32);
                gizmos.line_2d(q, point, body_color(id, alpha));
            }
            prev = Some(point);
        }
    }
}
