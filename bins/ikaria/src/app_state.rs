use bevy::prelude::*;

/// Top-level application states representing the main screens/views
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    SignIn,
    CharacterSelect,
    Game,
}
