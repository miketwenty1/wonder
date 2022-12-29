use async_channel::{Receiver, Sender};
use bevy_inspector_egui::Inspectable;

use bevy::{prelude::*, tasks::IoTaskPool};
use wasm_bindgen::prelude::wasm_bindgen;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_comm)
        .add_system(api_sender)
        .add_system(api_receiver)
        .run();
}

#[derive(Resource, Component, Inspectable, Default, Debug)]
pub struct Counter {
    pub c: i32,
    pub c_name: String,
}
#[derive(Resource, Clone)]
pub struct CommChannels {
    pub tx: Sender<String>,
    pub rx: Receiver<String>,
}

#[wasm_bindgen]
extern "C" {

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

fn setup_comm(mut commands: Commands) {
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

fn api_sender(comm_channels: ResMut<CommChannels>, mut query: Query<&mut Counter>) {
    for mut fieldd in query.iter_mut() {
        if fieldd.c_name == "sender" {
            fieldd.c += 1;
            // Every 100 clicks it tries triggers this system
            if fieldd.c % 100 == 0 {
                let pool = IoTaskPool::get();
                let cc = comm_channels.tx.clone();
                console_log!("sender_fn hit {} {}!", fieldd.c_name, fieldd.c);
                let task = pool.spawn(async move {
                    let api_response_text = reqwest::get("http://localhost:8081/easter")
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

fn api_receiver(comm_channels: ResMut<CommChannels>, mut query: Query<&mut Counter>) {
    for mut fieldd in query.iter_mut() {
        if fieldd.c_name == "receiver" {
            fieldd.c += 2;
            // Every 100 clicks it tries triggers this system
            if fieldd.c % 100 == 0 {
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
