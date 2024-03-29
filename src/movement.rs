use bevy::{
    prelude::{Camera, Query, Res, Transform, With, Without},
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
    mut player_query: Query<(&mut Player, &mut Transform, &mut Velocity, &mut Collider)>,
) {
    let time_delta = time.delta_seconds();
    let (mut player, mut transform, mut velocity, mut collider) = player_query.single_mut();
    for collision in &collider.collision {
        match collision {
            Collision::Left => transform.translation.x += 1.0,
            Collision::Right => transform.translation.x -= 1.0,
            Collision::Top => (),
            Collision::Bottom => {
                player.grounded = true;
            },
            Collision::Inside => (),
        }
    }
    collider.collision.clear(); // collisions resolved
    if player.state.contains(&ActorState::WALKING) {
        transform.translation.x += time_delta * velocity.x;
    }
    if !player.grounded {
        transform.translation.y += time_delta * (velocity.y - world::GRAVITY);
        velocity.y -= 5.0;
    } else {
        transform.translation.y += if velocity.y > 0.0 {time_delta * velocity.y} else {0.0};
    }
}

// Only supports a single player. To support multiple, adding a Tag component
// to both the player and the camera to make it clear which camera belongs to
// each player
pub fn camera_follow_player(
    mut cameras: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    player_query: Query<&mut Transform, (With<Player>, Without<Camera>)>,
) {
    let player_transform = player_query.single();
    for mut camera_transform in cameras.iter_mut() {
        camera_transform.translation.x = player_transform.translation.x;
        camera_transform.translation.y = player_transform.translation.y;
    }
}

// For a single player
pub fn check_player_collisions(
    mut player_query: Query<(&Transform, &mut Collider), With<Player>>,
    collider_query: Query<(&Transform, &Collider), Without<Player>>,
) {
    let (player_transform, mut player_collider) = player_query.single_mut();
    // check collision with walls
    for (other_transform, other_collider) in collider_query.iter() {
        if let Some(collide) = collide(
            other_transform.translation,
            other_collider.bound,
            player_transform.translation,
            player_collider.bound,
        ) {
            player_collider.collision.push(collide);
        }
    }
}
