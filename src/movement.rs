use bevy::{
    prelude::{info, Camera, Query, Res, Transform, With, Without},
    sprite::collide_aabb::{collide, Collision},
    time::Time,
};

use crate::{
    constants::world,
    physics::{Collider, Velocity},
    player::{ActorState, Player},
};

pub fn player_movement(
    time: Res<Time>,
    mut players: Query<(&Player, &mut Transform, &Velocity, &mut Collider)>,
) {
    let time_delta = time.delta_seconds();
    for (player, mut transform, velocity, mut collider) in players.iter_mut() {
        for collision in &collider.collision {
            info!("Entered game level: {}", 1);
            match collision {
                Collision::Left => transform.translation.x += 1.0,
                Collision::Right => transform.translation.x -= 1.0,
                Collision::Top => transform.translation.y += time_delta * world::GRAVITY,
                Collision::Bottom => transform.translation.y += time_delta * world::GRAVITY,
                Collision::Inside => transform.translation.y += time_delta * world::GRAVITY,
            }
        }
        collider.collision.clear();
        match player.state {
            ActorState::WALKING => {
                transform.translation.x += time_delta * velocity.x;
            }
            _ => (),
        }
        transform.translation.y -= time_delta * world::GRAVITY;
    }
}

// Only supports a single player. To support multiple, adding a Tag component
// to both the player and the camera to make it clear which camera belongs to
// each player
pub fn camera_follow_player(
    mut cameras: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    players: Query<&mut Transform, (With<Player>, Without<Camera>)>,
) {
    for player_transform in players.iter() {
        for mut camera_transform in cameras.iter_mut() {
            camera_transform.translation.x = player_transform.translation.x;
            camera_transform.translation.y = player_transform.translation.y;
        }
    }
}

#[derive(Default)]
struct CollisionEvent;

// For a single player
pub fn check_for_collisions(
    mut player_query: Query<(&Transform, &mut Collider), With<Player>>,
    collider_query: Query<(&Transform, &Collider), Without<Player>>,
) {
    let (player_transform, mut player_collider) = player_query.single_mut();
    // check collision with walls
    for (transform, collider) in collider_query.iter() {
        if let Some(collide) = collide(
            player_transform.translation,
            player_collider.bound,
            transform.translation,
            collider.bound,
        ) {
            info!("fuck");
            player_collider.collision.push(collide);
        }
    }
}
