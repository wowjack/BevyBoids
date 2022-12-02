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

    use bevy::math::vec2;
    commands.spawn(
        BoidBundle::with_velocity_and_position(
            boid_texture.clone(),
            vec2(0.5, 0.2),
            vec2(-200., 0.)
        )
    );
    commands.spawn(
        BoidBundle::with_velocity_and_position(
            boid_texture.clone(),
            vec2(-0.3, -0.2),
            vec2(200., -100.)
        )
    );
}

fn move_boids(mut boid_query: Query<(&Boid, &mut Transform)>) {
    for (boid, mut boid_transform) in boid_query.iter_mut() {
        boid_transform.translation += Vec3::from((boid.velocity, 0.));
    }
}

