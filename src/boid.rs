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
    pub fn new(texture: Handle<Image>) -> Self {
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

    pub fn with_position(texture: Handle<Image>, position: Vec2) -> Self {
        let mut new_boid_bundle: Self = BoidBundle::new(texture);
        new_boid_bundle.sprite.transform.translation = Vec3::from((position, 0.));
        return new_boid_bundle;
    }

    pub fn with_velocity(texture: Handle<Image>, velocity: Vec2) -> Self {
        //rotation must be set such that the boid looks like the boid moves forward
        let mut new_boid_bundle: Self = Self::new(texture);
        new_boid_bundle.boid.velocity = velocity;
        new_boid_bundle.sprite.transform.rotation = Quat::from_rotation_z(velocity.y.atan2(velocity.x) - std::f32::consts::FRAC_PI_2);
        return new_boid_bundle;
    }

    pub fn with_velocity_and_position(texture: Handle<Image>, velocity: Vec2, position: Vec2) -> Self {
        let mut new_boid_bundle: Self = Self::with_velocity(texture, velocity);
        new_boid_bundle.sprite.transform.translation = Vec3::from((position, 0.));
        return new_boid_bundle;
    }
}