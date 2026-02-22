use bevy::prelude::*;
use spacetimedb_sdk::Identity;

/// Event fired when authentication succeeds
#[derive(Event)]
#[allow(dead_code)]
pub struct AuthSuccessEvent {
    pub identity: Identity,
    pub token: String,
}

/// Event fired when a character is selected or created
#[derive(Event)]
#[allow(dead_code)]
pub struct CharacterSelectedEvent {
    pub character_id: u64,
    pub name: String,
}
