use bevy::prelude::*;
use bevy::tasks::IoTaskPool;

use crate::comms::InvoicePayChannel;
use crate::ActixServerURI;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.65, 0.25, 0.85);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

#[derive(Component)]
pub struct PayButton;

#[derive(Component)]
pub struct InvoiceSVG(String);

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

#[derive(Component)]
pub struct Blockheight(u32);

pub fn animate_sprite(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("spritesheets/gabe-idle-run.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 7, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..default()
        },
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));

    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_style = TextStyle {
        font,
        font_size: 60.0,
        color: Color::WHITE,
    };

    commands.spawn((
        Text2dBundle {
            text: Text::from_section("", text_style).with_alignment(TextAlignment::CENTER),
            transform: Transform::from_xyz(0.0, 250.0, 2.0),
            ..default()
        },
        Blockheight(0),
    ));
}

pub fn setup_pay_button(mut commands: Commands, asset_server: Res<AssetServer>) {
    trace!("very noisy");
    debug!("helpful for debugging");
    info!("helpful information that is worth printing by default");
    warn!("something bad happened that isn't a failure, but thats worth calling out");
    error!("something failed");
    // ui camera
    commands
        .spawn((
            ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(250.0), Val::Px(65.0)),
                    // center button
                    //margin: UiRect::all(Val::Percent(50.0)),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically align inner child text
                    align_items: AlignItems::Center,
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        top: Val::Px(250.0),
                        left: Val::Px(362.0),
                        ..default()
                    },
                    ..default()
                },
                background_color: NORMAL_BUTTON.into(),

                ..default()
            },
            PayButton,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Pay 50 sats",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        });
}

#[allow(clippy::type_complexity)]
#[allow(unused)]
pub fn pay_button_system(
    //mut state: ResMut<State<AppState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PayButton>),
    >,
    comm_channel: ResMut<InvoicePayChannel>,
    actix_server: Res<ActixServerURI>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                info!("creating invoice");
                let server = actix_server.clone().0;
                let pool = IoTaskPool::get();
                let cc = comm_channel.tx.clone();
                let _task = pool.spawn(async move {
                    let api_response_text = reqwest::get(format!("{}/invoice/50000", server))
                        .await
                        .unwrap()
                        .text()
                        .await
                        .unwrap();
                    cc.try_send(api_response_text);
                    //info!("debug invoice {}", api_response_text);
                });
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}
