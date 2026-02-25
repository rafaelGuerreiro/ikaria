use self::types::{DirectionV1, MapTileV1};
use spacetimedb::{Timestamp, table};

pub mod math;
pub mod services;
pub mod types;
pub mod views;

#[table(accessor = map_v1, private, index(accessor = position_ix, btree(columns = [x, y, z])))]
pub struct MapV1 {
    #[primary_key]
    pub map_id: u64,
    pub x: u16,
    pub y: u16,
    pub z: u16,
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
    pub z: u16,
}

#[table(accessor = character_position_v1, private)]
pub struct CharacterPositionV1 {
    #[primary_key]
    pub character_id: u64,
    pub x: u16,
    pub y: u16,
    pub z: u16,
    pub direction: DirectionV1,
    pub updated_at: Timestamp,
}
