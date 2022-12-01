#![allow(unused)]

use bevy::prelude::*;

#[derive(Component, PartialEq, Debug)]
pub struct Boid {
    pub uid: uid::Id<Boid>,
    pub velocity: Vec2,
}
impl Boid {
    pub fn new() -> Self {
        Boid {
            uid: uid::Id::new(),
            velocity: Vec2::default()
        }
    }
}


#[derive(Bundle)]
pub struct BoidBundle {
    pub sprite: SpriteBundle,
    pub boid: Boid
}
impl BoidBundle {
    pub fn from_texture(texture: Handle<Image>) -> Self {
        Self {
            sprite: SpriteBundle {
                texture,
                sprite: Sprite {
                    custom_size: Some(Vec2{ x:15., y:20.}),
                    ..default()
                },
                ..default()
            },
            boid: Boid::new()
        }
    }

    pub fn from_texture_and_position(texture: Handle<Image>, position: Vec2) -> Self {
        Self {
            sprite: SpriteBundle {
                texture,
                sprite: Sprite {
                    custom_size: Some(Vec2{ x:15., y:20.}),
                    ..default()
                },
                transform: Transform { translation: Vec3 { x: position.x, y: position.y, z: 0.}, ..default() },
                ..default()
            },
            boid: Boid::new()
        }
    }
}