// use async_channel::{Receiver, Sender};
// use bevy_inspector_egui::Inspectable;

use bevy::prelude::*;
use wasm_bindgen::prelude::wasm_bindgen;

mod game_scene;
mod player_setup_scene;
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

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    PlayerSetup,
    InGame,
}

fn main() {
    bevy_app("asdfasdfa".to_string());
}

#[wasm_bindgen]
pub fn bevy_app(user_token: String) {
    console_log!("user: {}", user_token);
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) //.add_plugins(DefaultPlugins)
        .add_state(AppState::PlayerSetup)
        //.insert_resource(WinitSettings::game())
        .add_system_set(
            SystemSet::on_enter(AppState::PlayerSetup)
                .with_system(player_setup_scene::setup_name_scene)
                .with_system(player_setup_scene::setup_start_button)
                .with_system(player_setup_scene::setup_core_stuff)
                .with_system(
                    player_setup_scene::setup_vkeyboard.after(player_setup_scene::setup_core_stuff),
                ),
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
                .with_system(player_setup_scene::cleanup_player_scene),
        )
        .add_system_set(SystemSet::on_enter(AppState::InGame).with_system(game_scene::setup))
        .add_system_set(
            SystemSet::on_update(AppState::InGame).with_system(game_scene::animate_sprite),
        )
        // .add_system(player_setup_scene::username_input)
        // .add_system(player_setup_scene::start_button_system)
        // .add_system(player_setup_scene::vkeyboard_system)
        // .add_system(player_setup_scene::case_vkeyboard_system)
        .run();
}
