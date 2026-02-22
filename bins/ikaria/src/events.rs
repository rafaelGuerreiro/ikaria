use bevy::prelude::*;
use spacetimedb_sdk::Identity;

/// Event fired when authentication succeeds
#[derive(Event)]
pub struct AuthSuccessEvent {
    pub identity: Identity,
    pub token: String,
}

/// Event fired when a character is selected or created
#[derive(Event)]
pub struct CharacterSelectedEvent {
    pub character_id: u64,
    pub name: String,
}
