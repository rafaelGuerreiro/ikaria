use bevy::prelude::*;
use spacetimedb_sdk::Identity;

/// Stores the authenticated session connection to SpacetimeDB
#[derive(Resource)]
pub struct SessionResource {
    pub connection: ikaria_types::autogen::DbConnection,
    pub identity: Identity,
    #[allow(dead_code)]
    pub token: String,
}

/// Stores the selected character for the current play session
#[derive(Resource)]
pub struct SelectedCharacterResource {
    #[allow(dead_code)]
    pub character_id: u64,
    pub name: String,
}
