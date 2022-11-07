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
        if keyboard_input.pressed(KeyCode::Left) {
            player.state = ActorState::WALKING;
            velocity.x = -50.0;
        } else if keyboard_input.pressed(KeyCode::Right) {
            player.state = ActorState::WALKING;
            velocity.x = 50.0;
        } else {
            player.state = ActorState::IDLE;
        }
    }
}
