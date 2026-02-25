use crate::repository::character::{
    CharacterStatsV1, CharacterV1, character_stats_v1__view, character_v1__view, current_character_v1__view,
};
use spacetimedb::{ViewContext, view};

#[view(name = vw_my_character_v1, public)]
pub fn vw_my_character_v1(ctx: &ViewContext) -> Option<CharacterV1> {
    let current = ctx.db.current_character_v1().user_id().find(ctx.sender)?;
    ctx.db.character_v1().character_id().find(current.character_id)
}

#[view(name = vw_my_character_stats_v1, public)]
pub fn vw_my_character_stats_v1(ctx: &ViewContext) -> Option<CharacterStatsV1> {
    let current = ctx.db.current_character_v1().user_id().find(ctx.sender)?;
    ctx.db.character_stats_v1().character_id().find(current.character_id)
}

#[view(name = vw_all_my_character_v1, public)]
pub fn vw_all_my_character_v1(ctx: &ViewContext) -> Vec<CharacterV1> {
    ctx.db.character_v1().user_id().filter(ctx.sender).collect()
}

#[view(name = vw_all_my_character_stats_v1, public)]
pub fn vw_all_my_character_stats_v1(ctx: &ViewContext) -> Vec<CharacterStatsV1> {
    ctx.db.character_stats_v1().user_id().filter(ctx.sender).collect()
}
