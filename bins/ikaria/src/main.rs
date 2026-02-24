use crate::constants::GAME_TITLE;
use app_state::AppState;
use bevy::prelude::*;
use screens::{character_select_screen::CharacterSelectPlugin, game_screen::GamePlugin, sign_in_screen::SignInPlugin};

pub mod app_state;
pub mod constants;
pub mod error;
pub mod events;
pub mod file_manager;
pub mod resources;
pub mod screens;
pub mod ui_helpers;
pub mod ui_style;
pub mod worlds;

struct IkariaClientPlugin;

impl Plugin for IkariaClientPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_camera)
            // Initialize state machine
            .init_state::<AppState>()
            // Add feature plugins
            .add_plugins(SignInPlugin)
            .add_plugins(CharacterSelectPlugin)
            .add_plugins(GamePlugin);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: GAME_TITLE.to_string(),
            resolution: (1024, 768).into(),
            ..default()
        }),
        ..default()
    }))
    .add_plugins(IkariaClientPlugin);

    app.run();
}
