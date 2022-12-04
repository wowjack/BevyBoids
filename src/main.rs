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
        .add_system(boid_window_border_wraparound)
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
            vec2(1., 0.4),
            vec2(-200., 0.)
        )
    );
    commands.spawn(
        BoidBundle::with_velocity_and_position(
            boid_texture.clone(),
            vec2(-0.8, -0.4),
            vec2(200., -100.)
        )
    );
}

fn move_boids(mut boid_query: Query<(&Boid, &mut Transform)>) {
    for (boid, mut boid_transform) in boid_query.iter_mut() {
        boid_transform.translation += Vec3::from((boid.velocity, 0.));
    }
}

fn boid_window_border_wraparound(mut boid_query: Query<(&mut Transform, &Sprite), With<Boid>>, windows: Res<Windows>) {
    let window = windows.get_primary().unwrap();
    for (mut boid_transform, sprite) in boid_query.iter_mut() {
        if boid_transform.translation.x+(sprite.custom_size.unwrap().x/2.) < -1.*window.width()/2.
           || boid_transform.translation.x-(sprite.custom_size.unwrap().x/2.) > window.width()/2.
        {
            boid_transform.translation.x *= -1.;
        }
        if boid_transform.translation.y+(sprite.custom_size.unwrap().y/2.) < -1.*window.height()/2.
           || boid_transform.translation.y-(sprite.custom_size.unwrap().y/2.) > window.height()/2.
        {
            boid_transform.translation.y *= -1.;
        }
    }
}

