use std::time::Duration;

use bevy::prelude::*;

use crate::{
    constants::tile,
    physics::Velocity,
    player::{ActorState, Player},
};

#[derive(Component)]
pub struct Animation {
    pub frames: usize,
    pub timer: Timer,
}

/*
 * System to run all animations. Set them somewhere else.
 */
pub fn play_animations(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut Animation,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (mut animation, mut sprite, texture_atlas_handle) in &mut query {
        animation.timer.tick(time.delta());
        if animation.timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            let prev_idx = sprite.index;
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
            // rewind animation after reaching the last frame
            if prev_idx / animation.frames != sprite.index / animation.frames {
                sprite.index = animation.frames * (prev_idx / animation.frames);
            }
        }
    }
}

// Prepare actor animation
pub fn animate_actor(
    mut query: Query<(&Player, &Velocity, &mut Animation, &mut TextureAtlasSprite)>,
) {
    for (player, velocity, mut animation, mut sprite) in query.iter_mut() {
        let current_line = sprite.index / tile::PLAYER_FRAMES;
        match player.state {
            ActorState::WALKING => {
                if current_line != 0 {
                    sprite.index = 0;
                }
                animation.timer.set_duration(Duration::from_secs_f32(0.1));
                sprite.flip_x = if velocity.x > 0.0 { false } else { true };
            }
            ActorState::IDLE => {
                if current_line != 1 {
                    sprite.index = 8; // index of start of idle animation
                    animation.timer.set_duration(Duration::from_secs_f32(0.3));
                }
            }
            _ => (),
        }
    }
}
