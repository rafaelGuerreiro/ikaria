use crate::{
    app_state::AppState,
    constants::SPACETIME_URI,
    external_resources,
    resources::SessionResource,
    ui_helpers::{self, DANGER_BUTTON, PRIMARY_BUTTON, SELECTOR_BUTTON},
    ui_style::sign_in as sign_in_ui,
    worlds::{WorldDefinition, load_worlds},
};
use bevy::prelude::*;
use ikaria_types::autogen::DbConnection;
use spacetimedb_sdk::DbContext;

pub struct SignInPlugin;

impl Plugin for SignInPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::SignIn), setup_sign_in);
        app.add_systems(Update, handle_world_selector_interaction.run_if(in_state(AppState::SignIn)));
        app.add_systems(Update, sync_world_selection_text.run_if(in_state(AppState::SignIn)));
        app.add_systems(Update, sync_sign_in_hint_text.run_if(in_state(AppState::SignIn)));
        app.add_systems(Update, handle_sign_in_button_interaction.run_if(in_state(AppState::SignIn)));
        app.add_systems(Update, attempt_token_auth.run_if(in_state(AppState::SignIn)));
        app.add_systems(Update, handle_auth_success.run_if(in_state(AppState::SignIn)));
        app.add_systems(OnExit(AppState::SignIn), cleanup_sign_in);
    }
}

/// Marker component for sign-in UI entities
#[derive(Component)]
pub(super) struct SignInUi;

#[derive(Component)]
pub(super) struct SignInButton;

#[derive(Component)]
pub(super) struct WorldSelectionText;

#[derive(Component)]
pub(super) struct SignInHintText;

#[derive(Component)]
pub(super) struct WorldSelectorButton {
    pub(super) step: isize,
}

/// Resource to track authentication state
#[derive(Resource, Default)]
struct AuthState {
    auth_requested: bool,
    attempted_token_auth: bool,
    connection_pending: Option<DbConnection>,
    worlds: Vec<WorldDefinition>,
    selected_world_index: usize,
    world_config_error: Option<String>,
}

impl AuthState {
    fn selected_world(&self) -> Option<&WorldDefinition> {
        self.worlds.get(self.selected_world_index)
    }
}

fn setup_sign_in(mut commands: Commands) {
    info!("Entering SignIn view");

    let (worlds, world_config_error) = match load_worlds() {
        Ok(worlds) => (worlds, None),
        Err(e) => {
            error!("Failed to load world config: {}", e);
            (Vec::new(), Some(e.to_string()))
        },
    };

    let initial_world_label = worlds
        .first()
        .map(|world| world.name.clone())
        .unwrap_or_else(|| sign_in_ui::WORLD_UNAVAILABLE_TEXT.to_string());

    let initial_hint = match &world_config_error {
        Some(error) => format!("{}{}", sign_in_ui::HINT_CONFIG_ERROR_PREFIX, error),
        None if worlds.is_empty() => sign_in_ui::HINT_NO_WORLDS_TEXT.to_string(),
        None => sign_in_ui::HINT_SELECT_WORLD_TEXT.to_string(),
    };

    commands.insert_resource(AuthState {
        worlds,
        world_config_error,
        ..default()
    });

    super::sign_in_screen_ui::spawn_sign_in_ui(&mut commands, initial_world_label, initial_hint);
}

#[allow(clippy::type_complexity)]
fn handle_world_selector_interaction(
    mut interaction_query: Query<
        (&Interaction, &WorldSelectorButton, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut auth_state: ResMut<AuthState>,
) {
    for (interaction, selector, mut color) in interaction_query.iter_mut() {
        *color = ui_helpers::interaction_background(*interaction, SELECTOR_BUTTON);

        if *interaction != Interaction::Pressed {
            continue;
        }

        if auth_state.auth_requested || auth_state.attempted_token_auth || auth_state.worlds.is_empty() {
            continue;
        }

        let total = auth_state.worlds.len() as isize;
        let current = auth_state.selected_world_index as isize;
        auth_state.selected_world_index = (current + selector.step).rem_euclid(total) as usize;
    }
}

fn sync_world_selection_text(auth_state: Res<AuthState>, mut text_query: Query<&mut Text, With<WorldSelectionText>>) {
    let label = auth_state
        .selected_world()
        .map(|world| world.name.clone())
        .unwrap_or_else(|| sign_in_ui::WORLD_UNAVAILABLE_TEXT.to_string());

    for mut text in text_query.iter_mut() {
        text.0 = label.clone();
    }
}

fn sync_sign_in_hint_text(auth_state: Res<AuthState>, mut hint_query: Query<&mut Text, With<SignInHintText>>) {
    let hint = match &auth_state.world_config_error {
        Some(error) => format!("{}{}", sign_in_ui::HINT_CONFIG_ERROR_PREFIX, error),
        None => match auth_state.selected_world() {
            Some(world) => format!("{}{}", sign_in_ui::HINT_SELECTED_WORLD_PREFIX, world.name),
            None => sign_in_ui::HINT_NO_WORLDS_TEXT.to_string(),
        },
    };

    for mut text in hint_query.iter_mut() {
        text.0 = hint.clone();
    }
}

#[allow(clippy::type_complexity)]
fn handle_sign_in_button_interaction(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>, With<SignInButton>),
    >,
    mut auth_state: ResMut<AuthState>,
) {
    let can_authenticate = auth_state.world_config_error.is_none() && auth_state.selected_world().is_some();
    let palette = if can_authenticate { PRIMARY_BUTTON } else { DANGER_BUTTON };

    for (interaction, mut color) in interaction_query.iter_mut() {
        *color = ui_helpers::interaction_background(*interaction, palette);

        if *interaction != Interaction::Pressed {
            continue;
        }

        if can_authenticate {
            auth_state.auth_requested = true;
        } else {
            warn!("Sign-in blocked: world configuration is not available");
        }
    }
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

    if !auth_state.auth_requested {
        return;
    }

    let selected_world = match auth_state.selected_world().cloned() {
        Some(world) => world,
        None => {
            error!("Cannot authenticate: no world selected");
            return;
        },
    };

    info!(
        "Attempting token-based authentication for world '{}' ({})",
        selected_world.name, selected_world.module_name
    );

    // Try to load saved token
    let token = match external_resources::read_saved_token() {
        Ok(token) => token,
        Err(e) => {
            warn!("Unable to load token file: {}", e);
            None
        },
    };

    if token.is_some() {
        info!("Found saved token, connecting with it...");
    } else {
        info!("No saved token found - connecting as anonymous/new identity");
    }

    // Build connection with token (or None for anonymous/new identity)
    let mut builder = DbConnection::builder()
        .with_uri(SPACETIME_URI)
        .with_module_name(selected_world.module_name.as_str());

    if let Some(ref token_str) = token {
        builder = builder.with_token(Some(token_str.as_str()));
    }

    let conn = match builder
        .on_connect(move |_ctx, identity, new_token: &str| {
            info!("Connected! Identity: {:?}", identity);
            // Save the new token
            if let Err(e) = external_resources::save_token(new_token) {
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
        let selected_world = match auth_state.selected_world().cloned() {
            Some(world) => world,
            None => {
                error!("Authentication succeeded without a selected world");
                return;
            },
        };

        info!(
            "Authentication successful for world '{}', transitioning to CharacterSelect",
            selected_world.name
        );

        // Get the token from file (we just saved it)
        let token = match external_resources::read_saved_token() {
            Ok(Some(token)) => token,
            Ok(None) => String::new(),
            Err(e) => {
                warn!("Unable to load token file: {}", e);
                String::new()
            },
        };

        // Take the connection from auth_state
        if let Some(connection) = auth_state.connection_pending.take() {
            // Subscribe to all tables
            connection.subscription_builder().subscribe_to_all_tables();

            // Store session resource
            commands.insert_resource(SessionResource {
                connection,
                identity,
                token,
                world: selected_world,
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
