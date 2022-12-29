use bevy::prelude::*; //text::Text2dBounds};
                      //use bevy::sprite::collide_aabb::{collide, Collision};
                      //use bevy_inspector_egui::Inspectable;
use ulam::{self, Coord};

use crate::components::Block;
use crate::player::Edge;
use crate::player::EdgeEvent;
// use crate::{AsciiSheet, TILE_SIZE};
pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(update_block_system);
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
    // let text_alignment_topleft = TextAlignment {
    //     vertical: VerticalAlign::Center,
    //     horizontal: HorizontalAlign::Center,
    // };
    let c = Coord { x: 0, y: 0 };

    commands
        .spawn_bundle(Text2dBundle {
            text: Text::from_section(ulam::get_ulam_point(&c).value.to_string(), text_style),
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
        })
        .insert(Block {
            size: 500.,
            num: 0,
            x: 0,
            y: 0,
        });
}

fn update_block_system(
    mut edge_event: EventReader<EdgeEvent>,
    mut query: Query<(&mut Text, &mut Block), With<Block>>,
) {
    for (mut text, mut block) in query.iter_mut() {
        //let mut text = text2.clone();
        for event in edge_event.iter() {
            //println!("event captured {:?}", event);
            //println!("Entity {:?} leveled up!", event);
            let mut delta_y = 0;
            let mut delta_x = 0;

            match event.0 {
                Edge::East => delta_x = 1,
                Edge::West => delta_x = -1,
                Edge::North => delta_y = 1,
                Edge::South => delta_y = -1,
            };

            // if event.0 == "EdgeEvent(East)" {
            //     println!("EASTER SUNDAY");
            // }
            let new_x: i32 = block.x + delta_x;
            let new_y: i32 = block.y + delta_y;

            block.x = new_x;
            block.y = new_y;

            let c = Coord { x: new_x, y: new_y };
            text.sections[0].value = ulam::get_ulam_point(&c).value.to_string();
            //println!("{}", &text.sections[0].value);
        }
    }
}
