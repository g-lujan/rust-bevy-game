use bevy::{
    prelude::{
        default, info, AssetServer, Assets, Color, Commands, Quat, Res, ResMut, Transform, Vec2,
        Vec3,
    },
    sprite::{Anchor, SpriteSheetBundle, TextureAtlas, TextureAtlasSprite},
};
use serde::Deserialize;
use std::fs;

use crate::{
    constants::{path, tile},
    physics::Collider,
};

#[derive(Deserialize, Debug)]
struct TileMap {
    tileheight: u8,
    tilewidth: u8,
    height: u32,
    width: u32,
    layers: Vec<Layer>,
    tilesets: Vec<TilesetHeader>,
}

#[derive(Deserialize, Debug)]
struct TilesetHeader {
    firstgid: u8,
    source: String,
}

#[derive(Deserialize, Debug)]
struct Tileset {
    columns: usize,
    image: String,
    imageheight: u32,
    imagewidth: u32,
    margin: u8,
    name: String,
    spacing: u8,
    tilecount: usize,
    tileheight: u32,
    tilewidth: u32,
    #[serde(alias = "type")]
    tileset_type: String,
}

#[derive(Deserialize, Debug)]
struct Layer {
    name: String,
    visible: bool,
    #[serde(alias = "type")]
    layer_type: String,
    #[serde(default)]
    data: Vec<u32>,
    #[serde(default)]
    objects: Vec<TiledObject>,
}

#[derive(Deserialize, Debug)]
struct TiledObject {
    class: String,
    name: String,
    height: f32,
    id: u32,
    rotation: f32,
    visible: bool,
    width: f32,
    x: f32,
    y: f32,
}

pub fn load_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // load resources
    info!("Trying to load tilemap {}", path::MAP1_TEXTURE);
    let map_data = fs::read_to_string(path::MAP1_TEXTURE).expect("Unable to read file");
    let tile_map: TileMap = serde_json::from_str(&map_data).expect("Unable to parse");
    info!("Trying to load tileset {}", &tile_map.tilesets[0].source);
    let tileset_data =
        fs::read_to_string(&tile_map.tilesets[0].source).expect("Unable to read file");
    let tileset: Tileset = serde_json::from_str(&tileset_data).expect("Unable to parse");

    // load base image
    let texture_atlas = TextureAtlas::from_grid(
        asset_server.load(&tileset.image),
        Vec2 {
            x: tileset.tilewidth as f32,
            y: tileset.tileheight as f32,
        },
        tileset.columns,
        tileset.tilecount / tileset.columns,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // spawn tiles and colliders
    for layer in tile_map.layers {
        if !layer.visible {
            continue;
        }
        match layer.layer_type.as_str() {
            "tilelayer" => {
                let mut curr_tile_col: u32 = 0;
                let mut curr_tile_line: u32 = 0;
                for tile_data in layer.data {
                    if curr_tile_col >= tile_map.width {
                        curr_tile_line += 1;
                        curr_tile_col = 0;
                    }
                    if tile_data == 0 {
                        curr_tile_col += 1;
                        continue;
                    }
                    commands.spawn_bundle(SpriteSheetBundle {
                        texture_atlas: texture_atlas_handle.clone(),
                        transform: Transform {
                            translation: Vec3::from(tiled_to_bevy_coord(
                                (curr_tile_col * tileset.tilewidth) as f32,
                                (curr_tile_line * tileset.tileheight) as f32,
                                tileset.tilewidth as f32,
                                tileset.tileheight as f32,
                                tile_map.width,
                                tile_map.height,
                            )),
                            rotation: Quat::from_rotation_z(tile_rotation(tile_data)),
                            scale: Vec3::ONE,
                        },
                        sprite: create_tile(tile_data),
                        ..default()
                    });
                    curr_tile_col += 1;
                }
            }
            "objectgroup" => {
                for object in layer.objects {
                    let bevy_coords = tiled_to_bevy_coord(
                        object.x,
                        object.y,
                        object.width,
                        object.height,
                        tile_map.width,
                        tile_map.height,
                    );
                    commands
                        .spawn()
                        .insert(Collider {
                            bound: Vec2 {
                                x: object.width,
                                y: object.height,
                            },
                            collision: Vec::new(),
                        })
                        .insert(Transform::from_xyz(
                            bevy_coords.0,
                            bevy_coords.1,
                            bevy_coords.2,
                        ));
                }
            }
            _ => (),
        }
    }
}

fn tile_rotation(raw_tile_id: u32) -> f32 {
    let horizontal_flip = (raw_tile_id & tile::FLIPPED_HORIZONTALLY_FLAG) != 0;
    let vertical_flip = (raw_tile_id & tile::FLIPPED_VERTICALLY_FLAG) != 0;
    let anti_diag_flip = (raw_tile_id & tile::FLIPPED_DIAGONALLY_FLAG) != 0;

    return if anti_diag_flip && horizontal_flip {
        90.0 * std::f32::consts::PI / 180.0
    } else if anti_diag_flip && vertical_flip {
        -90.0 * std::f32::consts::PI / 180.0
    } else {
        0.0
    };
}

fn create_tile(raw_tile_id: u32) -> TextureAtlasSprite {
    let horizontal_flip = (raw_tile_id & tile::FLIPPED_HORIZONTALLY_FLAG) != 0;
    let vertical_flip = (raw_tile_id & tile::FLIPPED_VERTICALLY_FLAG) != 0;

    // Clear all four flags to get actual id
    let actual_index: u32 = raw_tile_id
        & !(tile::FLIPPED_HORIZONTALLY_FLAG
            | tile::FLIPPED_VERTICALLY_FLAG
            | tile::FLIPPED_DIAGONALLY_FLAG
            | tile::ROTATED_HEXAGONAL_120_FLAG);

    TextureAtlasSprite {
        index: (actual_index - 1) as usize,
        flip_x: horizontal_flip,
        flip_y: vertical_flip,
        color: Color::WHITE,
        custom_size: None,
        anchor: Anchor::default(),
    }
}

// Tiled coordinates are in the top-left corner, with the y axis pointing downwards
// Bevy coordinates are in the center, with the y axis pointing upwards
fn tiled_to_bevy_coord(
    tiled_x: f32,
    tiled_y: f32,
    width: f32,
    height: f32,
    map_width: u32,
    map_height: u32,
) -> (f32, f32, f32) {
    (
        tiled_x - (map_width as f32 / 2f32) + (width as f32 / 2f32),
        -tiled_y + (map_height as f32 / 2f32) - (height as f32 / 2f32),
        0f32,
    )
}
