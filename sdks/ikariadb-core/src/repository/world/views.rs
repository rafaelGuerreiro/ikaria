use crate::repository::{
    character::{CharacterV1, character_v1__query, character_v1__view},
    world::{CharacterPositionV1, MapV1, character_position_v1__view, map_v1__query},
};
use spacetimedb::{Query, ViewContext, view};

#[view(name = vw_world_map_v1, public)]
pub fn vw_world_map_v1(ctx: &ViewContext) -> Query<MapV1> {
    ctx.from.map_v1().build()
}

#[view(name = vw_world_my_character_positions_v1, public)]
pub fn vw_world_my_character_positions_v1(ctx: &ViewContext) -> Vec<CharacterPositionV1> {
    ctx.db
        .character_v1()
        .user_id()
        .filter(ctx.sender)
        .filter_map(|character| ctx.db.character_position_v1().character_id().find(character.character_id))
        .collect()
}

#[view(name = vw_nearby_characters_v1, public)]
pub fn vw_nearby_characters_v1(ctx: &ViewContext) -> Query<CharacterV1> {
    ctx.from.character_v1().build()
}
