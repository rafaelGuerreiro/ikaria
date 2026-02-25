use super::character_select_screen::{
    CharacterCreationForm, CharacterFormStatusText, CharacterGenderButton, CharacterListItem, CharacterListSection,
    CharacterNameInputButton, CharacterNameInputText, CharacterRaceButton, CharacterSelectUi, CreateCharacterButton,
    EmptyCharacterListPrompt, Gender, Race, ShowCharacterCreationButton,
};
use crate::ui_style::{self, character_select as character_ui, palette};
use bevy::prelude::*;
use ikaria_types::autogen::CharacterV1;

pub(super) fn spawn_character_select_ui(commands: &mut Commands, world_name: &str, characters: &[CharacterV1]) {
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
                parent
                    .spawn((
                        Node {
                            flex_direction: FlexDirection::Row,
                            column_gap: Val::Px(character_ui::EMPTY_STATE_ROW_GAP),
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        EmptyCharacterListPrompt,
                    ))
                    .with_children(|prompt_parent| {
                        prompt_parent.spawn((
                            Text::new(character_ui::EMPTY_STATE_TEXT),
                            TextFont {
                                font_size: character_ui::EMPTY_STATE_TEXT_FONT_SIZE,
                                ..default()
                            },
                            TextColor(ui_style::color(palette::TEXT_SECONDARY)),
                        ));

                        prompt_parent
                            .spawn((
                                Button,
                                Node {
                                    width: Val::Px(character_ui::EMPTY_STATE_BUTTON_WIDTH),
                                    height: Val::Px(character_ui::EMPTY_STATE_BUTTON_HEIGHT),
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    ..default()
                                },
                                BackgroundColor(ui_style::color(palette::BUTTON_PRIMARY)),
                                ShowCharacterCreationButton,
                            ))
                            .with_children(|button| {
                                button.spawn((
                                    Text::new(character_ui::EMPTY_STATE_BUTTON_TEXT),
                                    TextFont {
                                        font_size: character_ui::EMPTY_STATE_BUTTON_FONT_SIZE,
                                        ..default()
                                    },
                                    TextColor(ui_style::color(palette::TEXT_INVERSE)),
                                ));
                            });
                    });
            } else {
                parent
                    .spawn((
                        Node {
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(character_ui::LIST_ROW_GAP),
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        CharacterListSection,
                    ))
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
                                            "{} ({:?} {:?})",
                                            character.display_name, character.race, character.class
                                        )),
                                        TextFont {
                                            font_size: character_ui::LIST_ITEM_FONT_SIZE,
                                            ..default()
                                        },
                                        TextColor(ui_style::color(palette::TEXT_PRIMARY)),
                                    ));
                                });
                        }

                        list_parent.spawn((
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

                        list_parent
                            .spawn((
                                Button,
                                Node {
                                    width: Val::Px(character_ui::LIST_CREATE_BUTTON_WIDTH),
                                    height: Val::Px(character_ui::LIST_CREATE_BUTTON_HEIGHT),
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    margin: UiRect::top(Val::Px(character_ui::LIST_CREATE_BUTTON_MARGIN_TOP)),
                                    ..default()
                                },
                                BackgroundColor(ui_style::color(palette::BUTTON_PRIMARY)),
                                ShowCharacterCreationButton,
                            ))
                            .with_children(|button| {
                                button.spawn((
                                    Text::new(character_ui::LIST_CREATE_BUTTON_TEXT),
                                    TextFont {
                                        font_size: character_ui::LIST_CREATE_BUTTON_FONT_SIZE,
                                        ..default()
                                    },
                                    TextColor(ui_style::color(palette::TEXT_INVERSE)),
                                ));
                            });
                    });
            }

            // Character creation form — always spawned, hidden by default
            spawn_character_creation_form(parent);
        });
}

fn spawn_character_creation_form(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(character_ui::FORM_ROW_GAP),
                align_items: AlignItems::Center,
                display: Display::None,
                ..default()
            },
            CharacterCreationForm,
        ))
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
                        Text::new(character_ui::NAME_PLACEHOLDER_ACTIVE),
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

            form_parent.spawn((
                Text::new(character_ui::RACE_LABEL_TEXT),
                TextFont {
                    font_size: character_ui::SECTION_LABEL_FONT_SIZE,
                    ..default()
                },
                TextColor(ui_style::color(palette::TEXT_TERTIARY)),
                Node {
                    margin: UiRect::top(Val::Px(character_ui::RACE_LABEL_MARGIN_TOP)),
                    ..default()
                },
            ));

            form_parent
                .spawn((Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(character_ui::RACE_ROW_GAP),
                    ..default()
                },))
                .with_children(|buttons| {
                    for race in [Race::Human, Race::Elf] {
                        buttons
                            .spawn((
                                Button,
                                Node {
                                    width: Val::Px(character_ui::RACE_BUTTON_WIDTH),
                                    height: Val::Px(character_ui::RACE_BUTTON_HEIGHT),
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    ..default()
                                },
                                BackgroundColor(ui_style::color(palette::BUTTON_RACE_DEFAULT)),
                                CharacterRaceButton { race },
                            ))
                            .with_children(|button| {
                                button.spawn((
                                    Text::new(race.label()),
                                    TextFont {
                                        font_size: character_ui::RACE_BUTTON_FONT_SIZE,
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
        });
}
