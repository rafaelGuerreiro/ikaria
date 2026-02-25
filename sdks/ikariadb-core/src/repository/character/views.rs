use crate::repository::character::{CharacterV1, character_v1__view, current_character_v1__view};
use spacetimedb::{ViewContext, view};

#[view(name = vw_my_character_v1, public)]
pub fn vw_my_character_v1(ctx: &ViewContext) -> Option<CharacterV1> {
    let current = ctx.db.current_character_v1().user_id().find(ctx.sender)?;
    ctx.db.character_v1().character_id().find(current.character_id)
}
