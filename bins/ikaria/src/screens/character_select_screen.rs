use crate::{
    app_state::AppState,
    resources::{SelectedCharacterResource, SessionResource},
};
use bevy::prelude::*;
use ikaria_types::autogen::{CharacterV1, CharacterV1TableAccess};
use spacetimedb_sdk::Table;

pub struct CharacterSelectPlugin;

impl Plugin for CharacterSelectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::CharacterSelect), setup_character_select);
        app.add_systems(Update, tick_connection.run_if(in_state(AppState::CharacterSelect)));
        app.add_systems(Update, handle_character_selected.run_if(in_state(AppState::CharacterSelect)));
        app.add_systems(Update, handle_character_creation.run_if(in_state(AppState::CharacterSelect)));
        app.add_systems(OnExit(AppState::CharacterSelect), cleanup_character_select);
    }
}

/// Marker component for character select UI
#[derive(Component)]
struct CharacterSelectUi;

/// Component marking character list items
#[derive(Component)]
struct CharacterListItem {
    character_id: u64,
    name: String,
}

/// Component for character name input field
#[derive(Component)]
struct CharacterNameInput;

/// Component for character gender selection
#[derive(Component)]
struct CharacterGenderButton {
    gender: Gender,
}

/// Character creation input state
#[derive(Resource, Default)]
struct CharacterCreationState {
    name: String,
    gender: Option<Gender>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Gender {
    Male,
    Female,
}

fn setup_character_select(mut commands: Commands, session: Res<SessionResource>) {
    info!("Entering CharacterSelect view");

    // Initialize character creation state
    commands.init_resource::<CharacterCreationState>();

    // Get characters for this user
    let characters: Vec<CharacterV1> = session
        .connection
        .db
        .character_v_1()
        .iter()
        .filter(|c| c.user_id == session.identity)
        .collect();

    info!("Found {} characters for user", characters.len());

    // Spawn UI
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
            CharacterSelectUi,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Character Selection"),
                TextFont {
                    font_size: 40.0,
                    ..default()
                },
                TextColor(Color::srgb(0.2, 0.2, 0.2)),
                Node {
                    margin: UiRect::bottom(Val::Px(30.0)),
                    ..default()
                },
            ));

            if characters.is_empty() {
                // Show character creation UI inline
                parent
                    .spawn((Node {
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(20.0),
                        align_items: AlignItems::Center,
                        ..default()
                    },))
                    .with_children(|form_parent| {
                        // Instructions
                        form_parent.spawn((
                            Text::new("Create Your Character"),
                            TextFont {
                                font_size: 28.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.3, 0.3, 0.3)),
                            Node {
                                margin: UiRect::bottom(Val::Px(10.0)),
                                ..default()
                            },
                        ));

                        // Name input section
                        form_parent
                            .spawn((Node {
                                flex_direction: FlexDirection::Column,
                                row_gap: Val::Px(8.0),
                                align_items: AlignItems::Center,
                                ..default()
                            },))
                            .with_children(|name_section| {
                                name_section.spawn((
                                    Text::new("Character Name:"),
                                    TextFont {
                                        font_size: 18.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(0.4, 0.4, 0.4)),
                                ));

                                name_section.spawn((
                                    Text::new("(Type name)"),
                                    TextFont {
                                        font_size: 16.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(0.6, 0.6, 0.6)),
                                    CharacterNameInput,
                                ));
                            });

                        // Gender selection section
                        form_parent
                            .spawn((Node {
                                flex_direction: FlexDirection::Column,
                                row_gap: Val::Px(8.0),
                                align_items: AlignItems::Center,
                                ..default()
                            },))
                            .with_children(|gender_section| {
                                gender_section.spawn((
                                    Text::new("Gender:"),
                                    TextFont {
                                        font_size: 18.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(0.4, 0.4, 0.4)),
                                ));

                                gender_section
                                    .spawn((Node {
                                        flex_direction: FlexDirection::Row,
                                        column_gap: Val::Px(20.0),
                                        ..default()
                                    },))
                                    .with_children(|buttons| {
                                        // Male button
                                        buttons.spawn((
                                            Text::new("[ Male ]"),
                                            TextFont {
                                                font_size: 18.0,
                                                ..default()
                                            },
                                            TextColor(Color::srgb(0.5, 0.5, 0.5)),
                                            CharacterGenderButton { gender: Gender::Male },
                                        ));

                                        // Female button
                                        buttons.spawn((
                                            Text::new("[ Female ]"),
                                            TextFont {
                                                font_size: 18.0,
                                                ..default()
                                            },
                                            TextColor(Color::srgb(0.5, 0.5, 0.5)),
                                            CharacterGenderButton { gender: Gender::Female },
                                        ));
                                    });
                            });

                        // Instructions
                        form_parent.spawn((
                            Text::new("\nPress M for Male, F for Female\nType to enter name\nPress ENTER to create character"),
                            TextFont {
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.5, 0.5, 0.5)),
                            TextLayout {
                                justify: Justify::Center,
                                ..default()
                            },
                            Node {
                                margin: UiRect::top(Val::Px(20.0)),
                                ..default()
                            },
                        ));

                        // Backend note
                        form_parent.spawn((
                            Text::new("Note: Backend reducer for character creation\nis not yet implemented"),
                            TextFont {
                                font_size: 12.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.8, 0.3, 0.3)),
                            TextLayout {
                                justify: Justify::Center,
                                ..default()
                            },
                            Node {
                                margin: UiRect::top(Val::Px(10.0)),
                                ..default()
                            },
                        ));
                    });
            } else {
                // Show character list inline
                parent
                    .spawn((Node {
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(10.0),
                        ..default()
                    },))
                    .with_children(|list_parent| {
                        for character in characters {
                            list_parent.spawn((
                                Text::new(format!(
                                    "{} (Level {} {})",
                                    character.name, character.level, character.vocation
                                )),
                                TextFont {
                                    font_size: 24.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.3, 0.3, 0.3)),
                                CharacterListItem {
                                    character_id: character.character_id,
                                    name: character.name.clone(),
                                },
                            ));
                        }
                    });

                parent.spawn((
                    Text::new("\nPress SPACE to auto-select first character"),
                    TextFont {
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.5, 0.5, 0.5)),
                    Node {
                        margin: UiRect::top(Val::Px(20.0)),
                        ..default()
                    },
                ));
            }
        });
}

fn tick_connection(session: Res<SessionResource>) {
    // Advance the connection each frame to process server messages
    if let Err(e) = session.connection.frame_tick() {
        warn!("Connection tick error: {}", e);
    }
}

fn handle_character_selected(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    _character_query: Query<&CharacterListItem>,
    mut next_state: ResMut<NextState<AppState>>,
    session: Res<SessionResource>,
) {
    // Auto-select first character on SPACE (starter implementation)
    if keyboard.just_pressed(KeyCode::Space) {
        // Get first character for this user
        let first_char = session
            .connection
            .db
            .character_v_1()
            .iter()
            .find(|c| c.user_id == session.identity);

        if let Some(character) = first_char {
            info!("Selected character: {} ({})", character.name, character.character_id);

            commands.insert_resource(SelectedCharacterResource {
                character_id: character.character_id,
                name: character.name.clone(),
            });

            next_state.set(AppState::Game);
        } else {
            info!("No character to select - create character flow would go here");
        }
    }
}

fn handle_character_creation(
    mut creation_state: ResMut<CharacterCreationState>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut name_input_query: Query<&mut Text, With<CharacterNameInput>>,
    mut gender_button_query: Query<(&mut TextColor, &CharacterGenderButton)>,
    _commands: Commands,
    _next_state: ResMut<NextState<AppState>>,
    session: Res<SessionResource>,
) {
    // Only process input if we're in character creation mode (no existing characters)
    let has_characters = session
        .connection
        .db
        .character_v_1()
        .iter()
        .any(|c| c.user_id == session.identity);

    if has_characters {
        return;
    }

    // Handle gender selection via M/F keys
    if keyboard.just_pressed(KeyCode::KeyM) {
        creation_state.gender = Some(Gender::Male);
    } else if keyboard.just_pressed(KeyCode::KeyF) {
        creation_state.gender = Some(Gender::Female);
    }

    // Update gender button visuals
    for (mut color, button) in gender_button_query.iter_mut() {
        if Some(button.gender) == creation_state.gender {
            *color = TextColor(Color::srgb(0.2, 0.6, 0.2)); // Highlight selected
        } else {
            *color = TextColor(Color::srgb(0.5, 0.5, 0.5)); // Default gray
        }
    }

    // Handle name input (simplified - just alphanumeric and backspace)
    for key in keyboard.get_just_pressed() {
        match key {
            KeyCode::Backspace => {
                creation_state.name.pop();
            },
            KeyCode::Space => {
                if creation_state.name.len() < 20 {
                    creation_state.name.push(' ');
                }
            },
            // Letters
            KeyCode::KeyA => creation_state.name.push('A'),
            KeyCode::KeyB => creation_state.name.push('B'),
            KeyCode::KeyC => creation_state.name.push('C'),
            KeyCode::KeyD => creation_state.name.push('D'),
            KeyCode::KeyE => creation_state.name.push('E'),
            KeyCode::KeyG => creation_state.name.push('G'),
            KeyCode::KeyH => creation_state.name.push('H'),
            KeyCode::KeyI => creation_state.name.push('I'),
            KeyCode::KeyJ => creation_state.name.push('J'),
            KeyCode::KeyK => creation_state.name.push('K'),
            KeyCode::KeyL => creation_state.name.push('L'),
            KeyCode::KeyN => creation_state.name.push('N'),
            KeyCode::KeyO => creation_state.name.push('O'),
            KeyCode::KeyP => creation_state.name.push('P'),
            KeyCode::KeyQ => creation_state.name.push('Q'),
            KeyCode::KeyR => creation_state.name.push('R'),
            KeyCode::KeyS => creation_state.name.push('S'),
            KeyCode::KeyT => creation_state.name.push('T'),
            KeyCode::KeyU => creation_state.name.push('U'),
            KeyCode::KeyV => creation_state.name.push('V'),
            KeyCode::KeyW => creation_state.name.push('W'),
            KeyCode::KeyX => creation_state.name.push('X'),
            KeyCode::KeyY => creation_state.name.push('Y'),
            KeyCode::KeyZ => creation_state.name.push('Z'),
            _ => {},
        }
    }

    // Limit name length
    if creation_state.name.len() > 20 {
        creation_state.name.truncate(20);
    }

    // Update name display
    for mut text in name_input_query.iter_mut() {
        if creation_state.name.is_empty() {
            text.0 = "(Type name)".to_string();
        } else {
            text.0 = creation_state.name.clone();
        }
    }

    // Handle character creation on ENTER
    if keyboard.just_pressed(KeyCode::Enter) {
        let name = creation_state.name.trim();

        if name.is_empty() {
            info!("Character name is empty");
            return;
        }

        if creation_state.gender.is_none() {
            info!("Gender not selected");
            return;
        }

        let gender_str = match creation_state.gender.unwrap() {
            Gender::Male => "Male",
            Gender::Female => "Female",
        };

        info!("Attempting to create character: {} ({})", name, gender_str);

        // TODO: Call backend reducer when available
        // For now, we'll just log a message explaining what would happen
        warn!("Backend reducer for character creation is not yet implemented");
        warn!("Would create character with name='{}', gender='{}'", name, gender_str);
        warn!("Please implement a 'create_character' reducer in the ikariadb module");

        // For demonstration purposes, we could create a mock character in the selected resource
        // and transition to the game, but that would be misleading since it won't persist.
        // Instead, we'll just show the message and stay on this screen.
    }
}

fn cleanup_character_select(mut commands: Commands, query: Query<Entity, With<CharacterSelectUi>>) {
    info!("Exiting CharacterSelect view");
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
    commands.remove_resource::<CharacterCreationState>();
}
