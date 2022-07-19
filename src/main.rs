//#![allow(clippy::redundant_field_names)]

use bevy::{prelude::*, window::PresentMode};
use bevy_web_resizer;
use block::BlockPlugin;
use components::Counter;
use debug::DebugPlugin;
use player::PlayerPlugin;

//render::camera::ScalingMode
pub const CLEAR: Color = Color::rgb(0.2, 0.2, 0.2);
pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const TILE_SIZE: f32 = 0.2;
pub const MAN_SPRITESHEET: &str = "mansprite3x3_gimp.png";
pub const SPRITESHEET_SIZE: (f32, f32) = (96.0, 32.0);
pub const SPRITE_SCALE: f32 = 4.;

const TIME_STEP: f32 = 1. / 60.;
const BASE_SPEED: f32 = 200.;
pub struct WinSize {
    pub w: f32,
    pub h: f32,
}

struct GameTextures {
    player: Handle<TextureAtlas>,
}
mod block;
mod components;
mod debug;
mod player;
mod web;

//use debug::DebugPlugin;
//use player::PlayerPlugin;

fn main() {
    let mut num: i32 = 0;
    let height = 900.0;
    let mut app = App::new();
    app.insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            width: height * RESOLUTION,
            height: height,
            title: "Wonder".to_string(),
            present_mode: PresentMode::Fifo,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(BlockPlugin)
        .add_startup_system(setup_system)
        .add_startup_system_to_stage(StartupStage::PreStartup, spritesheet_system)
        // .add_system(call_saul)
        .add_system(counter_system)
        // .add_startup_system_to_stage(StartupStage::PostStartup, player_spawn_system)
        // .add_plugin(PlayerPlugin)
        .add_plugin(DebugPlugin);

    #[cfg(target_arch = "wasm32")]
    {
        app.add_plugin(bevy_web_resizer::Plugin);
    }

    app.run()

    //.add_plugin(bevy_web_resizer::Plugin)
    //.add_startup_system_to_stage(StartupStage::PreStartup, load_character_sprites)

    //app.run();
}

fn setup_system(mut commands: Commands, mut windows: ResMut<Windows>) {
    // camera
    // let mut camera = OrthographicCameraBundle::new_2d();
    // camera.orthographic_projection.top = 1.0;
    // camera.orthographic_projection.bottom = -1.0;
    // camera.orthographic_projection.left = -1.0 * RESOLUTION;
    // camera.orthographic_projection.right = 1.0 * RESOLUTION;

    //camera.orthographic_projection.scaling_mode = ScalingMode::None;

    let window = windows.get_primary_mut().unwrap();
    let (win_w, win_h) = (window.width(), window.height());
    let win_size = WinSize { w: win_w, h: win_h };
    commands.insert_resource(win_size);
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let counter = Counter(0);
    commands.spawn().insert(counter);
}

fn spritesheet_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load(MAN_SPRITESHEET);
    let texture_atlas = TextureAtlas::from_grid_with_padding(
        texture_handle,
        Vec2::new(32.0, 32.0),
        3,
        3,
        Vec2::splat(2.0),
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // commands.insert_resource(GameTextures(texture_atlas_handle));

    let game_textures = GameTextures {
        player: texture_atlas_handle, //asset_server.load(MAN_SPRITESHEET),
    };
    commands.insert_resource(game_textures);
}

// fn main() {
//     let sys = actix::System::new("test");

//     actix::Arbiter::handle().spawn({
//         client::get("http://www.rust-lang.org") // <- Create request builder
//             .header("User-Agent", "Actix-web")
//             .finish()
//             .unwrap()
//             .send() // <- Send http request
//             .map_err(|_| ())
//             .and_then(|response| {
//                 // <- server http response
//                 println!("Response: {:?}", response);
//                 Ok(())
//             })
//     });

//     sys.run();
// }

// fn call_saul() {

// }

fn counter_system(mut query: Query<&mut Counter>) {
    for mut fieldd in query.iter_mut() {
        fieldd.0 += 1;
        if fieldd.0 % 100 == 0 {
            println!("hello {}!", fieldd.0);
            web::pg();
        }
    }
}
