#![allow(non_snake_case)]

mod boid;

use bevy::prelude::*;
use boid::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.7, 0.7, 0.7)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(init)
        .add_startup_system(spawn_boids)
        .add_system_set(Boid::boid_system_group())
        .run();
}

fn init(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

