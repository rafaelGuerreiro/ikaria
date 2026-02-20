pub mod account;
pub mod event;
pub mod item;
pub mod progression;
pub mod world;

use crate::{error::ServiceResult, repository::event::services::EventReducerContext};
use spacetimedb::ReducerContext;

pub fn init(ctx: &ReducerContext) {
    ctx.publish().system_init();
}

pub fn identity_connected(ctx: &ReducerContext) -> ServiceResult<()> {
    ctx.publish().user_signed_in(ctx.sender)?;
    Ok(())
}

pub fn identity_disconnected(ctx: &ReducerContext) {
    ctx.publish().user_signed_out(ctx.sender);
}
