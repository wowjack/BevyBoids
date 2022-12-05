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


pub fn spawn_boids(mut commands: Commands, assets: Res<AssetServer>, windows: Res<Windows>) {
    let boid_texture: Handle<Image> = assets.load("boid.png");
    let window = windows.get_primary().unwrap();

    use rand::Rng;
    let mut rng = rand::thread_rng();
    for _ in 0..30 {
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

fn boid_follow_others() {
    //Steer towards the average heading of nearby boids
}

fn boid_stick_together() {
    //Steer towards the average position of nearby boids
}