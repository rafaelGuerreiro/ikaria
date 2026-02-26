use crate::{
    constants::MAP_VIEW_RADIUS,
    repository::{
        character::{CharacterV1, character_v1__view, online_character_v1__view},
        world::{CharacterPositionV1, MapV1, map_v1__view, math, online_character_position_v1__view},
    },
};
use spacetimedb::{ViewContext, view};

const MAP_CAPACITY: usize = 3 * (MAP_VIEW_RADIUS as usize * 2 + 1).pow(2);

#[view(accessor = vw_world_map_v1, public)]
pub fn vw_world_map_v1(ctx: &ViewContext) -> Vec<MapV1> {
    let mut tiles = Vec::with_capacity(MAP_CAPACITY);
    for map_id in iter_map_ids(ctx) {
        if let Some(tile) = ctx.db.map_v1().map_id().find(map_id) {
            tiles.push(tile);
        }
    }
    tiles
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
    // Using 12 because we don't expect too many users playing near each other.
    let mut tiles = Vec::with_capacity(12);
    for map_id in iter_map_ids(ctx) {
        let positions = ctx.db.online_character_position_v1().map_id().filter(map_id);
        for position in positions {
            if let Some(character) = ctx.db.character_v1().character_id().find(position.character_id) {
                tiles.push(character);
            }
        }
    }
    tiles
}

type PositionRange = (u16, u16);
type PositionRanges = (PositionRange, PositionRange, PositionRange);

fn find_ranges(ctx: &ViewContext) -> Option<PositionRanges> {
    let current = ctx.db.online_character_v1().user_id().find(ctx.sender())?;
    let position = ctx
        .db
        .online_character_position_v1()
        .character_id()
        .find(current.character_id)?;

    let min_x = position.x.saturating_sub(MAP_VIEW_RADIUS);
    let max_x = position.x.saturating_add(MAP_VIEW_RADIUS);
    let min_y = position.y.saturating_sub(MAP_VIEW_RADIUS);
    let max_y = position.y.saturating_add(MAP_VIEW_RADIUS);
    let min_z = position.z.saturating_sub(1);
    let max_z = position.z.saturating_add(1);

    Some(((min_x, max_x), (min_y, max_y), (min_z, max_z)))
}

fn iter_map_ids(ctx: &ViewContext) -> impl Iterator<Item = u64> + '_ {
    find_ranges(ctx)
        .into_iter()
        .flat_map(|((min_x, max_x), (min_y, max_y), (min_z, max_z))| {
            (min_x..=max_x)
                .flat_map(move |x| (min_y..=max_y).flat_map(move |y| (min_z..=max_z).map(move |z| math::into_map_id(x, y, z))))
        })
}
