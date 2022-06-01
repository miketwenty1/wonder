use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::{
    components::{Player, Velocity},
    GameTextures, BASE_SPEED, SPRITE_SCALE, TIME_STEP,
};

#[derive(Component, Inspectable)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, player_spawn_system)
            .add_system(player_movement_system)
            .add_system(player_keyboard_event_system)
            .add_system(animate_sprite_system)
            .add_system(camera_follow);
    }
}

fn player_spawn_system(mut commands: Commands, game_textures: Res<GameTextures>) {
    //game_textures: Res<HandyboySpriteSheet>) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: 0,
                ..Default::default()
            },
            texture_atlas: game_textures.player.clone(),
            transform: Transform {
                scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player {
            direction: Direction::Down,
            up_animation_indexes: [6, 7, 6, 8].to_vec(),
            down_animation_indexes: [0, 1, 0, 2].to_vec(),
            left_animation_indexes: [3, 4, 3, 5].to_vec(),
            right_animation_indexes: [3, 4, 3, 5].to_vec(),
            current_animation_index: 0,
            flipx_animation_l: true,
            flipx_animation_r: false,
        })
        .insert(Velocity { x: 0., y: 0. })
        .insert(AnimationTimer(Timer::from_seconds(0.15, true)));
}

fn player_movement_system(
    mut query: Query<(&Velocity, &mut Transform, &mut Player), With<Player>>,
) {
    for (velocity, mut transform, mut player) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x * TIME_STEP * BASE_SPEED;
        translation.y += velocity.y * TIME_STEP * BASE_SPEED;
        if velocity.x > 0. {
            player.direction = Direction::Right;
        }
        if velocity.x < 0. {
            player.direction = Direction::Left;
        }
        if velocity.y > 0. {
            player.direction = Direction::Up;
        }
        if velocity.y < 0. {
            player.direction = Direction::Down;
        }
    }
}

fn player_keyboard_event_system(
    kb: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    // const diagonal_velocity: f32 = 0.7071068;
    //
    //     let key = kb.get_pressed().last().unwrap_or_else(|| &KeyCode::P);
    //     match key {
    //         (KeyCode::Left | KeyCode::A) (KeyCode::Up | KeyCode::W) => velocity.x = -1.,
    //         KeyCode::Right | KeyCode::D => velocity.x = 1.,
    //         KeyCode::Up | KeyCode::W => velocity.y = 1.,
    //         KeyCode::Down | KeyCode::S => velocity.y = -1.,
    //         _ => (velocity.x, velocity.y) = (0., 0.),
    //     }

    // WORKS BUT IS VERY VERBOSE
    // if let Ok(mut velocity) = query.get_single_mut() {
    //     // diagonal NW
    //     if (kb.pressed(KeyCode::Left) || kb.pressed(KeyCode::A))
    //         && (kb.pressed(KeyCode::Up) || kb.pressed(KeyCode::W))
    //     {
    //         velocity.x = -diagonal_velocity;
    //         velocity.y = diagonal_velocity;
    //     // diagonal NE
    //     } else if (kb.pressed(KeyCode::Right) || kb.pressed(KeyCode::D))
    //         && (kb.pressed(KeyCode::Up) || kb.pressed(KeyCode::W))
    //     {
    //         velocity.x = diagonal_velocity;
    //         velocity.y = diagonal_velocity;
    //     // diagonal SW
    //     } else if (kb.pressed(KeyCode::Left) || kb.pressed(KeyCode::A))
    //         && (kb.pressed(KeyCode::Down) || kb.pressed(KeyCode::S))
    //     {
    //         velocity.x = -diagonal_velocity;
    //         velocity.y = -diagonal_velocity;
    //         // diagonal SE
    //     } else if (kb.pressed(KeyCode::Right) || kb.pressed(KeyCode::D))
    //         && (kb.pressed(KeyCode::Down) || kb.pressed(KeyCode::S))
    //     {
    //         velocity.x = diagonal_velocity;
    //         velocity.y = -diagonal_velocity;
    //         // N
    //     } else if (kb.pressed(KeyCode::Up) || kb.pressed(KeyCode::W)) {
    //         velocity.x = 0.;
    //         velocity.y = 1.;
    //         // S
    //     } else if (kb.pressed(KeyCode::Down) || kb.pressed(KeyCode::S)) {
    //         velocity.x = 0.;
    //         velocity.y = -1.;
    //         // E
    //     } else if (kb.pressed(KeyCode::Right) || kb.pressed(KeyCode::D)) {
    //         velocity.x = 1.;
    //         velocity.y = 0.;
    //         // W
    //     } else if (kb.pressed(KeyCode::Left) || kb.pressed(KeyCode::A)) {
    //         velocity.x = -1.;
    //         velocity.y = 0.;
    //         // None
    //     } else {
    //         velocity.x = 0.;
    //         velocity.y = 0.;
    //     }
    // }
    if let Ok(mut velocity2) = query.get_single_mut() {
        let mut velocity = Vec3::ZERO;
        if kb.pressed(KeyCode::W) || kb.pressed(KeyCode::Up) {
            velocity.y = 1.;
        }

        if kb.pressed(KeyCode::A) || kb.pressed(KeyCode::Left) {
            velocity.x = -1.;
        }

        if kb.pressed(KeyCode::S) || kb.pressed(KeyCode::Down) {
            velocity.y = -1.;
        }

        if kb.pressed(KeyCode::D) || kb.pressed(KeyCode::Right) {
            velocity.x = 1.;
        }
        velocity = velocity.normalize_or_zero();
        velocity2.x = velocity.x;
        velocity2.y = velocity.y;
    }
}

fn animate_sprite_system(
    time: Res<Time>,
    //texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &Velocity,
        &mut Player,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        // &Handle<TextureAtlas>,
    )>,
) {
    for (velocity, mut player, mut timer, mut sprite) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            let dir_indexes: Vec<usize>;
            match player.direction {
                Direction::Up => {
                    dir_indexes = player.up_animation_indexes.clone();
                }
                Direction::Down => {
                    dir_indexes = player.down_animation_indexes.clone();
                }
                Direction::Left => {
                    dir_indexes = player.left_animation_indexes.clone();
                    sprite.flip_x = true;
                }
                Direction::Right => {
                    dir_indexes = player.right_animation_indexes.clone();
                    sprite.flip_x = false;
                }
            }
            if velocity.x == 0. && velocity.y == 0. {
                sprite.index = dir_indexes[0];
                player.current_animation_index = 0;
            } else {
                //let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
                if player.current_animation_index >= dir_indexes.len() - 1 {
                    sprite.index = dir_indexes[0];
                    player.current_animation_index = 0;
                } else {
                    player.current_animation_index += 1;
                    sprite.index = dir_indexes[player.current_animation_index];
                }
            }
        }
    }
}

fn camera_follow(
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    player_query: Query<(&Player, &Transform)>,
) {
    let mut cam_transform = camera_query.single_mut();
    let (_, player_transform) = player_query.single();

    cam_transform.translation.x = player_transform.translation.x;
    cam_transform.translation.y = player_transform.translation.y;
}
