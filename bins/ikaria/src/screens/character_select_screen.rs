use crate::{
    app_state::AppState,
    resources::{SelectedCharacterResource, SessionResource},
    ui_helpers::{
        self, CHARACTER_BUTTON, GENDER_BUTTON, GENDER_SELECTED_BUTTON, NAME_INPUT_ACTIVE, NAME_INPUT_INACTIVE, PRIMARY_BUTTON,
        RACE_BUTTON, RACE_SELECTED_BUTTON,
    },
    ui_style::character_select as character_ui,
};
use bevy::{
    input::{ButtonState, keyboard::KeyboardInput},
    prelude::*,
};
use ikaria_types::autogen::{
    CharacterV1, GenderV1, RaceV1, VwCharacterAllMineV1TableAccess, create_character_v_1, select_character_v_1,
};
use spacetimedb_sdk::{DbContext, Table};

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
        app.add_systems(
            Update,
            rebuild_on_character_change.run_if(in_state(AppState::CharacterSelect)),
        );
        app.add_systems(OnExit(AppState::CharacterSelect), cleanup_character_select);
    }
}

/// Marker component for character select UI
#[derive(Component)]
pub(super) struct CharacterSelectUi;

/// Component marking character list items
#[derive(Component)]
pub(super) struct CharacterListItem {
    pub(super) character_id: u64,
    pub(super) name: String,
}

/// Component for character name input button
#[derive(Component)]
pub(super) struct CharacterNameInputButton;

/// Component for character name input text
#[derive(Component)]
pub(super) struct CharacterNameInputText;

/// Component for character gender selection
#[derive(Component)]
pub(super) struct CharacterGenderButton {
    pub(super) gender: Gender,
}

/// Component for character race selection
#[derive(Component)]
pub(super) struct CharacterRaceButton {
    pub(super) race: Race,
}

#[derive(Component)]
pub(super) struct CreateCharacterButton;

#[derive(Component)]
pub(super) struct CharacterFormStatusText;

#[derive(Component)]
pub(super) struct EmptyCharacterListPrompt;

#[derive(Component)]
pub(super) struct ShowCharacterCreationButton;

#[derive(Component)]
pub(super) struct CharacterCreationForm;

#[derive(Component)]
pub(super) struct CharacterListSection;

/// Character creation input state
#[derive(Resource)]
struct CharacterCreationState {
    name: String,
    gender: Option<Gender>,
    race: Option<Race>,
    name_input_active: bool,
    show_creation_form: bool,
    create_requested: bool,
    status_message: String,
    character_count: usize,
}

impl Default for CharacterCreationState {
    fn default() -> Self {
        Self {
            name: String::new(),
            gender: None,
            race: None,
            name_input_active: true,
            show_creation_form: false,
            create_requested: false,
            status_message: character_ui::STATUS_DEFAULT_TEXT.to_string(),
            character_count: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Gender {
    Male,
    Female,
}

impl Gender {
    pub(super) fn label(self) -> &'static str {
        match self {
            Self::Male => character_ui::GENDER_MALE_TEXT,
            Self::Female => character_ui::GENDER_FEMALE_TEXT,
        }
    }

    fn to_autogen(self) -> GenderV1 {
        match self {
            Self::Male => GenderV1::Male,
            Self::Female => GenderV1::Female,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Race {
    Human,
    Elf,
}

impl Race {
    pub(super) fn label(self) -> &'static str {
        match self {
            Self::Human => character_ui::RACE_HUMAN_TEXT,
            Self::Elf => character_ui::RACE_ELF_TEXT,
        }
    }

    fn to_autogen(self) -> RaceV1 {
        match self {
            Self::Human => RaceV1::Human,
            Self::Elf => RaceV1::Elf,
        }
    }
}

fn setup_character_select(mut commands: Commands, session: Res<SessionResource>) {
    let world_name = session.world.name.as_str();
    info!("Entering CharacterSelect view for world '{}'", world_name);

    // Get characters for this user
    let characters: Vec<CharacterV1> = session
        .connection
        .db
        .vw_character_all_mine_v_1()
        .iter()
        .filter(|c| c.user_id == session.identity)
        .collect();

    info!("Found {} characters for user", characters.len());

    // Initialize character creation state with current count
    let creation_state = CharacterCreationState {
        character_count: characters.len(),
        ..Default::default()
    };
    commands.insert_resource(creation_state);

    super::character_select_screen_ui::spawn_character_select_ui(&mut commands, world_name, &characters);
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
    session: Res<SessionResource>,
) {
    for (interaction, character, mut color) in interaction_query.iter_mut() {
        *color = ui_helpers::interaction_background(*interaction, CHARACTER_BUTTON);

        if *interaction != Interaction::Pressed {
            continue;
        }

        info!("Selected character: {} ({})", character.name, character.character_id);

        if let Err(e) = session.connection.reducers().select_character_v_1(character.character_id) {
            warn!("Failed to call select_character reducer: {}", e);
        }

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
    button_query: Query<
        (
            &Interaction,
            Has<ShowCharacterCreationButton>,
            Has<CharacterNameInputButton>,
            Option<&CharacterGenderButton>,
            Option<&CharacterRaceButton>,
            Has<CreateCharacterButton>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    session: Res<SessionResource>,
) {
    for (interaction, is_show_form_button, is_name_input_button, gender_button, race_button, is_create_button) in
        button_query.iter()
    {
        if *interaction != Interaction::Pressed {
            continue;
        }

        if is_show_form_button {
            creation_state.show_creation_form = true;
            creation_state.name_input_active = true;
            continue;
        }

        if !creation_state.show_creation_form {
            continue;
        }

        if is_name_input_button {
            creation_state.name_input_active = true;
            continue;
        }

        if let Some(button) = gender_button {
            creation_state.gender = Some(button.gender);
            continue;
        }

        if let Some(button) = race_button {
            creation_state.race = Some(button.race);
            continue;
        }

        if is_create_button {
            creation_state.create_requested = true;
        }
    }

    if !creation_state.show_creation_form {
        return;
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

    if creation_state.race.is_none() {
        creation_state.status_message = character_ui::STATUS_RACE_REQUIRED_TEXT.to_string();
        info!("Race not selected");
        return;
    }

    let gender = creation_state.gender.expect("checked is_some above");
    let race = creation_state.race.expect("checked is_some above");
    info!(
        "Creating character: name='{}', gender='{}', race='{}'",
        display_name,
        gender.label(),
        race.label()
    );

    match session
        .connection
        .reducers()
        .create_character_v_1(display_name, gender.to_autogen(), race.to_autogen())
    {
        Ok(()) => {
            creation_state.status_message = character_ui::STATUS_CREATING_TEXT.to_string();
        },
        Err(e) => {
            warn!("Failed to call create_character reducer: {}", e);
            creation_state.status_message = format!("Error: {}", e);
        },
    }
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
fn sync_character_creation_visuals(
    creation_state: Res<CharacterCreationState>,
    mut creation_view_query: Query<
        (
            &mut Node,
            Has<EmptyCharacterListPrompt>,
            Has<CharacterCreationForm>,
            Has<CharacterListSection>,
        ),
        (
            Or<(
                With<EmptyCharacterListPrompt>,
                With<CharacterCreationForm>,
                With<CharacterListSection>,
            )>,
        ),
    >,
    mut primary_button_query: Query<
        (
            &Interaction,
            Has<ShowCharacterCreationButton>,
            Has<CreateCharacterButton>,
            &mut BackgroundColor,
        ),
        (
            With<Button>,
            Or<(With<ShowCharacterCreationButton>, With<CreateCharacterButton>)>,
            Without<CharacterNameInputButton>,
            Without<CharacterGenderButton>,
            Without<CharacterRaceButton>,
        ),
    >,
    mut form_text_query: Query<
        (&mut Text, Has<CharacterNameInputText>, Has<CharacterFormStatusText>),
        Or<(With<CharacterNameInputText>, With<CharacterFormStatusText>)>,
    >,
    mut name_input_button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (
            With<Button>,
            With<CharacterNameInputButton>,
            Without<CreateCharacterButton>,
            Without<CharacterGenderButton>,
            Without<CharacterRaceButton>,
        ),
    >,
    mut gender_button_query: Query<
        (&Interaction, &CharacterGenderButton, &mut BackgroundColor),
        (
            With<Button>,
            Without<CharacterNameInputButton>,
            Without<CreateCharacterButton>,
            Without<CharacterRaceButton>,
        ),
    >,
    mut race_button_query: Query<
        (&Interaction, &CharacterRaceButton, &mut BackgroundColor),
        (
            With<Button>,
            Without<CharacterNameInputButton>,
            Without<CreateCharacterButton>,
            Without<CharacterGenderButton>,
        ),
    >,
) {
    for (mut node, is_empty_prompt, is_creation_form, is_list_section) in creation_view_query.iter_mut() {
        if is_empty_prompt || is_list_section {
            node.display = if creation_state.show_creation_form {
                Display::None
            } else {
                Display::Flex
            };
        } else if is_creation_form {
            node.display = if creation_state.show_creation_form {
                Display::Flex
            } else {
                Display::None
            };
        }
    }

    for (interaction, _is_show_form_button, _is_create_button, mut color) in primary_button_query.iter_mut() {
        *color = ui_helpers::interaction_background(*interaction, PRIMARY_BUTTON);
    }

    for (mut text, is_name_input_text, is_status_text) in form_text_query.iter_mut() {
        if is_name_input_text {
            text.0 = if creation_state.name.is_empty() {
                if creation_state.name_input_active {
                    character_ui::NAME_PLACEHOLDER_ACTIVE.to_string()
                } else {
                    character_ui::NAME_PLACEHOLDER_INACTIVE.to_string()
                }
            } else {
                creation_state.name.clone()
            };
        } else if is_status_text {
            text.0 = creation_state.status_message.clone();
        }
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

    for (interaction, button, mut color) in race_button_query.iter_mut() {
        *color = if Some(button.race) == creation_state.race {
            ui_helpers::interaction_background(*interaction, RACE_SELECTED_BUTTON)
        } else {
            ui_helpers::interaction_background(*interaction, RACE_BUTTON)
        };
    }
}

/// Rebuild UI when new characters appear from subscription updates
fn rebuild_on_character_change(
    mut commands: Commands,
    session: Res<SessionResource>,
    mut creation_state: ResMut<CharacterCreationState>,
    ui_query: Query<Entity, With<CharacterSelectUi>>,
) {
    let characters: Vec<CharacterV1> = session
        .connection
        .db
        .vw_character_all_mine_v_1()
        .iter()
        .filter(|c| c.user_id == session.identity)
        .collect();

    if characters.len() == creation_state.character_count {
        return;
    }

    info!(
        "Character count changed ({} -> {}), rebuilding UI",
        creation_state.character_count,
        characters.len()
    );
    creation_state.character_count = characters.len();
    creation_state.show_creation_form = false;
    creation_state.name.clear();
    creation_state.gender = None;
    creation_state.race = None;
    creation_state.status_message = character_ui::STATUS_DEFAULT_TEXT.to_string();

    // Despawn old UI
    for entity in ui_query.iter() {
        commands.entity(entity).despawn();
    }

    // Rebuild with updated character list
    let world_name = session.world.name.as_str();
    super::character_select_screen_ui::spawn_character_select_ui(&mut commands, world_name, &characters);
}

fn cleanup_character_select(mut commands: Commands, query: Query<Entity, With<CharacterSelectUi>>) {
    info!("Exiting CharacterSelect view");
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
    commands.remove_resource::<CharacterCreationState>();
}
