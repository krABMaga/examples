use std::any::Any;
use std::collections::VecDeque;
use std::f32::consts::TAU;

use krabmaga::engine::location::Real2D;
use krabmaga::engine::schedule::Schedule;
use krabmaga::engine::state::State;
use krabmaga::rand;
use krabmaga::Rng;

use crate::model::body::{Body, BodyData};

// to generate the angle and get the most irrational number
const GOLDEN_ANGLE: f32 = 2.399_963;

const STAR_RADIUS: f32 = 8.0;
/// A trail point is recorded every TRAIL_EVERY steps.
const TRAIL_EVERY: u64 = 2;

/// Initial configurations
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SetupMode {
    SolarSystem,
    TwoPlanets,
    Random,
    Symmetrical,
}

impl SetupMode {
    pub const ALL: [SetupMode; 4] = [
        SetupMode::SolarSystem,
        SetupMode::TwoPlanets,
        SetupMode::Random,
        SetupMode::Symmetrical,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            SetupMode::SolarSystem => "Star + circular orbits",
            SetupMode::TwoPlanets => "Two planets (stable)",
            SetupMode::Random => "Random (zero momentum)",
            SetupMode::Symmetrical => "Symmetrical (zero-sum)",
        }
    }
}

// Everything the menu can change.
// Fields marked "setup" are read only when the system is (re)generated,
// the others act immediately.
#[derive(Clone)]
pub struct Params {
    // --- setup ---
    pub mode: SetupMode,
    pub number: usize,
    pub star_mass: f32,
    pub max_planet_mass: f32,
    pub init_speed: f32,
    pub tracked: usize,
    // --- live ---
    pub g: f32,
    pub dt: f32,
    pub softening: f32,
    pub keep_centered: bool,
    pub show_trails: bool,
    pub trail_len: usize,
}

impl Default for Params {
    fn default() -> Self {
        Params {
            mode: SetupMode::SolarSystem,
            number: 30,
            star_mass: 1000.0,
            max_planet_mass: 10.0,
            init_speed: 20.0,
            tracked: 8,
            g: 100.0,
            dt: 0.05,
            softening: 5.0,
            keep_centered: true,
            show_trails: true,
            trail_len: 200,
        }
    }
}

pub struct NBodyState {
    pub p: Params,
    pub dim: (f32, f32),
    // Snapshot read by every agent during a step.
    pub bodies: Vec<BodyData>,
    // Buffer written by the agents during a step.
    pub next: Vec<BodyData>,
    // One trail per tracked body: (x, y) points, oldest first.
    pub trails: Vec<VecDeque<(f32, f32)>>,
    // Index of the body that owns trails[0] (1 when body 0 is the star).
    pub trail_first: usize,
    pub ticks: u64,
}

impl NBodyState {
    pub fn new(dim: (f32, f32)) -> Self {
        let mut s = NBodyState {
            p: Params::default(),
            dim,
            bodies: Vec::new(),
            next: Vec::new(),
            trails: Vec::new(),
            trail_first: 0,
            ticks: 0,
        };
        s.setup();
        s
    }

    fn planet_radius(mass: f32) -> f32 {
        2.0 + 0.6 * mass.sqrt()
    }

    // (Re)generate all the bodies from the current `Params`.
    pub fn setup(&mut self) {
        let (w, h) = self.dim;
        let (cx, cy) = (w / 2.0, h / 2.0);
        let p = self.p.clone();
        let mut rng = rand::rng();
        let mut bodies: Vec<BodyData> = Vec::new();

        let r_min = 40.0f32;
        let r_max = (w.min(h) / 2.0 - 20.0).max(r_min + 10.0);
        let eps2 = p.softening * p.softening;

        // Speed of a circular orbit of radius r around the (softened) star.
        let circular_speed = |r: f32| -> f32 {
            (p.g * p.star_mass * r * r / (r * r + eps2).powf(1.5))
                .max(0.0)
                .sqrt()
        };

        match p.mode {
            SetupMode::SolarSystem | SetupMode::TwoPlanets => {
                bodies.push(BodyData {
                    mass: p.star_mass,
                    radius: STAR_RADIUS,
                    pos: Real2D { x: cx, y: cy },
                    vel: Real2D { x: 0.0, y: 0.0 },
                    is_star: true,
                });

                let two = p.mode == SetupMode::TwoPlanets;
                let n_planets = if two { 2 } else { p.number.saturating_sub(1) };

                for i in 0..n_planets {
                    let (r, a, m) = if two {
                        (
                            r_max * (0.45 + 0.4 * i as f32),
                            std::f32::consts::PI * i as f32,
                            p.max_planet_mass * (1.0 - 0.5 * i as f32),
                        )
                    } else {
                        let t = (i as f32 + 1.0) / (n_planets as f32 + 1.0);
                        (
                            r_min + (r_max - r_min) * t.sqrt(),
                            (i as f32 + 1.0) * GOLDEN_ANGLE,
                            rng.random_range(1.0..p.max_planet_mass.max(1.5)),
                        )
                    };
                    let v = circular_speed(r);
                    bodies.push(BodyData {
                        mass: m,
                        radius: Self::planet_radius(m),
                        pos: Real2D {
                            x: cx + r * a.cos(),
                            y: cy + r * a.sin(),
                        },
                        vel: Real2D {
                            x: -a.sin() * v,
                            y: a.cos() * v,
                        },
                        is_star: false,
                    });
                }
            }

            SetupMode::Random | SetupMode::Symmetrical => {
                let sym = p.mode == SetupMode::Symmetrical;
                let n_gen = if sym {
                    (p.number / 2).max(1)
                } else {
                    p.number.max(2)
                };

                for _ in 0..n_gen {
                    let r = r_max * rng.random_range(0.0f32..1.0).sqrt();
                    let a = rng.random_range(0.0f32..TAU);
                    let speed = p.init_speed * rng.random_range(0.0f32..1.0);
                    let sa = rng.random_range(0.0f32..TAU);
                    let m = rng.random_range(p.star_mass * 0.02..p.star_mass * 0.2);
                    bodies.push(BodyData {
                        mass: m,
                        radius: Self::planet_radius(m),
                        pos: Real2D {
                            x: cx + r * a.cos(),
                            y: cy + r * a.sin(),
                        },
                        vel: Real2D {
                            x: speed * sa.cos(),
                            y: speed * sa.sin(),
                        },
                        is_star: false,
                    });
                }

                if sym {
                    // Mirror every body through the centre: total momentum = 0
                    // and the centre of mass stays in the middle.
                    let half = bodies.clone();
                    for b in half {
                        bodies.push(BodyData {
                            pos: Real2D {
                                x: 2.0 * cx - b.pos.x,
                                y: 2.0 * cy - b.pos.y,
                            },
                            vel: Real2D {
                                x: -b.vel.x,
                                y: -b.vel.y,
                            },
                            ..b
                        });
                    }
                } else {
                    // Remove the net momentum so the whole system doesn't drift.
                    let m_tot: f32 = bodies.iter().map(|b| b.mass).sum();
                    let px: f32 = bodies.iter().map(|b| b.mass * b.vel.x).sum::<f32>() / m_tot;
                    let py: f32 = bodies.iter().map(|b| b.mass * b.vel.y).sum::<f32>() / m_tot;
                    for b in bodies.iter_mut() {
                        b.vel.x -= px;
                        b.vel.y -= py;
                    }
                }
            }
        }

        let first = if bodies.first().map_or(false, |b| b.is_star) {
            1
        } else {
            0
        };
        let tracked = p.tracked.min(bodies.len().saturating_sub(first));

        self.trails = vec![VecDeque::new(); tracked];
        self.trail_first = first;
        self.next = bodies.clone();
        self.bodies = bodies;
        self.ticks = 0;
    }

    // keep-centered: move everything so the centre of mass
    // stays in the middle of the view.
    fn recenter(&mut self) {
        let m_tot: f32 = self.bodies.iter().map(|b| b.mass).sum();
        if m_tot.abs() < 1e-6 {
            return;
        }
        let cx = self.bodies.iter().map(|b| b.mass * b.pos.x).sum::<f32>() / m_tot;
        let cy = self.bodies.iter().map(|b| b.mass * b.pos.y).sum::<f32>() / m_tot;
        let tx = self.dim.0 / 2.0 - cx;
        let ty = self.dim.1 / 2.0 - cy;
        for b in self.bodies.iter_mut() {
            b.pos.x += tx;
            b.pos.y += ty;
        }
    }

    fn record_trails(&mut self) {
        let first = self.trail_first;
        let max = self.p.trail_len;
        for (k, trail) in self.trails.iter_mut().enumerate() {
            let b = &self.bodies[first + k];
            trail.push_back((b.pos.x, b.pos.y));
            while trail.len() > max {
                trail.pop_front();
            }
        }
    }
}

impl State for NBodyState {
    // It Registers an agent Body {id} for each body
    fn init(&mut self, schedule: &mut Schedule) {
        for id in 0..self.bodies.len() {
            schedule.schedule_repeating(Box::new(Body { id: id as u32 }), 0.0, 0);
        }
    }

    fn update(&mut self, _step: u64) {}

    // Called once per step, after ALL agents have stepped.
    // we copy `next` into `bodies`
    fn after_step(&mut self, _schedule: &mut Schedule) {
        self.bodies.copy_from_slice(&self.next);
        if self.p.keep_centered {
            self.recenter();
        }
        self.ticks += 1;
        if self.ticks % TRAIL_EVERY == 0 {
            self.record_trails();
        }
    }

    // Called by the "Stop" button of the default panel (and by my "Setup").
    fn reset(&mut self) {
        self.setup();
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn as_state(&self) -> &(dyn State + 'static) {
        self
    }

    fn as_state_mut(&mut self) -> &mut (dyn State + 'static) {
        self
    }
}
