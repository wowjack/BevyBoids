#![allow(non_snake_case)]

mod boid;
mod dark_theme;
mod value_slider;
mod util;

use bevy::{color::palettes::tailwind::GRAY_600, feathers::{FeathersPlugins, controls::checkbox, theme::UiTheme}, prelude::*, ui::Checked, ui_widgets::{ValueChange, checkbox_self_update, observe}};
use boid::*;

use crate::{dark_theme::create_dark_theme, util::{update_avoid, update_boid_range, update_follow, update_num_boids, update_run_simulation, update_stick_together}, value_slider::value_slider};

const DEFAULT_RUN_STATE: bool = true;

const DEFAULT_NUM_BOIDS: u16 = 100;
const MIN_NUM_BOIDS: u16 = 0;
const MAX_NUM_BOIDS: u16 = 1000;

const DEFAULT_BOID_RANGE: f32 = 100.;
const MIN_BOID_RANGE: f32 = 1.;
const MAX_BOID_RANGE: f32 = 1000.;

const DEFAULT_AVOID_BOIDS: bool = true;
const DEFAULT_AVOID_BOID_STRENGTH: f32 = 0.02;
const MIN_AVOID_BOID_STRENGTH: f32 = 0.;
const MAX_AVOID_BOID_STRENGTH: f32 = 0.1;

const DEFAULT_STICK_TOGETHER: bool = true;
const DEFAULT_STICK_TOGETHER_STRENGTH: f32 = 0.01;
const MIN_STICK_TOGETHER_STRENGTH: f32 = 0.;
const MAX_STICK_TOGETHER_STRENGTH: f32 = 0.1;

const DEFAULT_FOLLOW_OTHERS: bool = true;
const DEFAULT_FOLLOW_OTHERS_STRENGTH: f32 = 0.05;
const MIN_FOLLOW_OTHERS_STRENGTH: f32 = 0.;
const MAX_FOLLOW_OTHERS_STRENGTH: f32 = 0.1;


fn main() {
    App::new()
        .insert_resource(ClearColor(GRAY_600.into()))
        .insert_resource(UiTheme(create_dark_theme()))
        .init_resource::<SimulationSettings>()
        .add_plugins((DefaultPlugins, FeathersPlugins))
        .add_systems(Startup, init)
        .add_systems(PostStartup, spawn_startup_boids)
        .add_systems(Update, (move_boids, steer_boids, rotate_boids))
        .add_observer(on_add_boid)
        .add_observer(on_random_pos)
        .add_observer(on_random_vel)
        .run();
}


fn init(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut settings: ResMut<SimulationSettings>,
) {
    commands.spawn(Camera2d);

    let handle: Handle<Image> = assets.load("boid.png");
    settings.boid_texture = Some(handle);

    commands.spawn((
        Node {
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(30.),
            width:  Val::Vw(100.),
            ..default()
        },
        children![
            (
                Node {
                    width: Val::Vw(30.),
                    height: Val::Vh(100.),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::FlexEnd,
                    align_content: AlignContent::Start,
                    row_gap: Val::Percent(2.),
                    padding: UiRect::all(Val::Px(10.)),
                    ..default()
                },
                children![
                    (checkbox(Checked, Spawn(Text("Run".into()))), observe(update_run_simulation), observe(checkbox_self_update)),
                    (value_slider("# Boids:", MIN_NUM_BOIDS as f32, MAX_NUM_BOIDS as f32, DEFAULT_NUM_BOIDS as f32, 0, 1.), observe(update_num_boids)),
                    (value_slider("Boid range:", MIN_BOID_RANGE as f32, MAX_BOID_RANGE as f32, DEFAULT_BOID_RANGE as f32, 0, 1.), observe(update_boid_range))
                ]
            ),
            (
                Node {
                    width: Val::Vw(50.),
                    height: Val::Vh(100.),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::FlexEnd,
                    align_content: AlignContent::Start,
                    row_gap: Val::Percent(2.),
                    padding: UiRect::all(Val::Px(10.)),
                    ..default()
                },
                children![
                    (value_slider("Avoid:", MIN_AVOID_BOID_STRENGTH as f32, MAX_AVOID_BOID_STRENGTH as f32, DEFAULT_AVOID_BOID_STRENGTH as f32, 3, 0.001), observe(update_avoid)),
                    (value_slider("Follow:", MIN_FOLLOW_OTHERS_STRENGTH as f32, MAX_FOLLOW_OTHERS_STRENGTH as f32, DEFAULT_FOLLOW_OTHERS_STRENGTH as f32, 3, 0.001), observe(update_follow)),
                    (value_slider("Stick:", MIN_STICK_TOGETHER_STRENGTH as f32, MAX_STICK_TOGETHER_STRENGTH as f32, DEFAULT_STICK_TOGETHER_STRENGTH as f32, 3, 0.001), observe(update_stick_together))
                ]
            )
        ]
    ));
}

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
    boid_texture: Option<Handle<Image>>,
}
impl Default for SimulationSettings {
    fn default() -> Self {
        Self {
            run: DEFAULT_RUN_STATE,
            boid_range: DEFAULT_BOID_RANGE,
            avoid_boids: DEFAULT_AVOID_BOIDS,
            avoid_boid_strength: DEFAULT_AVOID_BOID_STRENGTH,
            stick_together: DEFAULT_STICK_TOGETHER,
            stick_together_strength: DEFAULT_STICK_TOGETHER_STRENGTH,
            follow_others: DEFAULT_FOLLOW_OTHERS,
            follow_others_strength: DEFAULT_FOLLOW_OTHERS_STRENGTH,
            num_boids: DEFAULT_NUM_BOIDS,
            boid_texture: None,
        }
    }
}

