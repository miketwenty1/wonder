use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, InspectorPlugin};
// use futures_lite::future;
// use serde_json::Value;
use wasm_bindgen::prelude::wasm_bindgen;
// use std::{thread, time};

// use wasm_bindgen_futures::JsFuture;
// use web_sys::{Request, RequestInit, RequestMode, Response};

// use async_channel::{Receiver, Sender};
// use std::future::Future;

#[derive(Inspectable, Component)]
struct InspectableType;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
struct ReflectedType;
#[derive(Component)]
struct AnimateTranslation;

#[derive(Resource)]
struct SomeResource(String);

// mod web;

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

fn main() {
    //bevy_app("asdf".to_string());

    bevy_app("tid".to_string());
}

#[wasm_bindgen]
pub fn bevy_app(teststr: String) {
    //println!("testing testing testingtesting");
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(InspectorPlugin::<Counter>::new())
        .insert_resource(SomeResource(teststr))
        .add_startup_system(setup_system)
        //.register_inspectable::<InspectableType>() // tells bevy-inspector-egui how to display the struct in the world inspector
        .add_system(animate_translation)
        .run();
}

#[derive(Resource, Component, Inspectable, Default, Debug)]
pub struct Counter {
    pub c: i32,
    pub c_name: String,
}

fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    my_resource: Res<SomeResource>,
) {
    console_log!("welcome to the game");
    let g = &my_resource.0;
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_style = TextStyle {
        font,
        font_size: 60.0,
        color: Color::WHITE,
    };
    let text_alignment = TextAlignment::CENTER;
    // 2d camera
    commands.spawn(Camera2dBundle::default());
    // Demonstrate changing translation
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(g, text_style.clone()).with_alignment(text_alignment),
            ..default()
        },
        AnimateTranslation,
    ));

    let counter = Counter {
        c: 0,
        c_name: String::from("counter_one"),
    };
    let counter2 = Counter {
        c: 0,
        c_name: String::from("counter_two"),
    };
    commands.spawn(counter);
    commands.spawn(counter2);
}

fn animate_translation(
    time: Res<Time>,
    mut query: Query<&mut Transform, (With<Text>, With<AnimateTranslation>)>,
) {
    for mut transform in &mut query {
        transform.translation.x = 100.0 * time.elapsed_seconds().sin();
        transform.translation.y = 100.0 * time.elapsed_seconds().cos();
    }
}
