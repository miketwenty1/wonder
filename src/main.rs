// use bevy_inspector_egui::Inspectable;
use bevy::prelude::*;
use bevy_egui::EguiPlugin;

use wasm_bindgen::prelude::wasm_bindgen;

mod comms;
mod game_scene;
mod init_setup;
mod player_setup_scene;
mod qr_code_overlay_scene;
mod sharedstructs;
mod utils;

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
    bevy_app("localtesting".to_string());
}

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
        .init_resource::<qr_code_overlay_scene::CurrentQrString>()
        .add_plugin(EguiPlugin)
        .add_state(AppState::PlayerSetup)
        .add_state(AppQr::Off)
        // .add_state(QrA::Fi)
        .add_system_set(
            SystemSet::on_enter(AppState::PlayerSetup)
                .with_system(init_setup::setup)
                .with_system(comms::setup_comm)
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
                .with_system(game_scene::setup_pay_button),
        )
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(game_scene::animate_sprite)
                .with_system(comms::api_height_sender)
                .with_system(comms::api_receiver)
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
