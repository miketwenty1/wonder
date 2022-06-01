use bevy::{prelude::*, text::Text2dBounds};
use ulam::{self, Coord};
// use crate::{AsciiSheet, TILE_SIZE};
pub struct BlockPlugin;

#[derive(Component)]
pub struct Block {
    size: f32,
}

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_style = TextStyle {
        font,
        font_size: 85.0,
        color: Color::WHITE,
    };
    // let text_alignment = TextAlignment {
    //     vertical: VerticalAlign::Center,
    //     horizontal: HorizontalAlign::Center,
    // };

    let box_size = Size::new(500.0, 500.0);
    let box_position = Vec2::new(0.0, 0.0);
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(box_size.width, box_size.height)),
            ..default()
        },
        transform: Transform::from_translation(box_position.extend(0.0)),
        ..default()
    });
    let text_alignment_topleft = TextAlignment {
        vertical: VerticalAlign::Center,
        horizontal: HorizontalAlign::Center,
    };
    let c = Coord { x: 0, y: 0 };
    commands.spawn_bundle(Text2dBundle {
        text: Text::with_section(
            ulam::get_ulam_deets(&c).value.to_string(),
            text_style,
            text_alignment_topleft,
        ),
        // text_2d_bounds: Text2dBounds {
        //     // Wrap text in the rectangle
        //     size: box_size,
        // },
        // We align text to the top-left, so this transform is the top-left corner of our text. The
        // box is centered at box_position, so it is necessary to move by half of the box size to
        // keep the text in the box.
        transform: Transform::from_xyz(
            0.0, 0.0, //box_position.x - box_size.width / 2.0,
            //box_position.y + box_size.height / 2.0,
            1.0,
        ),
        ..default()
    });
}
