use bevy::prelude::*;

//const SERVER_URL: &str = "https://satoshisettlers.com";
const SERVER_URL: &str = "http://localhost:8081";

#[derive(Resource, Clone)]
pub struct ActixServerURI(pub String);

pub fn setup(mut commands: Commands) {
    commands.insert_resource(ActixServerURI(SERVER_URL.to_string()));
}
