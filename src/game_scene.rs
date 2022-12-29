use async_channel::{Receiver, Sender};
use bevy::prelude::*;
use bevy::tasks::IoTaskPool;
use wasm_bindgen::prelude::wasm_bindgen;

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

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);
#[derive(Resource, Component, Default, Debug)]
pub struct Counter {
    pub c: i32,
    pub c_name: String,
}
#[derive(Resource, Clone)]
pub struct CommChannels {
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
}

pub fn setup_comm(mut commands: Commands) {
    let (tx, rx) = async_channel::bounded(1);
    commands.insert_resource(CommChannels { tx, rx });
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
pub fn api_sender(comm_channels: ResMut<CommChannels>, mut query: Query<&mut Counter>) {
    for mut fieldd in query.iter_mut() {
        if fieldd.c_name == "sender" {
            fieldd.c += 1;
            // Every 100 clicks it tries triggers this system
            if fieldd.c % 1000 == 0 {
                let pool = IoTaskPool::get();
                let cc = comm_channels.tx.clone();
                console_log!("sender_fn hit {} {}!", fieldd.c_name, fieldd.c);
                let _task = pool.spawn(async move {
                    let api_response_text = reqwest::get("http://localhost:8081/blockheight")
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

pub fn api_receiver(comm_channels: ResMut<CommChannels>, mut query: Query<&mut Counter>) {
    for mut fieldd in query.iter_mut() {
        if fieldd.c_name == "receiver" {
            fieldd.c += 1;
            // Every 100 clicks it tries triggers this system
            if fieldd.c % 450 == 0 {
                console_log!("recv_fn hit {} {}!", fieldd.c_name, fieldd.c);
                let r = comm_channels.rx.try_recv();
                let r_msg = match r {
                    Ok(r) => r,
                    Err(e) => e.to_string(),
                };
                console_log!("this is a test {:?}", r_msg);
            }
        }
    }
}
