pub(crate) mod path {
    pub const PLAYER_TEXTURE: &'static str = "textures/characters/player.png";
    pub const MAP1_TEXTURE: &'static str = "assets/maps/map1.json";
}

pub(crate) mod tile {
    use bevy::prelude::{Vec2, Vec3};

    pub const PLAYER_FRAMES: usize = 8;

    pub const SIZE: Vec2 = Vec2::new(32.0, 32.0);
    pub const PLAYER_SCALE: Vec3 = Vec3::splat(6.0);

    // Reference: https://doc.mapeditor.org/en/stable/reference/global-tile-ids/#tile-flipping
    // Bits on the far end of the 32-bit gid are used for tile flags
    pub const FLIPPED_HORIZONTALLY_FLAG: u32 = 0x80000000; // bit 32
    pub const FLIPPED_VERTICALLY_FLAG: u32 = 0x40000000; // bit 31
    pub const FLIPPED_DIAGONALLY_FLAG: u32 = 0x20000000; // bit 30
    pub const ROTATED_HEXAGONAL_120_FLAG: u32 = 0x10000000; // bit 29, unused in our case
}

pub(crate) mod world {
    pub const GRAVITY: f32 = 50.0;
}
