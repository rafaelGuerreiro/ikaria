use crate::{
    app_state::AppState,
    resources::{SelectedCharacterResource, SessionResource},
    ui_helpers::{
        self, CHARACTER_BUTTON, GENDER_BUTTON, GENDER_SELECTED_BUTTON, NAME_INPUT_ACTIVE, NAME_INPUT_INACTIVE, PRIMARY_BUTTON,
    },
    ui_style::{self, character_select as character_ui, palette},
};
use bevy::{
    input::{ButtonState, keyboard::KeyboardInput},
    prelude::*,
};
use ikaria_types::autogen::{CharacterV1, CharacterV1TableAccess};
use spacetimedb_sdk::Table;

pub struct CharacterSelectPlugin;

impl Plugin for CharacterSelectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::CharacterSelect), setup_character_select);
        app.add_systems(Update, tick_connection.run_if(in_state(AppState::CharacterSelect)));
        app.add_systems(Update, handle_character_selected.run_if(in_state(AppState::CharacterSelect)));
        app.add_systems(
            Update,
            handle_character_creation_interactions.run_if(in_state(AppState::CharacterSelect)),
        );
        app.add_systems(
            Update,
            sync_character_creation_visuals.run_if(in_state(AppState::CharacterSelect)),
        );
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

/// Component for character name input button
#[derive(Component)]
struct CharacterNameInputButton;

/// Component for character name input text
#[derive(Component)]
struct CharacterNameInputText;

/// Component for character gender selection
#[derive(Component)]
struct CharacterGenderButton {
    gender: Gender,
}

#[derive(Component)]
struct CreateCharacterButton;

#[derive(Component)]
struct CharacterFormStatusText;

/// Character creation input state
#[derive(Resource)]
struct CharacterCreationState {
    name: String,
    gender: Option<Gender>,
    name_input_active: bool,
    create_requested: bool,
    status_message: String,
}

impl Default for CharacterCreationState {
    fn default() -> Self {
        Self {
            name: String::new(),
            gender: None,
            name_input_active: false,
            create_requested: false,
            status_message: character_ui::STATUS_DEFAULT_TEXT.to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Gender {
    Male,
    Female,
}

impl Gender {
    fn label(self) -> &'static str {
        match self {
            Self::Male => character_ui::GENDER_MALE_TEXT,
            Self::Female => character_ui::GENDER_FEMALE_TEXT,
        }
    }
}

fn setup_character_select(mut commands: Commands, session: Res<SessionResource>) {
    let world_name = session.world.name.as_str();
    info!("Entering CharacterSelect view for world '{}'", world_name);

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
                width: Val::Percent(ui_style::ROOT_WIDTH_PERCENT),
                height: Val::Percent(ui_style::ROOT_HEIGHT_PERCENT),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(character_ui::ROOT_ROW_GAP),
                ..default()
            },
            BackgroundColor(ui_style::color(palette::BACKGROUND_LIGHT)),
            CharacterSelectUi,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(format!("{}{}", character_ui::SCREEN_TITLE_PREFIX, world_name)),
                TextFont {
                    font_size: character_ui::TITLE_FONT_SIZE,
                    ..default()
                },
                TextColor(ui_style::color(palette::TEXT_PRIMARY)),
                Node {
                    margin: UiRect::bottom(Val::Px(character_ui::TITLE_MARGIN_BOTTOM)),
                    ..default()
                },
            ));

            if characters.is_empty() {
                // Show character creation UI inline
                parent
                    .spawn((Node {
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(character_ui::FORM_ROW_GAP),
                        align_items: AlignItems::Center,
                        ..default()
                    },))
                    .with_children(|form_parent| {
                        form_parent.spawn((
                            Text::new(character_ui::CREATE_TITLE_TEXT),
                            TextFont {
                                font_size: character_ui::CREATE_TITLE_FONT_SIZE,
                                ..default()
                            },
                            TextColor(ui_style::color(palette::TEXT_SECONDARY)),
                            Node {
                                margin: UiRect::bottom(Val::Px(character_ui::CREATE_TITLE_MARGIN_BOTTOM)),
                                ..default()
                            },
                        ));

                        form_parent.spawn((
                            Text::new(character_ui::NAME_LABEL_TEXT),
                            TextFont {
                                font_size: character_ui::SECTION_LABEL_FONT_SIZE,
                                ..default()
                            },
                            TextColor(ui_style::color(palette::TEXT_TERTIARY)),
                        ));

                        form_parent
                            .spawn((
                                Button,
                                Node {
                                    width: Val::Px(character_ui::NAME_INPUT_WIDTH),
                                    height: Val::Px(character_ui::NAME_INPUT_HEIGHT),
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::FlexStart,
                                    padding: UiRect::horizontal(Val::Px(character_ui::NAME_INPUT_PADDING_X)),
                                    border: UiRect::all(Val::Px(character_ui::NAME_INPUT_BORDER_WIDTH)),
                                    ..default()
                                },
                                BorderColor::all(ui_style::color(palette::BORDER_DEFAULT)),
                                BackgroundColor(ui_style::color(palette::SURFACE_DEFAULT)),
                                CharacterNameInputButton,
                            ))
                            .with_children(|input| {
                                input.spawn((
                                    Text::new(character_ui::NAME_PLACEHOLDER_INACTIVE),
                                    TextFont {
                                        font_size: character_ui::NAME_INPUT_FONT_SIZE,
                                        ..default()
                                    },
                                    TextColor(ui_style::color(palette::TEXT_MUTED)),
                                    CharacterNameInputText,
                                ));
                            });

                        form_parent.spawn((
                            Text::new(character_ui::GENDER_LABEL_TEXT),
                            TextFont {
                                font_size: character_ui::SECTION_LABEL_FONT_SIZE,
                                ..default()
                            },
                            TextColor(ui_style::color(palette::TEXT_TERTIARY)),
                            Node {
                                margin: UiRect::top(Val::Px(character_ui::GENDER_LABEL_MARGIN_TOP)),
                                ..default()
                            },
                        ));

                        form_parent
                            .spawn((Node {
                                flex_direction: FlexDirection::Row,
                                column_gap: Val::Px(character_ui::GENDER_ROW_GAP),
                                ..default()
                            },))
                            .with_children(|buttons| {
                                for gender in [Gender::Male, Gender::Female] {
                                    buttons
                                        .spawn((
                                            Button,
                                            Node {
                                                width: Val::Px(character_ui::GENDER_BUTTON_WIDTH),
                                                height: Val::Px(character_ui::GENDER_BUTTON_HEIGHT),
                                                align_items: AlignItems::Center,
                                                justify_content: JustifyContent::Center,
                                                ..default()
                                            },
                                            BackgroundColor(ui_style::color(palette::BUTTON_GENDER_DEFAULT)),
                                            CharacterGenderButton { gender },
                                        ))
                                        .with_children(|button| {
                                            button.spawn((
                                                Text::new(gender.label()),
                                                TextFont {
                                                    font_size: character_ui::GENDER_BUTTON_FONT_SIZE,
                                                    ..default()
                                                },
                                                TextColor(ui_style::color(palette::TEXT_PRIMARY)),
                                            ));
                                        });
                                }
                            });

                        form_parent
                            .spawn((
                                Button,
                                Node {
                                    width: Val::Px(character_ui::CREATE_BUTTON_WIDTH),
                                    height: Val::Px(character_ui::CREATE_BUTTON_HEIGHT),
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    margin: UiRect::top(Val::Px(character_ui::CREATE_BUTTON_MARGIN_TOP)),
                                    ..default()
                                },
                                BackgroundColor(ui_style::color(palette::BUTTON_PRIMARY)),
                                CreateCharacterButton,
                            ))
                            .with_children(|button| {
                                button.spawn((
                                    Text::new(character_ui::CREATE_BUTTON_TEXT),
                                    TextFont {
                                        font_size: character_ui::CREATE_BUTTON_FONT_SIZE,
                                        ..default()
                                    },
                                    TextColor(ui_style::color(palette::TEXT_INVERSE)),
                                ));
                            });

                        form_parent.spawn((
                            Text::new(character_ui::CREATE_HELP_TEXT),
                            TextFont {
                                font_size: character_ui::FORM_STATUS_FONT_SIZE,
                                ..default()
                            },
                            TextColor(ui_style::color(palette::TEXT_MUTED)),
                            TextLayout {
                                justify: Justify::Center,
                                ..default()
                            },
                            Node {
                                margin: UiRect::top(Val::Px(character_ui::FORM_STATUS_MARGIN_TOP)),
                                ..default()
                            },
                            CharacterFormStatusText,
                        ));

                        // Backend note
                        form_parent.spawn((
                            Text::new(character_ui::BACKEND_NOTE_TEXT),
                            TextFont {
                                font_size: character_ui::BACKEND_NOTE_FONT_SIZE,
                                ..default()
                            },
                            TextColor(ui_style::color(palette::TEXT_WARNING)),
                            TextLayout {
                                justify: Justify::Center,
                                ..default()
                            },
                            Node {
                                margin: UiRect::top(Val::Px(character_ui::BACKEND_NOTE_MARGIN_TOP)),
                                ..default()
                            },
                        ));
                    });
            } else {
                // Show character list inline
                parent
                    .spawn((Node {
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(character_ui::LIST_ROW_GAP),
                        align_items: AlignItems::Center,
                        ..default()
                    },))
                    .with_children(|list_parent| {
                        for character in characters {
                            list_parent
                                .spawn((
                                    Button,
                                    Node {
                                        width: Val::Px(character_ui::LIST_BUTTON_WIDTH),
                                        height: Val::Px(character_ui::LIST_BUTTON_HEIGHT),
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::Center,
                                        ..default()
                                    },
                                    BackgroundColor(ui_style::color(palette::BUTTON_CHARACTER_DEFAULT)),
                                    CharacterListItem {
                                        character_id: character.character_id,
                                        name: character.name.clone(),
                                    },
                                ))
                                .with_children(|button| {
                                    button.spawn((
                                        Text::new(format!(
                                            "{} (Level {} {})",
                                            character.name, character.level, character.vocation
                                        )),
                                        TextFont {
                                            font_size: character_ui::LIST_ITEM_FONT_SIZE,
                                            ..default()
                                        },
                                        TextColor(ui_style::color(palette::TEXT_PRIMARY)),
                                    ));
                                });
                        }
                    });

                parent.spawn((
                    Text::new(character_ui::LIST_HINT_TEXT),
                    TextFont {
                        font_size: character_ui::LIST_HINT_FONT_SIZE,
                        ..default()
                    },
                    TextColor(ui_style::color(palette::TEXT_HINT)),
                    Node {
                        margin: UiRect::top(Val::Px(character_ui::LIST_HINT_MARGIN_TOP)),
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

#[allow(clippy::type_complexity)]
fn handle_character_selected(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &CharacterListItem, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, character, mut color) in interaction_query.iter_mut() {
        *color = ui_helpers::interaction_background(*interaction, CHARACTER_BUTTON);

        if *interaction != Interaction::Pressed {
            continue;
        }

        info!("Selected character: {} ({})", character.name, character.character_id);

        commands.insert_resource(SelectedCharacterResource {
            character_id: character.character_id,
            name: character.name.clone(),
        });

        next_state.set(AppState::Game);
    }
}

#[allow(clippy::type_complexity)]
fn handle_character_creation_interactions(
    mut creation_state: ResMut<CharacterCreationState>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut keyboard_input_events: MessageReader<KeyboardInput>,
    name_input_query: Query<&Interaction, (Changed<Interaction>, With<Button>, With<CharacterNameInputButton>)>,
    gender_button_query: Query<
        (&Interaction, &CharacterGenderButton),
        (
            Changed<Interaction>,
            With<Button>,
            Without<CreateCharacterButton>,
            Without<CharacterNameInputButton>,
        ),
    >,
    create_button_query: Query<&Interaction, (Changed<Interaction>, With<Button>, With<CreateCharacterButton>)>,
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

    for interaction in name_input_query.iter() {
        if *interaction == Interaction::Pressed {
            creation_state.name_input_active = true;
        }
    }

    for (interaction, button) in gender_button_query.iter() {
        if *interaction == Interaction::Pressed {
            creation_state.gender = Some(button.gender);
            creation_state.name_input_active = false;
        }
    }

    for interaction in create_button_query.iter() {
        if *interaction == Interaction::Pressed {
            creation_state.create_requested = true;
            creation_state.name_input_active = false;
        }
    }

    if creation_state.name_input_active && keyboard.just_pressed(KeyCode::Backspace) {
        creation_state.name.pop();
    }

    for event in keyboard_input_events.read() {
        if !creation_state.name_input_active || event.state != ButtonState::Pressed {
            continue;
        }

        if let Some(text) = event.text.as_ref() {
            let typed_text: String = text.chars().filter(|character| !character.is_control()).collect();
            if !typed_text.is_empty() {
                creation_state.name.push_str(&typed_text);
            }
        }
    }

    if keyboard.just_pressed(KeyCode::Enter) {
        creation_state.create_requested = true;
    }

    if !creation_state.create_requested {
        return;
    }

    creation_state.create_requested = false;
    let display_name = creation_state.name.trim().to_string();

    if display_name.is_empty() {
        creation_state.status_message = character_ui::STATUS_NAME_REQUIRED_TEXT.to_string();
        info!("Character name is empty");
        return;
    }

    if creation_state.gender.is_none() {
        creation_state.status_message = character_ui::STATUS_GENDER_REQUIRED_TEXT.to_string();
        info!("Gender not selected");
        return;
    }

    let gender = creation_state.gender.expect("checked is_some above");
    info!(
        "Attempting to create character: name='{}', gender='{}'",
        display_name,
        gender.label()
    );

    // TODO: Call backend reducer when available
    // For now, we'll just log a message explaining what would happen
    warn!("Backend reducer for character creation is not yet implemented");
    warn!(
        "Would create character with name='{}', gender='{}'",
        display_name,
        gender.label()
    );
    warn!("Please implement a 'create_character' reducer in the ikariadb module");

    creation_state.status_message = character_ui::STATUS_BACKEND_PENDING_TEXT.to_string();
}

#[allow(clippy::type_complexity)]
fn sync_character_creation_visuals(
    creation_state: Res<CharacterCreationState>,
    session: Res<SessionResource>,
    mut name_input_text_query: Query<&mut Text, (With<CharacterNameInputText>, Without<CharacterFormStatusText>)>,
    mut status_text_query: Query<&mut Text, (With<CharacterFormStatusText>, Without<CharacterNameInputText>)>,
    mut name_input_button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (
            With<Button>,
            With<CharacterNameInputButton>,
            Without<CreateCharacterButton>,
            Without<CharacterGenderButton>,
        ),
    >,
    mut gender_button_query: Query<
        (&Interaction, &CharacterGenderButton, &mut BackgroundColor),
        (
            With<Button>,
            Without<CharacterNameInputButton>,
            Without<CreateCharacterButton>,
        ),
    >,
    mut create_button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (
            With<Button>,
            With<CreateCharacterButton>,
            Without<CharacterNameInputButton>,
            Without<CharacterGenderButton>,
        ),
    >,
) {
    let has_characters = session
        .connection
        .db
        .character_v_1()
        .iter()
        .any(|c| c.user_id == session.identity);

    if has_characters {
        return;
    }

    for mut text in name_input_text_query.iter_mut() {
        text.0 = if creation_state.name.is_empty() {
            if creation_state.name_input_active {
                character_ui::NAME_PLACEHOLDER_ACTIVE.to_string()
            } else {
                character_ui::NAME_PLACEHOLDER_INACTIVE.to_string()
            }
        } else {
            creation_state.name.clone()
        };
    }

    for mut text in status_text_query.iter_mut() {
        text.0 = creation_state.status_message.clone();
    }

    for (interaction, mut color) in name_input_button_query.iter_mut() {
        *color = if creation_state.name_input_active {
            ui_helpers::interaction_background(*interaction, NAME_INPUT_ACTIVE)
        } else {
            ui_helpers::interaction_background(*interaction, NAME_INPUT_INACTIVE)
        };
    }

    for (interaction, button, mut color) in gender_button_query.iter_mut() {
        *color = if Some(button.gender) == creation_state.gender {
            ui_helpers::interaction_background(*interaction, GENDER_SELECTED_BUTTON)
        } else {
            ui_helpers::interaction_background(*interaction, GENDER_BUTTON)
        };
    }

    for (interaction, mut color) in create_button_query.iter_mut() {
        *color = ui_helpers::interaction_background(*interaction, PRIMARY_BUTTON);
    }
}

fn cleanup_character_select(mut commands: Commands, query: Query<Entity, With<CharacterSelectUi>>) {
    info!("Exiting CharacterSelect view");
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
    commands.remove_resource::<CharacterCreationState>();
}
