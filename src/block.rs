use bevy::prelude::*;

use crate::{AsciiSheet, TILE_SIZE};
pub struct BlockPlugin;

#[derive(Component)]
pub struct Block {
    size: f32,
}

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_block).add_system(collider);
    }
}

fn collider(
    mut player_query: Query<(&Player, &mut Transform)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
}

fn spawn_block(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let mut sprite = TextureAtlasSprite::new(1);
    sprite.color = Color::rgb(0.3, 0.3, 0.9);
    sprite.custom_size = Some(Vec2::splat(TILE_SIZE));

    let player = commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: sprite,
            texture_atlas: ascii.0.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 900.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("Player"))
        .insert(Player { speed: 3.0 })
        .id();

    let mut background_sprite = TextureAtlasSprite::new(0);
    background_sprite.color = Color::rgb(0.7, 0.7, 0.7);
    background_sprite.custom_size = Some(Vec2::splat(TILE_SIZE));

    let background = commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: background_sprite,
            texture_atlas: ascii.0.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, -1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("Background"))
        .id();

    commands.entity(player).push_children(&[background]);
}
