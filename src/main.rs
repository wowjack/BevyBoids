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

    commands.spawn(BoidBundle::from_texture(boid_texture.clone()));

    commands.spawn(
        BoidBundle {
            boid: Boid { velocity: Vec2{ x: 0.5, y: 0.1 }, ..Boid::new() },
            ..BoidBundle::from_texture_and_position(boid_texture.clone(), Vec2{ x:-200., y:0.})
        }
    );
}

fn move_boids(mut boid_query: Query<(&Boid, &mut Transform)>) {
    for (boid, mut boid_transform) in boid_query.iter_mut() {
        boid_transform.translation.x += boid.velocity.x as f32;
        boid_transform.translation.y += boid.velocity.y as f32;
    }
}

