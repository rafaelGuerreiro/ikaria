use crate::{
    app_state::AppState,
    constants::{MODULE_NAME, SPACETIME_URI, TOKEN_FILE_PATH},
    resources::SessionResource,
};
use bevy::prelude::*;
use ikaria_types::autogen::DbConnection;
use spacetimedb_sdk::DbContext;
use std::fs;

pub struct SignInPlugin;

impl Plugin for SignInPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::SignIn), setup_sign_in);
        app.add_systems(Update, attempt_token_auth.run_if(in_state(AppState::SignIn)));
        app.add_systems(Update, handle_auth_success.run_if(in_state(AppState::SignIn)));
        app.add_systems(OnExit(AppState::SignIn), cleanup_sign_in);
    }
}

/// Marker component for sign-in UI entities
#[derive(Component)]
struct SignInUi;

/// Resource to track authentication state
#[derive(Resource, Default)]
struct AuthState {
    attempted_token_auth: bool,
    connection_pending: Option<DbConnection>,
}

fn setup_sign_in(mut commands: Commands) {
    info!("Entering SignIn view");
    commands.init_resource::<AuthState>();

    // Spawn simple UI
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
            BackgroundColor(Color::srgb(0.95, 0.95, 0.95)),
            SignInUi,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Signing in..."),
                TextFont {
                    font_size: 32.0,
                    ..default()
                },
                TextColor(Color::srgb(0.2, 0.2, 0.2)),
            ));
        });
}

fn attempt_token_auth(mut auth_state: ResMut<AuthState>) {
    if auth_state.attempted_token_auth {
        // Advance the connection to allow callbacks to fire
        if let Some(ref conn) = auth_state.connection_pending {
            // Tick the connection to process network events and trigger callbacks
            if let Err(e) = conn.frame_tick() {
                warn!("Connection tick error during auth: {}", e);
            }

            // Try to get identity - if available, auth is successful
            if conn.try_identity().is_some() {
                // Auth successful - handle_auth_success will transition
                return;
            }
        }
        return;
    }

    info!("Attempting token-based authentication...");

    // Try to load saved token
    let token = load_token_from_file();

    if token.is_some() {
        info!("Found saved token, connecting with it...");
    } else {
        info!("No saved token found - connecting as anonymous/new identity");
    }

    // Build connection with token (or None for anonymous/new identity)
    let mut builder = DbConnection::builder().with_uri(SPACETIME_URI).with_module_name(MODULE_NAME);

    if let Some(ref token_str) = token {
        builder = builder.with_token(Some(token_str.as_str()));
    }

    let conn = match builder
        .on_connect(move |_ctx, identity, new_token: &str| {
            info!("Connected! Identity: {:?}", identity);
            // Save the new token
            if let Err(e) = save_token_to_file(new_token) {
                warn!("Failed to save token: {}", e);
            }
        })
        .on_disconnect(|_ctx, err| {
            if let Some(e) = err {
                warn!("Disconnected with error: {:?}", e);
            } else {
                info!("Disconnected normally");
            }
        })
        .build()
    {
        Ok(c) => c,
        Err(e) => {
            error!("Failed to build connection: {}", e);
            auth_state.attempted_token_auth = true;
            return;
        },
    };

    info!("Connection builder succeeded, waiting for on_connect callback...");
    auth_state.connection_pending = Some(conn);
    auth_state.attempted_token_auth = true;
}

fn handle_auth_success(mut commands: Commands, mut auth_state: ResMut<AuthState>, mut next_state: ResMut<NextState<AppState>>) {
    if let Some(ref connection) = auth_state.connection_pending
        && let Some(identity) = connection.try_identity()
    {
        info!("Authentication successful, transitioning to CharacterSelect");

        // Get the token from file (we just saved it)
        let token = load_token_from_file().unwrap_or_default();

        // Take the connection from auth_state
        if let Some(connection) = auth_state.connection_pending.take() {
            // Subscribe to all tables
            connection.subscription_builder().subscribe_to_all_tables();

            // Store session resource
            commands.insert_resource(SessionResource {
                connection,
                identity,
                token,
            });

            // Transition to character select
            next_state.set(AppState::CharacterSelect);
        }
    }
}

fn cleanup_sign_in(mut commands: Commands, query: Query<Entity, With<SignInUi>>) {
    info!("Exiting SignIn view");
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
    commands.remove_resource::<AuthState>();
}

fn load_token_from_file() -> Option<String> {
    fs::read_to_string(TOKEN_FILE_PATH)
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

fn save_token_to_file(token: &str) -> std::io::Result<()> {
    fs::write(TOKEN_FILE_PATH, token)
}
