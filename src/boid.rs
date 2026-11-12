use std::f32::consts::{FRAC_PI_2, TAU};

use crate::*;
use bevy::math::bounding::{BoundingCircle, IntersectsVolume, RayCast2d};
use rand::{self, RngExt};

#[derive(Component, Default)]
#[require(
    Sprite,
    Transform = Transform::from_scale(Vec3::new(0.04, 0.05, 1.))
)]
pub struct Boid {
    prev_velocity: Vec2,
    velocity: Vec2
}


pub fn on_add_boid(
    on: On<Add, Boid>,
    settings: Res<SimulationSettings>,
    mut boid_query: Query<&mut Sprite, With<Boid>>,
) {
    let Ok(mut boid_sprite) = boid_query.get_mut(on.entity) else { return };
    boid_sprite.image = settings.boid_texture.clone().unwrap();
}


pub fn spawn_startup_boids(
    mut commands: Commands,
    settings: Res<SimulationSettings>
) {
    commands.spawn_batch([
        (Boid::default(), WithPos::Position(Vec2::new(-4.5, 0.)), WithVel::Velocity(Vec2::new(0., 0.1))),
        (Boid::default(), WithPos::Position(Vec2::new(4.5, 0.)), WithVel::Velocity(Vec2::new(0., 0.1)))
    ].into_iter());


    //let iter = (0..settings.num_boids).map(|_| {
    //    (Boid::default(), WithPos::Random, WithVel::Random)
    //});
    //commands.spawn_batch(iter);
}

pub fn move_boids(
    boid_query: Query<(&Boid, &mut Transform)>,
    camera_query: Query<(&Camera, &GlobalTransform)>
) {
    let Ok((cam, cam_gtrans)) = camera_query.single() else { return };
    let Some(min) = cam.ndc_to_world(cam_gtrans, Vec3A::new(-1., -1., 0.)) else { return };
    let Some(max) = cam.ndc_to_world(cam_gtrans, Vec3A::new(1., 1., 0.)) else { return };
    let size = max - min;

    for (boid, mut transform) in boid_query {
        transform.translation += boid.velocity.extend(0.);

        if transform.translation.x < min.x {
            transform.translation.x += size.x;
        } else if transform.translation.x > max.x {
            transform.translation.x -= size.x
        }
        if transform.translation.y < min.y {
            transform.translation.y += size.y;
        } else if transform.translation.y > max.y {
            transform.translation.y -= size.y
        }
    }
}

pub fn rotate_boids(
    boid_query: Query<(&Boid, &mut Transform), Changed<Boid>>
) {
    for (boid, mut trans) in boid_query {
        let angle = boid.velocity.to_angle();
        *trans = trans.with_rotation(Quat::from_rotation_z(angle - FRAC_PI_2));
    }
}


#[derive(Component)]
pub enum WithPos {
    Random,
    Position(Vec2)
}
#[derive(Component)]
pub enum WithVel {
    Random,
    Velocity(Vec2)
}

pub fn on_random_pos(
    on: On<Insert, WithPos>,
    mut commands: Commands,
    mut boid_query: Query<(&mut Transform, &WithPos), With<Boid>>,
    camera_query: Query<(&Camera, &GlobalTransform)>
) {
    let Ok((mut transform, withpos)) = boid_query.get_mut(on.entity) else { return };
    
    match withpos {
        WithPos::Random => {
            let Ok((cam, cam_gtrans)) = camera_query.single() else { return };
            let Some(min) = cam.ndc_to_world(cam_gtrans, Vec3A::new(-1., -1., 0.)) else { return };
            let Some(max) = cam.ndc_to_world(cam_gtrans, Vec3A::new(1., 1., 0.)) else { return };

            let mut rng = rand::rng();
            let x = rng.random_range(min.x..max.x);
            let y = rng.random_range(min.y..max.y);

            transform.translation = Vec3::new(x, y, 0.);
        },
        WithPos::Position(p) => {
            transform.translation = p.extend(0.);
        }
    }

    commands.get_entity(on.entity).unwrap().remove::<WithPos>();
    
}
pub fn on_random_vel(
    on: On<Insert, WithVel>,
    mut commands: Commands,
    mut boid_query: Query<(&mut Boid, &WithVel)>
) {
    let Ok((mut boid, withvel)) = boid_query.get_mut(on.entity) else { return };

    match withvel {
        WithVel::Random => {
            let mut rng = rand::rng();
            let angle = rng.random_range(0.0..(TAU));
            let vel = Vec2::from_angle(angle) * 1.;
            boid.velocity = vel;
            boid.prev_velocity = boid.velocity;
        },
        WithVel::Velocity(v) => {
            boid.velocity = *v;
            boid.prev_velocity = boid.velocity;
        },
    }

    commands.get_entity(on.entity).unwrap().remove::<WithVel>();
}



pub fn steer_boids(
    mut boid_query: Query<(&mut Boid, &Transform)>,
    settings: Res<SimulationSettings>
) {
    let mut iter = boid_query.iter_combinations_mut();
    while let Some([boid1, boid2]) = iter.fetch_next() {
        steer_boid(boid1, boid2, &settings);
    }

    let iter = boid_query.iter_mut();
    for (mut boid, _)  in iter {
        boid.velocity = boid.prev_velocity.rotate_towards(boid.velocity, 0.1);
        boid.prev_velocity = boid.velocity;
    }
}


fn steer_boid(
    mut boid1: (Mut<Boid>, &Transform),
    mut boid2: (Mut<Boid>, &Transform),
    settings: &SimulationSettings
) {
    let distance_squared = boid1.1.translation.distance_squared(boid2.1.translation);
    if distance_squared > settings.boid_range.powi(2) {
        return
    }

    avoid_boids(&mut boid1.0, &boid1.1.translation, &mut boid2.0, &boid2.1.translation, settings, distance_squared);
    follow_boids(&mut boid1.0, &mut boid2.0, settings);
    stick_together(&mut boid1.0, &boid1.1.translation, &mut boid2.0, &boid2.1.translation, settings);
    
    
}

fn avoid_boids(
    boid1: &mut Boid,
    pos1: &Vec3,
    boid2: &mut Boid,
    pos2: &Vec3,
    settings: &SimulationSettings,
    distance_squared: f32,
) {
    let raycast = RayCast2d::new(pos1.truncate(), Dir2::new(boid1.velocity).unwrap(), settings.boid_range);
    let intersects = raycast.intersects(&BoundingCircle::new(pos2.truncate(), 10.));
    if intersects || distance_squared < 500. {
        boid1.velocity = boid1.velocity.rotate_towards((pos1-pos2).truncate(), settings.avoid_boid_strength);
    }

    let raycast = RayCast2d::new(pos2.truncate(), Dir2::new(boid2.velocity).unwrap(), settings.boid_range);
    let intersects = raycast.intersects(&BoundingCircle::new(pos1.truncate(), 10.));
    if intersects || distance_squared < 500. {
        boid2.velocity = boid2.velocity.rotate_towards((pos2-pos1).truncate(), settings.avoid_boid_strength);
    }
}

fn stick_together(
    boid1: &mut Boid,
    pos1: &Vec3,
    boid2: &mut Boid,
    pos2: &Vec3,
    settings: &SimulationSettings
) {
    boid1.velocity = boid1.velocity.rotate_towards((pos2-pos1).truncate(), settings.stick_together_strength);
    boid2.velocity = boid2.velocity.rotate_towards((pos1-pos2).truncate(), settings.stick_together_strength);
}

fn follow_boids(
    boid1: &mut Boid,
    boid2: &mut Boid,
    settings: &SimulationSettings
) {
    boid1.velocity = boid1.velocity.rotate_towards(boid2.velocity, settings.follow_others_strength);
    boid2.velocity = boid2.velocity.rotate_towards(boid1.velocity, settings.follow_others_strength);
}