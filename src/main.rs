#![allow(non_snake_case)]

mod boid;

use bevy::prelude::*;
use boid::*;
use bevy_egui::{egui, EguiContext};
use bevy_prototype_lyon::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.7, 0.7, 0.7)))
        .init_resource::<UIState>()
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_egui:: EguiPlugin)
        .add_plugin(ShapePlugin)
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
    boid_range: f32,
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
            boid_range: 100.,
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
    boid_query: Query<Entity, With<Boid>>,
    mut commands: Commands
) {
    egui::TopBottomPanel::bottom("ui bottom panel")
        .min_height(50.)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                ui.label("Number of boids: ");
                ui.add(egui::Slider::new(&mut ui_state.num_boids, 0..=500));
                ui.label("\tRun simulation: ");
                ui.checkbox(&mut ui_state.run, "");
                ui.label("\tBoid sight range: ");
                let range_slider = ui.add(egui::Slider::new(&mut ui_state.boid_range, 1.0..=500.));
                if range_slider.drag_started() || range_slider.changed() {
                    for boid in boid_query.into_iter() {
                        let shape = shapes::Circle {
                            radius: ui_state.boid_range,
                            center: bevy::math::vec2(0., 0.)
                        };
                        commands.entity(boid).despawn_descendants();
                        commands.entity(boid).with_children(|builder| {
                            builder.spawn(GeometryBuilder::build_as(
                                &shape,
                                DrawMode::Outlined {
                                    fill_mode: FillMode::color(Color::rgba(0., 0., 0., 0.)),
                                    outline_mode: StrokeMode::new(Color::BLACK, 0.5),
                                },
                                Transform::default(),
                            ));
                        });
                    }
                } else if range_slider.drag_released() {
                    for boid in boid_query.into_iter() {
                        commands.entity(boid).despawn_descendants();
                    }
                }
            });
            ui.horizontal(|ui| {
                ui.label("Avoid other boids: ");
                ui.checkbox(&mut ui_state.avoid_boids, "");
                ui.label("\tBoid avoid strength: ");
                ui.add(egui::Slider::new(&mut ui_state.avoid_boid_strength, 0.001..=0.25));
            });
            ui.horizontal(|ui| {
                ui.label("Stick together: ");
                ui.checkbox(&mut ui_state.stick_together, "");
                ui.label("\tStick together strength: ");
                ui.add(egui::Slider::new(&mut ui_state.stick_together_strength, 0.001..=0.25));
                
            });
            ui.horizontal(|ui| {
                ui.label("Follow others: ");
                ui.checkbox(&mut ui_state.follow_others, "");
                ui.label("\tFollow others strength: ");
                ui.add(egui::Slider::new(&mut ui_state.follow_others_strength, 0.001..=0.25));
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