use bevy::prelude::*;

use crate::animation::Animation;
use crate::constants::path;
use crate::constants::tile;
use crate::physics::Collider;
use crate::physics::Velocity;

#[derive(PartialEq)]
pub enum ActorState {
    IDLE,
    WALKING,
    JUMPING,
    INACTIVE,
}

#[derive(Component)]
pub struct Player {
    pub name: String,
    pub state: Vec<ActorState>,
    pub grounded: bool,
}

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load(path::PLAYER_TEXTURE);
    let texture_atlas = TextureAtlas::from_grid(texture_handle, tile::SIZE, 8, 2);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    // Add components
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(tile::PLAYER_SCALE),
            ..default()
        })
        .insert(Animation {
            frames: tile::PLAYER_FRAMES,
            timer: Timer::from_seconds(0.1, true),
        })
        .insert(Player {
            state: vec!(ActorState::IDLE),
            name: "Player1".to_string(),
            grounded: false,
        })
        .insert(Transform::from_xyz(200.0, 200.0, 0.0))
        .insert(Collider {
            bound: tile::SIZE,
            collision: Vec::new(),
        })
        .insert(Velocity { x: 0.0, y: 1.0 });
}
