use bevy::{prelude::*, render::texture::ImageSettings};
use movement::{camera_follow_player, check_player_collisions};
use tiled_map::load_map;

mod animation;
mod constants;
mod tiled_map;
use crate::animation::{animate_actor, play_animations};
mod player;
use crate::player::spawn_player;
mod input;
use crate::input::player_input;
mod movement;
use crate::movement::player_movement;
mod physics;

fn main() {
    App::new()
        .insert_resource(ImageSettings::default_nearest()) // prevents blurry sprites
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(load_map)
        .add_system(player_input.before(player_movement))
        .add_system(check_player_collisions.before(player_movement))
        .add_system(player_movement)
        .add_system(animate_actor.after(player_movement))
        .add_system(camera_follow_player.after(player_movement))
        .add_system(play_animations.after(animate_actor))
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Camera
    commands.spawn_bundle(Camera2dBundle::default());
    // Player
    spawn_player(commands, asset_server, texture_atlases);
}
