

use bevy::color::palettes::css::{BLACK, WHITE};

use crate::*;

pub fn update_num_boids(
    on: On<ValueChange<f32>>,
    mut settings: ResMut<SimulationSettings>,
    mut commands: Commands,
    boid_query: Query<Entity, With<Boid>>
) {
    match settings.num_boids.cmp(&(on.value as u16)) {
        std::cmp::Ordering::Equal => (),
        std::cmp::Ordering::Less => {
            let iter = (settings.num_boids..(on.value as u16)).map(|_| {
                (Boid::default(), WithPos::Random, WithVel::Random)
            });
            commands.spawn_batch(iter);
        },
        std::cmp::Ordering::Greater => {
            boid_query.iter()
                .take((settings.num_boids - (on.value as u16)) as usize)
                .for_each(|e| commands.get_entity(e).unwrap().despawn());
        },
    }

    settings.num_boids = on.value as u16
}

pub fn update_boid_range(
    on: On<ValueChange<f32>>,
    mut settings: ResMut<SimulationSettings>,
    boid_query: Query<&Transform, With<Boid>>,
    mut gizmos: Gizmos
) {
    settings.boid_range = on.value;
    let Some(transform) = boid_query.iter().next() else { return };
    gizmos.circle_2d(Isometry2d::from_translation(transform.translation.xy()), settings.boid_range, BLACK);
}

pub fn update_run_simulation(
    on: On<ValueChange<bool>>,
    mut settings: ResMut<SimulationSettings>
) {
    settings.run = on.value
}


pub fn update_avoid(
    on: On<ValueChange<f32>>,
    mut settings: ResMut<SimulationSettings>
) {
    settings.avoid_boid_strength = on.value
}

pub fn update_follow(
    on: On<ValueChange<f32>>,
    mut settings: ResMut<SimulationSettings>
) {
    settings.follow_others_strength = on.value
}

pub fn update_stick_together(
    on: On<ValueChange<f32>>,
    mut settings: ResMut<SimulationSettings>
) {
    settings.stick_together_strength = on.value
}