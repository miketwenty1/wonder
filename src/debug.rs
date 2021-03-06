use bevy::prelude::*;
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};

use crate::components::Block;
use crate::components::Player;
use crate::components::Velocity;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app.add_plugin(WorldInspectorPlugin::new())
                .register_inspectable::<Player>()
                .register_inspectable::<Block>()
                .register_inspectable::<Velocity>();
        }
    }
}
