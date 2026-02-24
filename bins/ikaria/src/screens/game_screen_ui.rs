use super::game_screen::GameUi;
use crate::ui_style::{self, game as game_ui, palette};
use bevy::prelude::*;

pub(super) fn spawn_game_ui(commands: &mut Commands, character_name: &str) {
    commands
        .spawn((
            Node {
                width: Val::Percent(ui_style::ROOT_WIDTH_PERCENT),
                height: Val::Percent(ui_style::ROOT_HEIGHT_PERCENT),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(ui_style::color(palette::BACKGROUND_GAME)),
            GameUi,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(format!(
                    "{}{}{}",
                    game_ui::WELCOME_PREFIX,
                    character_name,
                    game_ui::WELCOME_SUFFIX
                )),
                TextFont {
                    font_size: game_ui::WELCOME_FONT_SIZE,
                    ..default()
                },
                TextColor(ui_style::color(palette::TEXT_INVERSE)),
                Node {
                    margin: UiRect::bottom(Val::Px(game_ui::WELCOME_MARGIN_BOTTOM)),
                    ..default()
                },
            ));

            parent.spawn((
                Text::new(game_ui::INFO_TEXT),
                TextFont {
                    font_size: game_ui::INFO_FONT_SIZE,
                    ..default()
                },
                TextColor(ui_style::color(palette::TEXT_SOFT_INVERSE)),
                TextLayout {
                    justify: Justify::Center,
                    ..default()
                },
            ));
        });
}
