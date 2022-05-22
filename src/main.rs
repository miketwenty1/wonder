use bevy::{prelude::*, text::Text2dBounds};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(change_text_of_block)
        .run();
}

#[derive(Component)]
struct TextChanges;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_style = TextStyle {
        font,
        font_size: 60.0,
        color: Color::WHITE,
    };
    // 2d camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    let box_size = Size::new(300.0, 300.0);
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
    let text_alignment_center = TextAlignment {
        vertical: VerticalAlign::Center,
        horizontal: HorizontalAlign::Center,
    };
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section("0", text_style, text_alignment_center),
            text_2d_bounds: Text2dBounds {
                // Wrap text in the rectangle
                size: box_size,
            },
            // We align text to the top-left, so this transform is the top-left corner of our text. The
            // box is centered at box_position, so it is necessary to move by half of the box size to
            // keep the text in the box.
            // transform: Transform::from_xyz(
            //     box_position.x - box_size.width / 2.0,
            //     box_position.y + box_size.height / 2.0,
            //     1.0,
            // ),
            ..default()
        })
        .insert(TextChanges);
}

fn change_text_of_block(time: Res<Time>, mut query: Query<&mut Text, With<TextChanges>>) {
    for mut text in query.iter_mut() {
        let block_str = (text.sections[0].value).to_string();
        let block_int: i32 = block_str.parse().unwrap();
        //let block_fin = block_int

        //println!("{}", block_int);
        text.sections[0].value = format!("{}", block_int + 1);
    }
}
