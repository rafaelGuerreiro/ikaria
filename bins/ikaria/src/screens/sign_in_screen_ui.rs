use super::sign_in_screen::{SignInButton, SignInHintText, SignInUi, WorldSelectionText, WorldSelectorButton};
use crate::ui_style::{self, palette, sign_in as sign_in_ui};
use bevy::prelude::*;

pub(super) fn spawn_sign_in_ui(commands: &mut Commands, initial_world_label: String, initial_hint: String) {
    commands
        .spawn((
            Node {
                width: Val::Percent(ui_style::ROOT_WIDTH_PERCENT),
                height: Val::Percent(ui_style::ROOT_HEIGHT_PERCENT),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(sign_in_ui::ROOT_ROW_GAP),
                ..default()
            },
            BackgroundColor(ui_style::color(palette::BACKGROUND_LIGHT)),
            SignInUi,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(sign_in_ui::TITLE_TEXT),
                TextFont {
                    font_size: sign_in_ui::TITLE_FONT_SIZE,
                    ..default()
                },
                TextColor(ui_style::color(palette::TEXT_PRIMARY)),
                Node {
                    margin: UiRect::bottom(Val::Px(sign_in_ui::TITLE_MARGIN_BOTTOM)),
                    ..default()
                },
            ));

            parent.spawn((
                Text::new(sign_in_ui::WORLD_LABEL_TEXT),
                TextFont {
                    font_size: sign_in_ui::LABEL_FONT_SIZE,
                    ..default()
                },
                TextColor(ui_style::color(palette::TEXT_SECONDARY)),
            ));

            parent
                .spawn((Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    column_gap: Val::Px(sign_in_ui::SELECTOR_COLUMN_GAP),
                    ..default()
                },))
                .with_children(|selector| {
                    selector
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(sign_in_ui::SELECTOR_BUTTON_SIZE),
                                height: Val::Px(sign_in_ui::SELECTOR_BUTTON_SIZE),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            BackgroundColor(ui_style::color(palette::BUTTON_SELECTOR)),
                            WorldSelectorButton { step: -1 },
                        ))
                        .with_children(|button| {
                            button.spawn((
                                Text::new(sign_in_ui::SELECTOR_LEFT_TEXT),
                                TextFont {
                                    font_size: sign_in_ui::SELECTOR_ARROW_FONT_SIZE,
                                    ..default()
                                },
                                TextColor(ui_style::color(palette::TEXT_INVERSE)),
                            ));
                        });

                    selector
                        .spawn((
                            Node {
                                width: Val::Px(sign_in_ui::SELECTOR_VALUE_WIDTH),
                                height: Val::Px(sign_in_ui::SELECTOR_VALUE_HEIGHT),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                border: UiRect::all(Val::Px(sign_in_ui::SELECTOR_BORDER_WIDTH)),
                                ..default()
                            },
                            BorderColor::all(ui_style::color(palette::BORDER_DEFAULT)),
                            BackgroundColor(ui_style::color(palette::SURFACE_DEFAULT)),
                        ))
                        .with_children(|value| {
                            value.spawn((
                                Text::new(initial_world_label.clone()),
                                TextFont {
                                    font_size: sign_in_ui::SELECTOR_VALUE_FONT_SIZE,
                                    ..default()
                                },
                                TextColor(ui_style::color(palette::TEXT_PRIMARY)),
                                WorldSelectionText,
                            ));
                        });

                    selector
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(sign_in_ui::SELECTOR_BUTTON_SIZE),
                                height: Val::Px(sign_in_ui::SELECTOR_BUTTON_SIZE),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            BackgroundColor(ui_style::color(palette::BUTTON_SELECTOR)),
                            WorldSelectorButton { step: 1 },
                        ))
                        .with_children(|button| {
                            button.spawn((
                                Text::new(sign_in_ui::SELECTOR_RIGHT_TEXT),
                                TextFont {
                                    font_size: sign_in_ui::SELECTOR_ARROW_FONT_SIZE,
                                    ..default()
                                },
                                TextColor(ui_style::color(palette::TEXT_INVERSE)),
                            ));
                        });
                });

            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(sign_in_ui::SIGN_IN_BUTTON_WIDTH),
                        height: Val::Px(sign_in_ui::SIGN_IN_BUTTON_HEIGHT),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    BackgroundColor(ui_style::color(palette::BUTTON_PRIMARY)),
                    SignInButton,
                ))
                .with_children(|button| {
                    button.spawn((
                        Text::new(sign_in_ui::SIGN_IN_BUTTON_TEXT),
                        TextFont {
                            font_size: sign_in_ui::SIGN_IN_BUTTON_FONT_SIZE,
                            ..default()
                        },
                        TextColor(ui_style::color(palette::TEXT_INVERSE)),
                    ));
                });

            parent.spawn((
                Text::new(initial_hint),
                TextFont {
                    font_size: sign_in_ui::HINT_FONT_SIZE,
                    ..default()
                },
                TextColor(ui_style::color(palette::TEXT_SECONDARY)),
                TextLayout {
                    justify: Justify::Center,
                    ..default()
                },
                Node {
                    margin: UiRect::top(Val::Px(sign_in_ui::HINT_MARGIN_TOP)),
                    ..default()
                },
                SignInHintText,
            ));
        });
}
