#![allow(unused)]

use bevy::prelude::*;

#[derive(Component, PartialEq, Eq, Debug)]
pub struct Boid {
    uid: uid::Id<Boid>,
    pub velocity: (i32, i32),
}
impl Boid {
    pub fn new() -> Self {
        Boid {
            uid: uid::Id::new(),
            velocity: (1, 1)
        }
    }
}


#[derive(Bundle)]
pub struct BoidBundle {
    sprite: SpriteBundle,
    boid: Boid
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
                ..default()
            },
            boid: Boid::new()
        }
    }
}