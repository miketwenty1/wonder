// use async_channel::{Receiver, Sender};
// use bevy_inspector_egui::Inspectable;
use bevy::prelude::*;
use bevy_egui::EguiPlugin;

use wasm_bindgen::prelude::wasm_bindgen;

mod game_scene;
mod player_setup_scene;
mod qr_code_overlay_scene;
mod utils;

//const SERVER_URL: &str = "https://satoshisettlers.com";
const SERVER_URL: &str = "http://localhost:8081";

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    PlayerSetup,
    InGame,
    QrOverlay,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppQr {
    Off,
    Fifty,
}

fn main() {
    bevy_app("asdfasdfa".to_string());
    //let a = web_sys::clip; //.write_text(sometext);
}

// fn copy_to_clipboard(&self) -> bool {
//     if let Some(selected_text) = self.text_buffer.selected_text() {
//         let navigator = web_sys::window(). .navigator();
//         if let Some(clipboard) = navigator.clipboard() {
//             let _ = clipboard.write_text(&selected_text);
//             return true;
//         } else {
//             log::warn!("no navigator clipboard");
//         }
//     }
//     false
// }

#[wasm_bindgen]
pub fn bevy_app(user_token: String) {
    info!("user: {}", user_token);
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(bevy::log::LogPlugin {
                    // level: bevy::log::Level::TRACE,
                    filter: "wgpu=warn,bevy_ecs=info".to_string(),
                    ..default()
                }),
        ) //.add_plugins(DefaultPlugins)
        .init_resource::<qr_code_overlay_scene::CurrentQrCode>()
        .add_plugin(EguiPlugin)
        .add_state(AppState::PlayerSetup)
        .add_state(AppQr::Off)
        // .add_state(QrA::Fi)
        .add_system_set(
            SystemSet::on_enter(AppState::PlayerSetup)
                .with_system(player_setup_scene::setup_name_scene)
                .with_system(player_setup_scene::setup_start_button)
                .with_system(utils::setup_music)
                .with_system(player_setup_scene::setup_vkeyboard), //.with_system(player_setup_scene::setup_overlay),
        )
        .add_system_set(
            SystemSet::on_update(AppState::PlayerSetup)
                .with_system(player_setup_scene::username_input)
                .with_system(player_setup_scene::start_button_system)
                .with_system(player_setup_scene::vkeyboard_system)
                .with_system(player_setup_scene::case_vkeyboard_system),
        )
        .add_system_set(
            SystemSet::on_exit(AppState::PlayerSetup)
                .with_system(player_setup_scene::cleanup_player_scene), //.with_system(player_setup_scene::clean_overlay),
        )
        .add_system_set(
            SystemSet::on_enter(AppState::InGame)
                .with_system(game_scene::setup)
                .with_system(game_scene::setup_pay_button)
                .with_system(game_scene::setup_comm),
        )
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                //.with_system(utils::setup_music)
                .with_system(game_scene::animate_sprite)
                .with_system(game_scene::api_sender)
                .with_system(game_scene::api_receiver)
                .with_system(game_scene::pay_button_system),
        )
        .add_system_set(
            SystemSet::on_enter(AppQr::Fifty).with_system(qr_code_overlay_scene::setup_qr),
        )
        .add_system_set(
            SystemSet::on_update(AppQr::Fifty).with_system(qr_code_overlay_scene::update_qr),
        )
        .add_system_set(
            SystemSet::on_exit(AppQr::Fifty).with_system(qr_code_overlay_scene::clean_qr),
        )
        .run();
}
