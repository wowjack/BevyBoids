#![allow(non_snake_case)]

mod boid;

use bevy::prelude::*;
use boid::*;
use bevy_egui::{egui, EguiContext, EguiPlugin, EguiSettings};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.7, 0.7, 0.7)))
        .init_resource::<UIState>()
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_egui:: EguiPlugin)
        .add_startup_system(init)
        .add_startup_system(spawn_boids)
        .add_system_set(Boid::boid_system_group())
        .add_system(ui_example)
        .add_system(handle_num_boids_changes)
        .run();
}

fn init(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Resource)]
pub struct UIState {
    run: bool,
    avoid_boids: bool,
    avoid_boid_strength: f32,
    stick_together: bool,
    stick_together_strength: f32,
    follow_others: bool,
    follow_others_strength: f32,
    num_boids: u16,
    prev_num_boids: u16,
    boid_texture: Option<Handle<Image>>,
}
impl Default for UIState {
    fn default() -> Self {
        Self {
            run: true,
            avoid_boids: true,
            avoid_boid_strength: 0.02,
            stick_together: true,
            stick_together_strength: 0.01,
            follow_others: true,
            follow_others_strength: 0.05,
            num_boids: 100,
            prev_num_boids: 100,
            boid_texture: None,
        }
    }
}

fn ui_example(
    mut egui_ctx: ResMut<EguiContext>,
    mut ui_state: ResMut<UIState>,
) {
    egui::TopBottomPanel::bottom("ui bottom panel")
        .min_height(50.)
        .show(egui_ctx.ctx_mut(), |ui| {

            ui.horizontal(|ui| {
                ui.label("Number of boids: ");
                ui.add(egui::Slider::new(&mut ui_state.num_boids, 0..=500));
                ui.label("\tRun simulation: ");
                ui.checkbox(&mut ui_state.run, "");
            });
            ui.horizontal(|ui| {
                ui.label("Avoid other boids: ");
                ui.checkbox(&mut ui_state.avoid_boids, "");
                ui.label("\tBoid avoid strength: ");
                ui.add(egui::Slider::new(&mut ui_state.avoid_boid_strength, 0.001..=1.0));
            });
            ui.horizontal(|ui| {
                ui.label("Stick together: ");
                ui.checkbox(&mut ui_state.stick_together, "");
                ui.label("\tStick together strength: ");
                ui.add(egui::Slider::new(&mut ui_state.stick_together_strength, 0.001..=1.0));
                
            });
            ui.horizontal(|ui| {
                ui.label("Follow others: ");
                ui.checkbox(&mut ui_state.follow_others, "");
                ui.label("\tFollow others strength: ");
                ui.add(egui::Slider::new(&mut ui_state.follow_others_strength, 0.001..=1.0).text("value"));
            });
        });
}

fn handle_num_boids_changes(
    mut ui_state: ResMut<UIState>,
    boid_entity_query: Query<Entity, With<Boid>>,
    mut commands: Commands,
    windows: Res<Windows>
) {
    if ui_state.num_boids == ui_state.prev_num_boids {return}

    let window = windows.get_primary().unwrap();
    
    if ui_state.num_boids < ui_state.prev_num_boids {
        for (num, entity) in boid_entity_query.into_iter().enumerate() {
            if num < ui_state.num_boids.into() {continue}
            commands.entity(entity).despawn();
        }
    } else {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        for _ in 0..(ui_state.num_boids - ui_state.prev_num_boids) {
            let xvel = rng.gen_range(0.3..2.0) * if rand::random() {-1.} else {1.};
            let yvel = rng.gen_range(0.3..2.0) * if rand::random() {-1.} else {1.};
            let xpos = rng.gen_range(window.width()/-2.0 .. window.width()/2.0);
            let ypos = rng.gen_range(window.height()/-2.0 .. window.height()/2.0);
            commands.spawn(
            BoidBundle::with_velocity_and_position(
                ui_state.boid_texture.as_ref().unwrap().clone(),
                bevy::math::vec2(xvel, yvel),
                bevy::math::vec2(xpos, ypos)
            )
        );
        }
    }
    ui_state.prev_num_boids = ui_state.num_boids;
}