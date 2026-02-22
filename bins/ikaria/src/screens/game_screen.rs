use crate::{
    app_state::AppState,
    resources::{SelectedCharacterResource, SessionResource},
};
use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), setup_game);
        app.add_systems(Update, tick_connection.run_if(in_state(AppState::Game)));
        app.add_systems(Update, handle_input.run_if(in_state(AppState::Game)));
        app.add_systems(OnExit(AppState::Game), cleanup_game);
    }
}

/// Marker component for game UI
#[derive(Component)]
struct GameUi;

fn setup_game(mut commands: Commands, _session: Res<SessionResource>, character: Res<SelectedCharacterResource>) {
    info!("Entering Game view with character: {}", character.name);

    // Spawn game UI placeholder
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::srgb(0.2, 0.5, 0.2)), // Green for grassland
            GameUi,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(format!("Welcome, {}!", character.name)),
                TextFont {
                    font_size: 40.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 1.0, 1.0)),
                Node {
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                },
            ));

            parent.spawn((
                Text::new("Game world placeholder\n\nPress ESC to return to character select"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                TextLayout {
                    justify: Justify::Center,
                    ..default()
                },
            ));
        });
}

fn tick_connection(session: Res<SessionResource>) {
    // Advance the connection each frame to process server messages
    if let Err(e) = session.connection.frame_tick() {
        warn!("Connection tick error: {}", e);
    }
}

fn handle_input(keyboard: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<AppState>>) {
    // ESC to return to character select (for testing)
    if keyboard.just_pressed(KeyCode::Escape) {
        info!("Returning to character select");
        next_state.set(AppState::CharacterSelect);
    }
}

fn cleanup_game(mut commands: Commands, query: Query<Entity, With<GameUi>>) {
    info!("Exiting Game view");
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
