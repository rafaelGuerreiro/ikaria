use bevy::prelude::*;

struct IkariaClientPlugin;

impl Plugin for IkariaClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup);
    }
}

fn startup() {}

fn main() {
    App::new().add_plugins(DefaultPlugins).add_plugins(IkariaClientPlugin).run();
}
