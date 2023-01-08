use bevy::prelude::*;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::character_components::{Balance, Health, Location, Name, Player};
use crate::AppQr;
use crate::AppState;

// #[derive(Resource, Clone)]
// pub struct ClipboardChannel {
//     pub tx: Sender<JsFuture>,
//     pub rx: Receiver<String>,
// }

#[derive(Resource)]
pub struct Counter(i32);

#[derive(Debug, Clone, Eq, PartialEq, Hash, Component)]
pub struct UsernameText(String);

#[derive(Component)]
pub struct StartButton;

#[derive(Component, Debug)]
pub struct KeyBoardButton(char);

#[derive(Component)]
pub struct CapitalizeToggle(bool);

#[derive(Component)]
enum KeyType {
    Letter,
    Function,
    Number,
}

#[derive(Resource)]
pub struct PlayerTextEntity {
    input_prompt_text: Entity,
    pub username: Entity,
}

// #[derive(Resource)]
// pub struct KeyboardEntity {
//     keyboard: Vec<Entity>,
// }

#[derive(Resource)]
pub struct StartButtonEntity {
    start_button: Entity,
}

#[derive(Component)]
pub struct Capitalizable;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.65, 0.25, 0.85);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);
//const KEYBOARD_COLOR: Color = Color::rgb(0.4, 0.4, 0.4);
const USERNAME_LENGTH: usize = 21;
const ACCEPTABLE_CHARS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890 ";
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

pub fn setup_name_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.insert_resource(Counter(0));
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_style = TextStyle {
        font,
        font_size: 60.0,
        color: Color::WHITE,
    }; //directives: web_sys_unstable_apis is disabled

    let input_prompt_text = commands
        .spawn((Text2dBundle {
            text: Text::from_section("Type in a name:", text_style.clone())
                .with_alignment(TextAlignment::CENTER),
            transform: Transform::from_xyz(0.0, 250.0, 1.0),
            ..default()
        },))
        .id();

    let username = commands
        .spawn((
            Text2dBundle {
                text: Text::from_section("", text_style).with_alignment(TextAlignment::CENTER),
                transform: Transform::from_xyz(0.0, 180.0, 1.0),
                ..default()
            },
            UsernameText("".to_string()),
        ))
        .id();
    commands.insert_resource(PlayerTextEntity {
        input_prompt_text,
        username,
    });
}

pub fn username_input(
    mut char_evr: EventReader<ReceivedCharacter>,
    keys: Res<Input<KeyCode>>,
    //mut string: Local<String>,
    mut username: Query<&mut Text, With<UsernameText>>,
    mut q_button: Query<&mut Visibility, With<StartButton>>,
    mut counter: ResMut<Counter>,
) {
    counter.0 += 1;
    //info!("{}", counter.0);
    let user_string_result = username.get_single_mut();
    let mut user_string = match user_string_result {
        Ok(r) => r.sections[0].value.clone(),
        Err(_) => "Satoshi".to_string(),
    };
    // let mut user_string = username.get_single_mut().unwrap().sections[0].value.clone();
    // only allow 21 char names
    if user_string.len() < USERNAME_LENGTH {
        for ev in char_evr.iter() {
            console_log!("Got char: '{}'", ev.char);
            if ACCEPTABLE_CHARS.contains(ev.char) {
                user_string.push(ev.char);
            } else {
                console_log!("got invalid char from physical input");
            }
        }
    }
    // if keys.just_pressed(KeyCode::Return) {
    //     console_log!("Text input: {}", *string);
    //     string.clear();
    // }
    if keys.just_pressed(KeyCode::Back) {
        console_log!("trying to delete a char");
    }
    let r_vis = q_button.get_single_mut();

    match r_vis {
        Ok(mut v) => {
            if !user_string.is_empty() {
                v.is_visible = true;
            } else {
                v.is_visible = false;
            }
        }
        Err(_) => return,
    }

    //let s = string.clone();
    username.get_single_mut().unwrap().sections[0].value = user_string;
}

pub fn setup_start_button(mut commands: Commands, asset_server: Res<AssetServer>) {
    // ui camera
    let start_button = commands
        .spawn((
            ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                    // center button
                    //margin: UiRect::all(Val::Percent(50.0)),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically align inner child text
                    align_items: AlignItems::Center,
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        top: Val::Px(250.0),
                        left: Val::Px(562.0),
                        ..default()
                    },
                    ..default()
                },
                //transform: Transform::from_xyz(100.0, -200.0, 1.0),
                background_color: NORMAL_BUTTON.into(),

                ..default()
            },
            StartButton,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Start",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        })
        .id();
    commands.insert_resource(StartButtonEntity { start_button });
}

#[allow(clippy::type_complexity)]
pub fn start_button_system(
    mut state: ResMut<State<AppState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<StartButton>),
    >,
    mut text_query: Query<&mut Text, Without<UsernameText>>,
    mut username: Query<&mut Text, With<UsernameText>>,
    mut commands: Commands,
) {
    let user_string_result = username.get_single_mut();
    let user_string = match user_string_result {
        Ok(r) => r.sections[0].value.clone(),
        Err(_) => "Satoshi".to_string(),
    };

    for (interaction, mut color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text.sections[0].value = "Start".to_string();
                *color = PRESSED_BUTTON.into();
                console_log!("start pressed");
                let clean_username = clean_name(&user_string);
                console_log!("name selected: {}", clean_username);

                commands.spawn((
                    Player,
                    Name(clean_username.to_string()),
                    Balance(0),
                    Health(10),
                    Location(0),
                ));
                state.set(AppState::InGame).unwrap();
            }
            Interaction::Hovered => {
                text.sections[0].value = "Ready?".to_string();
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                text.sections[0].value = "Start".to_string();
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

pub fn setup_vkeyboard(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    //mut keyboard_res: Res<KeyboardEntity>,
) {
    let key_chars = ["1234567890<", "qwertyuiop", "^asdfghjkl", "zxcvbnm "];

    let number_set = "1234567890";
    let function_set = "<^ ";
    let letter_set = "abcdefghijklmnopqrstuvwxyz";

    //let mut loop_counter = 0;
    let reverse_x_by = [0.0, 520.0, 520.0, 430.0];
    let mut y_offset = 300.0;
    let mut x_offset = 320.0;

    for (loop_counter, row) in key_chars.into_iter().enumerate() {
        x_offset -= reverse_x_by[loop_counter];
        y_offset += 50.0;
        for char in row.chars() {
            x_offset += 50.0;
            let keyboard = commands
                .spawn((
                    ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(40.0), Val::Px(40.0)),
                            // center button
                            // margin: UiRect::all(Val::Auto),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            position_type: PositionType::Absolute,
                            position: UiRect {
                                top: Val::Px(y_offset),
                                left: Val::Px(x_offset),
                                ..default()
                            },

                            ..default()
                        },
                        //transform: Transform::from_xyz(100.0, -200.0, 1.0),
                        background_color: NORMAL_BUTTON.into(),

                        ..default()
                    },
                    KeyBoardButton(char),
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        char,
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 32.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                })
                .id();

            if letter_set.contains(char) {
                // (KeyType::Letter); //.insert();
                commands.entity(keyboard).insert(Capitalizable);
                commands.entity(keyboard).insert(KeyType::Letter);
            } else if number_set.contains(char) {
                commands.entity(keyboard).insert(KeyType::Number);
            } else if function_set.contains(char) {
                commands.entity(keyboard).insert(KeyType::Function);
            } else {
                console_log!("a key is not defined as a type")
            }
            // let ent_vec = &mut keyboard_res.keyboard;
            // ent_vec.push(keyboard);
            // commands.insert_resource(KeyboardEntity {
            //     keyboard: ent_vec.to_vec(),
            // });
        }
    }
    commands.spawn(CapitalizeToggle(false));
    // ui camera
}

#[allow(clippy::type_complexity)]
pub fn vkeyboard_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &KeyBoardButton),
        (Changed<Interaction>, With<KeyBoardButton>),
    >,
    //mut text_query: Query<&mut Text>,
    mut username: Query<&mut Text, With<UsernameText>>,
    mut c_toggle: Query<&mut CapitalizeToggle>,
    //mut state: ResMut<State<AppState>>,
) {
    let user_string_result = username.get_single_mut();
    let mut user_string = match user_string_result {
        Ok(r) => r.sections[0].value.clone(),
        Err(_) => return, //"Satoshi".to_string()
    };

    for (interaction, mut color, key) in &mut interaction_query {
        let k = key.0;
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                match k {
                    '<' => {
                        user_string.pop();
                    }
                    '^' => {
                        c_toggle.get_single_mut().unwrap().0 = !c_toggle.single_mut().0;
                        console_log!("capitalize is: {}", c_toggle.single_mut().0);
                    }
                    _ => {
                        if user_string.len() < USERNAME_LENGTH && ACCEPTABLE_CHARS.contains(k) {
                            if c_toggle.get_single_mut().unwrap().0 {
                                user_string.push(k.to_ascii_uppercase());
                            } else {
                                user_string.push(k);
                            }
                        } else {
                            console_log!("got invalid char from vkeyboard");
                        }
                    }
                }
                console_log!("this from keyboard... {}", k);
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }

    username.get_single_mut().unwrap().sections[0].value = user_string;

    //let user_result2 = username.get_single_mut()
}

pub fn case_vkeyboard_system(
    toggle: Query<&CapitalizeToggle>,
    mut keyboard_keys_query: Query<(&mut KeyBoardButton, &Children), With<Capitalizable>>,
    mut text_query: Query<&mut Text>,
) {
    let tog_result = toggle.get_single(); //.unwrap().0;

    let tog = match tog_result {
        Ok(r) => r.0,
        Err(_) => false,
    };

    if tog {
        for (key, children) in &mut keyboard_keys_query {
            let mut text = text_query.get_mut(children[0]).unwrap();
            text.sections[0].value = key.0.to_ascii_uppercase().to_string();
        }
    } else {
        for (key, children) in &mut keyboard_keys_query {
            let mut text = text_query.get_mut(children[0]).unwrap();
            text.sections[0].value = key.0.to_ascii_lowercase().to_string();
        }
    }
}

pub fn cleanup_player_scene(
    mut commands: Commands,
    //keyboard: Res<KeyboardEntity>,
    start_button: Res<StartButtonEntity>,
    player_text: Res<PlayerTextEntity>,
    mut keyboard_keys: Query<(Entity, With<KeyBoardButton>)>,
) {
    for key in keyboard_keys.iter_mut() {
        commands.entity(key.0).despawn_recursive();
    }

    commands
        .entity(start_button.start_button)
        .despawn_recursive();
    //commands.entity(player_text.username).despawn_recursive();
    commands
        .entity(player_text.input_prompt_text)
        .despawn_recursive();
}

pub fn clean_name(user_input: &str) -> &str {
    if !user_input.chars().all(|x| ACCEPTABLE_CHARS.contains(x)) {
        return "Anon de ausu";
    }

    if user_input.is_empty() {
        return "Anon de umbra";
    }
    if user_input.len() > 21 {
        return "Anon de longitudo";
    }
    if user_input.contains("  ") {
        return "Anon de inanis";
    }
    let f = user_input.chars().next().unwrap();
    let l = user_input.chars().last().unwrap();
    if f.to_string() == " " {
        return "Anon de primis";
    }
    if l.to_string() == " " {
        "Anon de novissime"
    } else {
        user_input
    }
}

// pub fn setup_overlay(mut state: ResMut<State<AppState>>) {

// }

// pub fn clean_overlay(mut state: ResMut<State<AppState>>) {
//     state.pop().unwrap();
// }
