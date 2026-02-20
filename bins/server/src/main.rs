use anyhow::Result;
use ikaria_shared::protocol::WorldTick;
use spacetimedb as _;

fn main() -> Result<()> {
    let _boot_tick = WorldTick { tick: 0 };
    println!("Ikaria server scaffold initialized (SpacetimeDB backend).");
    Ok(())
}
