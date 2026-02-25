use self::types::{ClassV1, RaceV1};
use crate::repository::character::types::GenderV1;
use spacetimedb::{Identity, Timestamp, table};

pub mod reducers;
pub mod services;
pub mod types;
pub mod views;

#[table(name = character_v1, private)]
pub struct CharacterV1 {
    #[auto_inc]
    #[primary_key]
    pub character_id: u64,
    #[index(btree)]
    pub user_id: Identity,
    #[unique]
    pub name: String,
    pub display_name: String,
    pub race: RaceV1,
    pub class: ClassV1,
    pub gender: GenderV1,
    pub created_at: Timestamp,
}

#[table(name = character_stats_v1, private)]
pub struct CharacterStatsV1 {
    #[primary_key]
    pub character_id: u64,
    pub level: u16,
    pub experience: u64,
    pub health: u32,
    pub mana: u32,
    pub capacity: u32,
}

#[table(name = current_character_v1, private)]
pub struct CurrentCharacterV1 {
    #[primary_key]
    pub user_id: Identity,
    pub character_id: u64,
    pub signed_in_at: Timestamp,
}
