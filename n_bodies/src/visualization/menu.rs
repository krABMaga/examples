use bevy_egui::{egui, EguiContexts};
use krabmaga::bevy::prelude::{Commands, Entity, Query, Res, ResMut, With};
use krabmaga::engine::schedule::Schedule;
use krabmaga::engine::state::State;
use krabmaga::visualization::asset_handle_factory::AssetHandleFactoryResource;
use krabmaga::visualization::simulation_descriptor::SimulationDescriptor;
use krabmaga::visualization::visualization_state::VisualizationState;
use krabmaga::visualization::wrappers::{ActiveSchedule, ActiveState, SimulationRenderEntity};

use crate::model::state::{NBodyState, SetupMode};
use crate::visualization::vis_state::NBodyVis;

/// Floating control window (the default krABMaga panel on the left keeps
/// Start / Pause / Stop and the "steps per second" slider).
pub fn menu_system(
    mut egui_context: EguiContexts,
    mut sim_data: ResMut<SimulationDescriptor>,
    active_schedule: Res<ActiveSchedule>,
    active_state: Res<ActiveState<NBodyState>>,
    on_init: Res<NBodyVis>,
    mut sprite_factory: AssetHandleFactoryResource,
    query: Query<Entity, With<SimulationRenderEntity>>,
    mut commands: Commands,
) {
    let Ok(ctx) = egui_context.ctx_mut() else {
        return;
    };

    let mut do_setup = false;

    {
        let mut s = active_state.0.lock().expect("error on lock");
        let n = s.bodies.len();
        let (w, h) = s.dim;
        let outside = s
            .bodies
            .iter()
            .filter(|b| b.pos.x < 0.0 || b.pos.x > w || b.pos.y < 0.0 || b.pos.y > h)
            .count();
        let p = &mut s.p;

        egui::Window::new("N-Bodies")
            .default_pos(egui::pos2(320.0, 12.0))
            .show(ctx, |ui| {
                ui.label(format!("Bodies: {n}    outside the view: {outside}"));
                ui.separator();

                ui.strong("Initial conditions (apply with Setup)");
                egui::ComboBox::from_label("Configuration")
                    .selected_text(p.mode.label())
                    .show_ui(ui, |ui| {
                        for m in SetupMode::ALL {
                            ui.selectable_value(&mut p.mode, m, m.label());
                        }
                    });
                ui.add(egui::Slider::new(&mut p.number, 2..=300).text("Number of bodies"));
                ui.add(egui::Slider::new(&mut p.star_mass, 10.0..=5000.0).text("Central mass"));
                ui.add(
                    egui::Slider::new(&mut p.max_planet_mass, 1.0..=100.0).text("Max planet mass"),
                );
                ui.add(
                    egui::Slider::new(&mut p.init_speed, 0.0..=60.0).text("Initial speed (random)"),
                );
                ui.add(egui::Slider::new(&mut p.tracked, 0..=50).text("Bodies with trail"));

                if ui.button("⟲  Setup").clicked() {
                    do_setup = true;
                }
                ui.label("then press ▶ Start in the left panel");

                ui.separator();
                ui.strong("Live parameters");
                ui.add(
                    egui::Slider::new(&mut p.g, -300.0..=500.0).text("G (negative = repulsive)"),
                );
                ui.add(
                    egui::Slider::new(&mut p.dt, 0.005..=0.2)
                        .logarithmic(true)
                        .text("dt"),
                );
                ui.add(egui::Slider::new(&mut p.softening, 0.0..=30.0).text("Softening"));
                ui.checkbox(&mut p.keep_centered, "Keep centered (centre of mass)");
                ui.checkbox(&mut p.show_trails, "Show trails");
                ui.add(egui::Slider::new(&mut p.trail_len, 0..=1000).text("Trail length"));
            });
    }

    if do_setup {
        // Same sequence used by the "Stop" button of krABMaga's default panel.
        sim_data.paused = true;
        for entity in query.iter() {
            commands.entity(entity).despawn();
        }

        let mut new_schedule = Schedule::new();
        {
            let mut s = active_state.0.lock().expect("error on lock");
            s.reset();
            s.init(&mut new_schedule);
            on_init.on_init(
                &mut commands,
                &mut sprite_factory,
                &mut s,
                &mut new_schedule,
                &mut *sim_data,
            );
            on_init.setup_graphics(&mut new_schedule, &mut commands, &mut s, sprite_factory);
        }
        *active_schedule.0.lock().expect("error on lock") = new_schedule;
    }
}
