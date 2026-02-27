use self::types::{DirectionV1, MapTileV1};
use crate::repository::world::types::MovementV1;
use spacetimedb::{Timestamp, table};

pub mod reducers;
pub mod services;
pub mod types;
pub mod views;

#[table(accessor = map_v1, private)]
pub struct MapV1 {
    #[primary_key]
    pub map_id: u64,
    #[index(btree)]
    pub sector_key: u64,
    pub x1: u16,
    pub y1: u16,
    pub x2: u16,
    pub y2: u16,
    pub z: u8,
    pub tile: MapTileV1,
}

#[table(accessor = town_temple_v1, private)]
pub struct TownTempleV1 {
    #[auto_inc]
    #[primary_key]
    pub town_temple_id: u64,
    pub name: String,
    pub x: u16,
    pub y: u16,
    pub z: u8,
}

#[table(accessor = movement_cooldown_v1, private)]
pub struct MovementCooldownV1 {
    #[primary_key]
    pub character_id: u64,
    pub can_move_at: Timestamp,
}

#[table(accessor = online_character_position_v1, private)]
#[table(accessor = offline_character_position_v1, private)]
pub struct CharacterPositionV1 {
    #[primary_key]
    pub character_id: u64,
    #[index(btree)]
    pub map_id: u64,
    pub x: u16,
    pub y: u16,
    pub z: u8,
    pub movement: MovementV1,
    pub direction: DirectionV1,
}
