#![allow(unused)]

use bevy::prelude::*;

const NUM_BOIDS: u8 = 50;
const BOID_RANGE: f32 = 100.;

#[derive(Component, PartialEq, Debug)]
pub struct Boid {
    pub uid: uid::Id<Boid>,
    pub velocity: Vec2,
}
impl Boid {
    pub fn new() -> Self {
        Boid {
            uid: uid::Id::new(),
            velocity: Vec2{ x: 0., y: 0.}
        }
    }

    pub fn boid_system_group() -> SystemSet {
        //Boids must be spawned using spawn_boids first
        SystemSet::new()
            .label("Boid Systems")
            .with_system(move_boids)
            .with_system(boid_window_border_wraparound)
            .with_system(boid_avoid_others)
            .with_system(boid_follow_others)
            .with_system(boid_stick_together)
    }

    pub fn steer_towards_point(boid: &mut core::cell::RefMut<(Mut<Boid>, Mut<Transform>)>, point: Vec3, multiplier: f32) {
        //Turns towards point by multiplier amount. 1 makes it steer directly towards point
        let boid_angle = boid.1.rotation.to_euler(EulerRot::XYZ).2;
        let comparison_point = point - boid.1.translation;
        let angle_between = comparison_point.angle_between(bevy::math::vec3(boid_angle.sin()*-1., boid_angle.cos(), 0.));
        let point_angle = (comparison_point.y.atan2(comparison_point.x) + (std::f32::consts::PI*1.5)) % std::f32::consts::TAU;

        use std::f32::consts::*;
        if (point_angle-boid_angle+(PI+TAU))%TAU - PI < 0. {
            boid.1.rotate_z(angle_between * multiplier * -1.);
            boid.0.velocity = Vec2::from_angle(angle_between * multiplier * -1.).rotate(boid.0.velocity);
        } else {
            boid.1.rotate_z(angle_between * multiplier);
            boid.0.velocity = Vec2::from_angle(angle_between * multiplier).rotate(boid.0.velocity);
        }
    }
    pub fn steer_towards_velocity(boid: &mut core::cell::RefMut<(Mut<Boid>, Mut<Transform>)>, velocity: Vec2, multiplier: f32) {
        let boid_angle = boid.1.rotation.to_euler(EulerRot::XYZ).2;
        let target_heading = (velocity.y.atan2(velocity.x) + (std::f32::consts::PI*1.5)) % std::f32::consts::TAU;
        let angle_between = Vec3::from((velocity, 0.)).angle_between(bevy::math::vec3(boid_angle.sin()*-1., boid_angle.cos(), 0.));

        use std::f32::consts::*;
        if (target_heading-boid_angle+(PI+TAU))%TAU - PI < 0. {
            boid.1.rotate_z(angle_between * multiplier * -1.);
            boid.0.velocity = Vec2::from_angle(angle_between * multiplier * -1.).rotate(boid.0.velocity);
        } else {
            boid.1.rotate_z(angle_between * multiplier);
            boid.0.velocity = Vec2::from_angle(angle_between * multiplier).rotate(boid.0.velocity);
        }
    }
}

pub fn spawn_boids(mut commands: Commands, assets: Res<AssetServer>, windows: Res<Windows>) {
    let boid_texture: Handle<Image> = assets.load("boid.png");
    let window = windows.get_primary().unwrap();

    use rand::Rng;
    let mut rng = rand::thread_rng();
    for _ in 0..NUM_BOIDS {
        let xvel = rng.gen_range(0.3..2.0) * if rand::random() {-1.} else {1.};
        let yvel = rng.gen_range(0.3..2.0) * if rand::random() {-1.} else {1.};
        let xpos = rng.gen_range(window.width()/-2.0 .. window.width()/2.0);
        let ypos = rng.gen_range(window.height()/-2.0 .. window.height()/2.0);
        use bevy::math::vec2;
        commands.spawn(
            BoidBundle::with_velocity_and_position(
                boid_texture.clone(),
                vec2(xvel, yvel),
                vec2(xpos, ypos)
            )
        );
    }
}


//PERHAPS COMBINING THE FOLLOWING SYSTEMS WILL IMPROVE PERFORMANCE

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

fn boid_avoid_others(mut boid_query: Query<(&mut Boid, &mut Transform)>) {
    //Avoid running into other boids
}

fn boid_follow_others(mut boid_query: Query<(&mut Boid, &mut Transform)>) {
    //Steer towards the average heading of nearby boids
    use std::{rc::Rc, cell::RefCell};
    let boid_list: Vec<Rc<RefCell<(Mut<Boid>, Mut<Transform>)>>> = boid_query.iter_mut().map(|b| Rc::new(RefCell::new(b))).collect();
    for boid_ref in boid_list.iter() {
        let mut velocity_sum = bevy::math::vec2(0., 0.);
        let mut velocity_count = 0u8;
        {
        let boid = boid_ref.borrow();
        for cmp_boid_ref in boid_list.iter() {
            let cmp_boid = cmp_boid_ref.borrow();
            if cmp_boid.0.uid==boid.0.uid || !boid_is_nearby(&boid.1, &cmp_boid.1, BOID_RANGE) { continue }
            velocity_sum += cmp_boid.0.velocity;
            velocity_count += 1;
        }
        }
        if velocity_count == 0 { continue }

        velocity_sum /= velocity_count as f32;
        let mut boid = boid_ref.borrow_mut();
        Boid::steer_towards_velocity(&mut boid, velocity_sum, 0.05);
    }
}

fn boid_stick_together(mut boid_query: Query<(&mut Boid, &mut Transform)>) {
    //Steer towards the average position of nearby boids
    use std::{rc::Rc, cell::RefCell};
    let boid_list: Vec<Rc<RefCell<(Mut<Boid>, Mut<Transform>)>>> = boid_query.iter_mut().map(|b| Rc::new(RefCell::new(b))).collect();
    for boid_ref in boid_list.iter() {
        let mut position_sum = bevy::math::vec3(0., 0., 0.);
        let mut position_count = 0u8;
        {
        let boid = boid_ref.borrow();
        for cmp_boid_ref in boid_list.iter() {
            let cmp_boid = cmp_boid_ref.borrow();
            if cmp_boid.0.uid==boid.0.uid || !boid_is_nearby(&boid.1, &cmp_boid.1, BOID_RANGE) { continue }
            position_sum += cmp_boid.1.translation;
            position_count += 1;
        }
        }
        if position_count == 0 { continue }

        position_sum /= position_count as f32;
        let mut boid = boid_ref.borrow_mut();
        Boid::steer_towards_point(&mut boid, position_sum, 0.01);
    }
}

fn boid_is_nearby(transform1: &Mut<Transform>, transform2: &Mut<Transform>, range: f32) -> bool {
    transform1.translation.distance(transform2.translation) < range
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
