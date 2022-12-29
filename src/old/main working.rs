use async_channel::{Receiver, Sender};
use bevy_inspector_egui::{Inspectable, InspectorPlugin};
use std::future::Future;

use bevy::{
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task},
};
use futures_lite::future;
use wasm_bindgen::prelude::wasm_bindgen;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_comm)
        //.add_system(handle_tasks)
        .add_system(send_counter)
        .add_system(recv_counter)
        .run();
}

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
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
    pub api_data: String,
}

#[derive(Component, Debug)]
struct HairColor(String);

#[derive(Component, Debug)]
struct ApiBoy {
    task: Task<String>,
}

fn setup_comm(mut commands: Commands) {
    let api_data = "".to_string();
    let (tx, rx) = async_channel::bounded(1);
    commands.insert_resource(CommChannels { tx, rx, api_data });
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

fn send_counter(
    mut commands: Commands,
    comm_channels: ResMut<CommChannels>,
    mut query: Query<&mut Counter>,
) {
    for mut fieldd in query.iter_mut() {
        if fieldd.c_name == "sender" {
            fieldd.c += 1;
            if fieldd.c % 100 == 0 {
                let pool = AsyncComputeTaskPool::get();
                // console_log!("send_fn hit {} {}!", fieldd.c_name, fieldd.c);
                // let send_msg = format!("im a sender message - {}", fieldd.c.to_string());

                // let s = comm_channels.tx.try_send(send_msg.clone());
                // let s_msg = match s {
                //     Ok(()) => format!("message sent -> {}", send_msg),
                //     Err(e) => format!("{}", e.to_string()),
                // };
                // console_log!("{}", s_msg);
                let cc = comm_channels.tx.clone();
                let task = pool.spawn(async move {
                    let body = reqwest::get("http://localhost:8081/easter")
                        .await
                        .unwrap()
                        .text()
                        .await
                        .unwrap();

                    cc.try_send(body);
                });

                // let b = HairColor("brown".to_string());
                // let a = commands.spawn(b).id();
                // commands.entity(a).insert(ApiBoy { task });
            };
        };
    }
}

// fn send_async(sender_boy: Sender<String>) {
//     wasm_bindgen_futures::spawn_local(async move {
//         let body = reqwest::get("http://localhost:8081/easter")
//             .await
//             .unwrap()
//             .text();

//         let local = tokio::task::LocalSet::new();
//         local
//             .run_until(async move {
//                 tokio::task::spawn_local(body).await.unwrap();
//             })
//             .await;
//         // sender_boy.try_send(local.to_string()).unwrap();
//     });
// }

fn recv_counter(comm_channels: ResMut<CommChannels>, mut query: Query<&mut Counter>) {
    for mut fieldd in query.iter_mut() {
        if fieldd.c_name == "receiver" {
            fieldd.c += 2;
            if fieldd.c % 100 == 0 {
                console_log!("recv_fn hit {} {}!", fieldd.c_name, fieldd.c);
                // let send_msg = format!("im a sender message - {}", fieldd.c.to_string());

                // let s = comm_channels.tx.try_send(send_msg.clone());
                // let s_msg = match s {
                //     Ok(()) => format!("message sent -> {}", send_msg),
                //     Err(e) => format!("{}", e.to_string()),
                // };
                // console_log!("{}", s_msg);
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
pub fn run_async<F>(future: F)
where
    F: Future<Output = ()> + Send + 'static,
{
    wasm_bindgen_futures::spawn_local(async move {
        let local = tokio::task::LocalSet::new();
        local
            .run_until(async move {
                tokio::task::spawn_local(future).await.unwrap();
            })
            .await;
    });
}
