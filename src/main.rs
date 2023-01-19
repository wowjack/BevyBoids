#![allow(non_snake_case)]

mod boid;

use bevy::prelude::*;
use boid::*;
use bevy_egui::{egui, EguiContext};
use bevy_prototype_lyon::prelude::*;

pub const GUI_PANEL_HEIGHT: f32 = 90.;

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
enum AppState {
    Opening,
    Running
}

fn main() {
    App::new()
        .add_state(AppState::Opening)
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .init_resource::<SimulationSettings>()
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_egui:: EguiPlugin)
        .add_plugin(ShapePlugin)
        .add_startup_system(init)
        .add_system_set(
            SystemSet::on_enter(AppState::Opening).with_system(open_text)
        )
        .add_system_set(
            SystemSet::on_update(AppState::Opening).with_system(intro_timer)
        )
        .add_system_set(
            SystemSet::on_enter(AppState::Running).with_system(spawn_boids).with_system(remove_intro)
        )
        .add_system_set(
            Boid::boid_system_group().with_system(ui).with_system(handle_num_boids_changes)
        )
        .run();
}


fn init(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}


/////////////////// TITLE SCREEN SYSTEMS ///////////////////

fn open_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/Arial.ttf");
    commands.spawn(Text2dBundle {
        text: Text::from_section("BOIDS", TextStyle {
            font: font.clone(),
            font_size: 120.,
            color: Color::WHITE
            }).with_alignment(TextAlignment::CENTER),
        transform: Transform {
            scale: bevy::math::vec3(1., 1., 1.),
            ..default()
        },
        ..default()
    });

    commands.spawn(Text2dBundle {
        text: Text::from_section("Jack Kingham", TextStyle {
            font: font.clone(),
            font_size: 20.,
            color: Color::WHITE
            }).with_alignment(TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center
            }),
        transform: Transform {
            translation: bevy::math::vec3(0., -85., 0.),
            scale: bevy::math::vec3(1., 1., 1.),
            ..default()
        },
        ..default()
    });
}

fn intro_timer(mut app_state: ResMut<State<AppState>>, time: Res<Time>) {
    if time.elapsed_seconds() > 3. {
        app_state.set(AppState::Running).expect("Changing app state from intro to running failed.");
    }
}

fn remove_intro(query: Query<Entity, With<Text>>, mut commands: Commands, mut bg_color: ResMut<ClearColor>) {
    for text in query.into_iter() {
        commands.entity(text).despawn();
    }
    bg_color.0 = Color::rgb(0.6, 0.6, 0.6);
}


/////////////////// GUI SYSTEMS ///////////////////

#[derive(Resource)]
pub struct SimulationSettings {
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
impl Default for SimulationSettings {
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

fn ui(
    mut egui_ctx: ResMut<EguiContext>,
    mut ui_state: ResMut<SimulationSettings>,
    boid_query: Query<Entity, With<Boid>>,
    mut commands: Commands
) {
    let _gui = egui::TopBottomPanel::bottom("ui bottom panel")
        .height_range(GUI_PANEL_HEIGHT..=GUI_PANEL_HEIGHT)
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
    mut ui_state: ResMut<SimulationSettings>,
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
        commands.spawn_batch(BoidBundle::random_boids(&ui_state.boid_texture.as_ref().unwrap().clone(), (ui_state.num_boids-ui_state.prev_num_boids).into(), window));
    }
    ui_state.prev_num_boids = ui_state.num_boids;
}