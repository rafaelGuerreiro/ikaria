use app_state::AppState;
use bevy::prelude::*;
use features::{character_select::CharacterSelectPlugin, game::GamePlugin, sign_in::SignInPlugin};

pub mod app_state;
pub mod constants;
pub mod events;
pub mod features;
pub mod resources;

struct IkariaClientPlugin;

impl Plugin for IkariaClientPlugin {
    fn build(&self, app: &mut App) {
        app
            // Initialize state machine
            .init_state::<AppState>()
            // Add feature plugins
            .add_plugins(SignInPlugin)
            .add_plugins(CharacterSelectPlugin)
            .add_plugins(GamePlugin);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Ikaria".to_string(),
                resolution: (1024, 768).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(IkariaClientPlugin)
        .run();
}
