use bevy::prelude::*;

pub fn setup_music(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    let a = audio.play_with_settings(
        asset_server.load("sounds/Windless Slopes.ogg"),
        PlaybackSettings::LOOP.with_volume(0.20),
    );

    // let music = asset_server.load("sounds/Windless Slopes.ogg");
    // audio.play(music);
}
