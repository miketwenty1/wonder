use bevy::prelude::Component;
use bevy_inspector_egui::Inspectable;
// use ulam::Coord;

use crate::player::Direction;

#[derive(Component, Inspectable)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Inspectable)]
pub struct Player {
    pub direction: Direction,
    pub up_animation_indexes: Vec<usize>,
    pub down_animation_indexes: Vec<usize>,
    pub left_animation_indexes: Vec<usize>,
    pub right_animation_indexes: Vec<usize>,
    pub current_animation_index: usize,
    // should the animation be flipped along the x for Left or Right?
    pub flipx_animation_l: bool,
    pub flipx_animation_r: bool,
}

#[derive(Component, Inspectable)]
pub struct Block {
    pub size: f32,
    pub num: i32,
    pub x: i32,
    pub y: i32,
}

#[derive(Default, Inspectable)]
struct CollisionEvent;

#[derive(Default, Inspectable)]
struct Collider;

#[derive(Component, Inspectable, Debug)]
pub struct Counter(pub i32);
