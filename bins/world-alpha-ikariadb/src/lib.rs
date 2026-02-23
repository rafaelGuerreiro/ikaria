use ikariadb_core::ServiceResult;
use spacetimedb::{ReducerContext, reducer};

#[reducer(init)]
pub fn init(ctx: &ReducerContext) {
    ikariadb_core::init(ctx);
}

#[reducer(client_connected)]
pub fn identity_connected(ctx: &ReducerContext) -> ServiceResult<()> {
    ikariadb_core::identity_connected(ctx)
}

#[reducer(client_disconnected)]
pub fn identity_disconnected(ctx: &ReducerContext) {
    ikariadb_core::identity_disconnected(ctx);
}
