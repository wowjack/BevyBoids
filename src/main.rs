#![allow(non_snake_case)]

use bevy::prelude::*;
use boid::*;

mod boid;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(init)
        .add_startup_system(spawn_boids)
        .add_system(move_boids)
        .run();
}

fn init(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_boids(mut commands: Commands, assets: Res<AssetServer>) {
    let boid_texture: Handle<Image> = assets.load("boid.png");

    commands.spawn(BoidBundle::from_texture(boid_texture));
}

fn move_boids(mut boid_query: Query<(&Boid, &mut Transform)>) {
    for (boid, mut boid_transform) in boid_query.iter_mut() {
        boid_transform.translation.x += boid.velocity.0 as f32;
        boid_transform.translation.y += boid.velocity.1 as f32;
    }
}

