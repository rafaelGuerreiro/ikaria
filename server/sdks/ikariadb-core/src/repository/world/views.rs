use crate::{
    constants::{MAP_VIEW_RADIUS, SECTOR_SIZE},
    repository::{
        character::{CharacterV1, character_v1__view, online_character_v1__view},
        world::{
            CharacterPositionV1, MapV1, map_v1__view, occupied_tile_v1__view, online_character_position_v1__view, types::Rect,
        },
    },
};
use spacetimedb::{ViewContext, view};

#[view(accessor = vw_world_map_v1, public)]
pub fn vw_world_map_v1(ctx: &ViewContext) -> Vec<MapV1> {
    let Some((rect, (min_z, max_z))) = find_ranges(ctx) else {
        return Vec::new();
    };

    let mut chunks = Vec::new();

    let sec_min_x = rect.min.x / SECTOR_SIZE;
    let sec_max_x = rect.max.x / SECTOR_SIZE;
    let sec_min_y = rect.min.y / SECTOR_SIZE;
    let sec_max_y = rect.max.y / SECTOR_SIZE;

    for z in min_z..=max_z {
        for sx in sec_min_x..=sec_max_x {
            for sy in sec_min_y..=sec_max_y {
                let sector_key = ((z as u64) << 32) | ((sx as u64) << 16) | (sy as u64);
                for chunk in ctx.db.map_v1().sector_key().filter(sector_key) {
                    if chunk.z == z && Rect::from(&chunk).overlaps(&rect) {
                        chunks.push(chunk);
                    }
                }
            }
        }
    }

    chunks
}

#[view(accessor = vw_world_my_character_position_v1, public)]
pub fn vw_world_my_character_position_v1(ctx: &ViewContext) -> Option<CharacterPositionV1> {
    let current = ctx.db.online_character_v1().user_id().find(ctx.sender())?;
    ctx.db
        .online_character_position_v1()
        .character_id()
        .find(current.character_id)
}

#[view(accessor = vw_nearby_characters_v1, public)]
pub fn vw_nearby_characters_v1(ctx: &ViewContext) -> Vec<CharacterV1> {
    let mut characters = Vec::with_capacity(12);
    for map_id in iter_map_ids(ctx) {
        let Some(occupied) = ctx.db.occupied_tile_v1().map_id().find(map_id) else {
            continue;
        };
        for &character_id in &occupied.character_ids {
            if let Some(character) = ctx.db.character_v1().character_id().find(character_id) {
                characters.push(character);
            }
        }
    }
    characters
}

#[view(accessor = vw_nearby_character_positions_v1, public)]
pub fn vw_nearby_character_positions_v1(ctx: &ViewContext) -> Vec<CharacterPositionV1> {
    let mut positions = Vec::with_capacity(12);
    for map_id in iter_map_ids(ctx) {
        let Some(occupied) = ctx.db.occupied_tile_v1().map_id().find(map_id) else {
            continue;
        };
        for &character_id in &occupied.character_ids {
            if let Some(position) = ctx.db.online_character_position_v1().character_id().find(character_id) {
                positions.push(position);
            }
        }
    }
    positions
}

type ZRange = (u8, u8);

fn find_ranges(ctx: &ViewContext) -> Option<(Rect, ZRange)> {
    let current = ctx.db.online_character_v1().user_id().find(ctx.sender())?;
    let position = ctx
        .db
        .online_character_position_v1()
        .character_id()
        .find(current.character_id)?;

    let rect = Rect::new(
        position.x.saturating_sub(MAP_VIEW_RADIUS),
        position.y.saturating_sub(MAP_VIEW_RADIUS),
        position.x.saturating_add(MAP_VIEW_RADIUS),
        position.y.saturating_add(MAP_VIEW_RADIUS),
    );
    let min_z = position.z.saturating_sub(1);
    let max_z = position.z.saturating_add(1);

    Some((rect, (min_z, max_z)))
}

fn iter_map_ids(ctx: &ViewContext) -> impl Iterator<Item = u64> + '_ {
    use super::types::Vec3;
    find_ranges(ctx).into_iter().flat_map(|(rect, (min_z, max_z))| {
        (rect.min.x..=rect.max.x).flat_map(move |x| {
            (rect.min.y..=rect.max.y).flat_map(move |y| (min_z..=max_z).map(move |z| Vec3::new(x, y, z).map_id()))
        })
    })
}
