use bevy::prelude::*;
use ikaria_shared::protocol::WorldTick;

struct IkariaClientPlugin;

impl Plugin for IkariaClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup);
    }
}

fn startup() {
    let tick = WorldTick { tick: 0 };
    info!("Ikaria client scaffold initialized at tick {}.", tick.tick);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(IkariaClientPlugin)
        .run();
}
