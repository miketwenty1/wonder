use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::{ManSheet, TILE_SIZE};

pub struct PlayerPlugin;

#[derive(Component, Inspectable)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component, Inspectable)]
pub struct Player {
    speed: f32,
    index: u32,
    direction: Direction,
    idle: bool,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player);
        // .add_system(player_movement);
    }
}

fn player_movement(
    mut player_query: Query<(&mut Player, &mut Transform)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
    //texture_atlases: Res<Assets<TextureAtlas>>,
) {
    let (mut player, mut transform) = player_query.single_mut();

    if keyboard.pressed(KeyCode::W) {
        transform.translation.y += player.speed * TILE_SIZE * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::A) {
        transform.translation.x -= player.speed * TILE_SIZE * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::S) {
        transform.translation.y -= player.speed * TILE_SIZE * time.delta_seconds();
        player.index = 1;
    }
    if keyboard.pressed(KeyCode::D) {
        transform.translation.x += player.speed * TILE_SIZE * time.delta_seconds();
    }
}

fn spawn_player(
    mut commands: Commands,
    man_spritesheet: Res<ManSheet>,
    mut texture_atlases: ManSheet,
) {
    let mut sprite = TextureAtlasSprite::new(0);
    sprite.color = Color::rgb(1.0, 1.0, 1.0);
    sprite.custom_size = Some(Vec2::splat(TILE_SIZE));

    let player = commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: sprite,
            texture_atlas: texture_atlases.0.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 900.0),
                ..Default::default()
            },
            global_transform: GlobalTransform {
                translation: Default::default(),
                rotation: Default::default(),
                scale: Default::default(),
            },
            visibility: Visibility { is_visible: (true) },
            //..Default::default()
        })
        .insert(Name::new("Player"))
        .insert(Player {
            speed: 3.0,
            index: 0,
            idle: true,
            direction: Direction::Down,
        })
        .id();
    commands.entity(player); //.push_children(&[background]);
}
