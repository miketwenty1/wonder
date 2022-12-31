use async_channel::{Receiver, Sender};
use bevy::prelude::*;
use bevy::tasks::IoTaskPool;
use egui_extras::RetainedImage;
use qrcode::render::svg;
use qrcode::{EcLevel, QrCode, Version};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::error::Error;
use std::str::Bytes;
use validator::Validate;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::AppState;
use crate::SERVER_URL;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.65, 0.25, 0.85);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

#[derive(Component)]
pub struct PayButton;

#[wasm_bindgen]
extern "C" {

    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn log_u32(a: u32);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn log_many(a: &str, b: &str);

}

macro_rules! console_log {
  ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[derive(Debug, Serialize, Validate, Deserialize)]
pub struct BlockData {
    #[validate(length(equal = 64))]
    pub hash: String,
    #[validate(range(min = 0, max = 1_000_000))]
    pub height: i32,
}

#[derive(Debug, Serialize, Validate, Deserialize)]
pub struct InvoiceData {
    pub invoice: String,
}

#[derive(Component)]
pub struct InvoiceSVG(String);

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

#[derive(Component)]
pub struct Blockheight(u32);

#[derive(Resource, Component, Default, Debug)]
pub struct Counter {
    pub c: i32,
    pub c_name: String,
}
#[derive(Resource, Clone)]
pub struct HeightChannel {
    pub tx: Sender<String>,
    pub rx: Receiver<String>,
}

#[derive(Resource, Clone)]
pub struct InvoiceChannel {
    pub tx: Sender<String>,
    pub rx: Receiver<String>,
}

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

#[allow(clippy::redundant_clone)]
pub fn setup_comm(mut commands: Commands) {
    let (tx_height, rx_height) = async_channel::bounded(1);
    let (tx_pay, rx_pay) = async_channel::bounded(1);
    commands.insert_resource(HeightChannel {
        tx: tx_height,
        rx: rx_height,
    });
    commands.insert_resource(InvoiceChannel {
        tx: tx_pay,
        rx: rx_pay,
    });

    let sender_counter = Counter {
        c: 0,
        c_name: String::from("sender"),
    };
    let receiver_counter = Counter {
        c: 0,
        c_name: String::from("receiver"),
    };

    commands.spawn(sender_counter);
    commands.spawn(receiver_counter);
}

#[allow(unused)]
pub fn api_sender(comm_channels: ResMut<HeightChannel>, mut query: Query<&mut Counter>) {
    for mut fieldd in query.iter_mut() {
        if fieldd.c_name == "sender" {
            fieldd.c += 1;
            // Every 100 clicks it tries triggers this system
            if fieldd.c % 5250 == 0 {
                let pool = IoTaskPool::get();
                let cc = comm_channels.tx.clone();
                console_log!("sender_fn hit {} {}!", fieldd.c_name, fieldd.c);
                let _task = pool.spawn(async move {
                    let api_response_text = reqwest::get(format!("{}/blockheight", SERVER_URL))
                        .await
                        .unwrap()
                        .text()
                        .await
                        .unwrap();
                    cc.try_send(api_response_text);
                });
            };
        };
    }
}

pub fn api_receiver(
    comm_channel: ResMut<HeightChannel>,
    invoice_channel: ResMut<InvoiceChannel>,
    mut query: Query<&mut Counter>,
    mut block_height_query: Query<&mut Text, With<Blockheight>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for mut fieldd in query.iter_mut() {
        if fieldd.c_name == "receiver" {
            fieldd.c += 1;
            // Every 100 clicks it tries triggers this system
            if fieldd.c % 500 == 0 {
                console_log!("recv_fn hit {} {}!", fieldd.c_name, fieldd.c);
                let r = comm_channel.rx.try_recv();
                let r_invoice = invoice_channel.rx.try_recv();

                let r_msg = match r {
                    Ok(r) => r,
                    Err(e) => e.to_string(),
                };
                let r_msg_invoice = match r_invoice {
                    Ok(r_invoice) => r_invoice,
                    Err(e) => e.to_string(),
                };
                //block_height_query.get_single_mut().unwrap().0 = r_msg.

                let yz = serde_json::from_str::<BlockData>(&r_msg);
                let r_invoice_result = serde_json::from_str::<InvoiceData>(&r_msg_invoice);

                match yz {
                    Ok(o) => {
                        console_log!("received new block data");
                        block_height_query.get_single_mut().unwrap().sections[0].value =
                            format!("Current Blockheight: {}", o.height);
                    }
                    Err(e) => {
                        console_log!("waiting to receive new block data: {}", e);
                    }
                };
                match r_invoice_result {
                    Ok(o) => {
                        info!("received new Invoice data {:#?}", o);

                        // let code = QrCode::new(o.invoice).unwrap();
                        // let image = code
                        //     .render::<unicode::Dense1x2>()
                        //     .dark_color(unicode::Dense1x2::Light)
                        //     .light_color(unicode::Dense1x2::Dark)
                        //     .build();

                        // let font = asset_server.load("fonts/FiraSans-Bold.ttf");
                        // let text_style = TextStyle {
                        //     font,
                        //     font_size: 60.0,
                        //     color: Color::WHITE,
                        // };

                        // commands.spawn((
                        //     Text2dBundle {
                        //         text: Text::from_section(image.clone(), text_style)
                        //             .with_alignment(TextAlignment::CENTER),
                        //         transform: Transform::from_xyz(0.0, 0.0, 4.0),
                        //         ..default()
                        //     },
                        //     InvoiceSVG(image.clone()),
                        // ));
                        // info!("{}", image);
                    }
                    Err(e) => {
                        info!("no new invoice data to get: {}", e);
                    }
                };
            }
        }
    }
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
    comm_channel: ResMut<InvoiceChannel>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                info!("creating invoice");

                let pool = IoTaskPool::get();
                let cc = comm_channel.tx.clone();
                let _task = pool.spawn(async move {
                    let api_response_text = reqwest::get(format!("{}/invoice/50000", SERVER_URL))
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
