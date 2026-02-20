pub mod types;

use self::types::DirectionV1;
use spacetimedb::{Timestamp, table};

#[table(name = town_v1, private)]
pub struct TownV1 {
    #[auto_inc]
    #[primary_key]
    pub town_id: u64,
    pub name: String,
    pub temple_x: u16,
    pub temple_y: u16,
    pub temple_z: u16,
}

#[table(name = character_position_v1, private)]
pub struct CharacterPositionV1 {
    #[auto_inc]
    #[primary_key]
    pub character_position_id: u64,
    #[index(btree)]
    pub character_id: u64,
    #[index(btree)]
    pub town_id: u64,
    pub x: u16,
    pub y: u16,
    pub z: u16,
    pub direction: DirectionV1,
    pub updated_at: Timestamp,
}
