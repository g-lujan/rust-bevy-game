use bevy::{
    prelude::{Component, Vec2},
    sprite::collide_aabb::Collision,
};

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Collider {
    pub bound: Vec2,
    pub collision: Vec<Collision>,
}
