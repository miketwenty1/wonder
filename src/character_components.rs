use bevy::prelude::*;

#[derive(Component)]
pub struct Name(pub String);

#[derive(Component)]
pub struct Balance(pub u32);

#[derive(Component)]
pub struct Health(pub u32);

#[derive(Component)]
pub struct Location(pub u32);

// pub enum UnitTypeEnum {
//     Player,
//     Npc,
//     Monster,
// }

// should only be one of these!!!
#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Npc;

#[derive(Component)]
pub struct Monster;
