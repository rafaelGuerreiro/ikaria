use crate::error::ServiceResult;
use spacetimedb::{ReducerContext, reducer};

pub mod error;
pub mod extend;
pub mod repository;

#[reducer(init)]
pub fn init(ctx: &ReducerContext) {
    repository::init(ctx);
}

#[reducer(client_connected)]
pub fn identity_connected(ctx: &ReducerContext) -> ServiceResult<()> {
    repository::identity_connected(ctx)?;
    Ok(())
}

#[reducer(client_disconnected)]
pub fn identity_disconnected(ctx: &ReducerContext) {
    repository::identity_disconnected(ctx);
}
