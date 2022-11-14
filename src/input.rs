use crate::physics::Collider;
use crate::physics::Velocity;
use crate::player::ActorState;
use crate::player::Player;
use bevy::prelude::*;

pub fn player_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Player, &mut Velocity, &mut Collider)>,
) {
    for (mut player, mut velocity, collider) in query.iter_mut() {
        player.state.clear();
        if keyboard_input.pressed(KeyCode::Left) {
            player.state.push(ActorState::WALKING);
            velocity.x = -50.0;
        } 
        if keyboard_input.pressed(KeyCode::Right) {
            player.state.push(ActorState::WALKING);
            velocity.x = 50.0;
        }
        if keyboard_input.pressed(KeyCode::Space) && player.grounded {
            player.state.push(ActorState::JUMPING);
            velocity.y = 300.0;
            player.grounded = false;
        }
    }
}
