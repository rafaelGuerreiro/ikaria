use crate::ui_helpers::Rgb;
use bevy::prelude::Color;

pub const ROOT_WIDTH_PERCENT: f32 = 100.0;
pub const ROOT_HEIGHT_PERCENT: f32 = 100.0;

pub mod palette {
    use crate::ui_helpers::Rgb;

    pub const BACKGROUND_LIGHT: Rgb = (0.95, 0.95, 0.95);
    pub const BACKGROUND_GAME: Rgb = (0.2, 0.5, 0.2);
    pub const SURFACE_DEFAULT: Rgb = (1.0, 1.0, 1.0);
    pub const SURFACE_MUTED: Rgb = (0.96, 0.96, 0.96);

    pub const TEXT_PRIMARY: Rgb = (0.2, 0.2, 0.2);
    pub const TEXT_SECONDARY: Rgb = (0.35, 0.35, 0.35);
    pub const TEXT_TERTIARY: Rgb = (0.4, 0.4, 0.4);
    pub const TEXT_MUTED: Rgb = (0.45, 0.45, 0.45);
    pub const TEXT_HINT: Rgb = (0.5, 0.5, 0.5);
    pub const TEXT_INVERSE: Rgb = (0.95, 0.95, 0.95);
    pub const TEXT_SOFT_INVERSE: Rgb = (0.9, 0.9, 0.9);
    pub const TEXT_WARNING: Rgb = (0.8, 0.3, 0.3);

    pub const BORDER_DEFAULT: Rgb = (0.35, 0.35, 0.35);

    pub const BUTTON_PRIMARY: Rgb = (0.2, 0.45, 0.85);
    pub const BUTTON_PRIMARY_HOVER: Rgb = (0.25, 0.5, 0.9);
    pub const BUTTON_PRIMARY_PRESSED: Rgb = (0.15, 0.35, 0.7);

    pub const BUTTON_SELECTOR: Rgb = (0.25, 0.45, 0.8);
    pub const BUTTON_SELECTOR_HOVER: Rgb = (0.3, 0.52, 0.9);
    pub const BUTTON_SELECTOR_PRESSED: Rgb = (0.2, 0.35, 0.65);

    pub const BUTTON_DANGER: Rgb = (0.45, 0.28, 0.28);
    pub const BUTTON_DANGER_HOVER: Rgb = (0.5, 0.3, 0.3);
    pub const BUTTON_DANGER_PRESSED: Rgb = (0.6, 0.25, 0.25);

    pub const INPUT_FOCUS: Rgb = (0.8, 0.88, 0.98);
    pub const INPUT_HOVER: Rgb = (0.9, 0.95, 1.0);

    pub const BUTTON_GENDER_DEFAULT: Rgb = (0.75, 0.75, 0.75);
    pub const BUTTON_GENDER_HOVER: Rgb = (0.82, 0.82, 0.82);
    pub const BUTTON_GENDER_SELECTED: Rgb = (0.46, 0.72, 0.46);
    pub const BUTTON_GENDER_SELECTED_HOVER: Rgb = (0.5, 0.78, 0.5);
    pub const BUTTON_GENDER_PRESSED: Rgb = (0.36, 0.62, 0.36);

    pub const BUTTON_RACE_DEFAULT: Rgb = (0.75, 0.75, 0.75);
    pub const BUTTON_RACE_HOVER: Rgb = (0.82, 0.82, 0.82);
    pub const BUTTON_RACE_SELECTED: Rgb = (0.46, 0.72, 0.46);
    pub const BUTTON_RACE_SELECTED_HOVER: Rgb = (0.5, 0.78, 0.5);
    pub const BUTTON_RACE_PRESSED: Rgb = (0.36, 0.62, 0.36);

    pub const BUTTON_CHARACTER_DEFAULT: Rgb = (0.84, 0.84, 0.84);
    pub const BUTTON_CHARACTER_HOVER: Rgb = (0.72, 0.82, 0.92);
    pub const BUTTON_CHARACTER_PRESSED: Rgb = (0.5, 0.7, 0.9);
}

pub mod sign_in {
    pub const ROOT_ROW_GAP: f32 = 16.0;
    pub const TITLE_FONT_SIZE: f32 = 32.0;
    pub const TITLE_MARGIN_BOTTOM: f32 = 14.0;
    pub const LABEL_FONT_SIZE: f32 = 18.0;

    pub const SELECTOR_COLUMN_GAP: f32 = 8.0;
    pub const SELECTOR_BUTTON_SIZE: f32 = 44.0;
    pub const SELECTOR_ARROW_FONT_SIZE: f32 = 22.0;
    pub const SELECTOR_VALUE_WIDTH: f32 = 260.0;
    pub const SELECTOR_VALUE_HEIGHT: f32 = 44.0;
    pub const SELECTOR_VALUE_FONT_SIZE: f32 = 20.0;
    pub const SELECTOR_BORDER_WIDTH: f32 = 2.0;

    pub const SIGN_IN_BUTTON_WIDTH: f32 = 220.0;
    pub const SIGN_IN_BUTTON_HEIGHT: f32 = 60.0;
    pub const SIGN_IN_BUTTON_FONT_SIZE: f32 = 28.0;

    pub const HINT_FONT_SIZE: f32 = 16.0;
    pub const HINT_MARGIN_TOP: f32 = 6.0;

    pub const TITLE_TEXT: &str = "Welcome to Ikaria";
    pub const WORLD_LABEL_TEXT: &str = "World";
    pub const SELECTOR_LEFT_TEXT: &str = "◀";
    pub const SELECTOR_RIGHT_TEXT: &str = "▶";
    pub const SIGN_IN_BUTTON_TEXT: &str = "Sign In";
    pub const WORLD_UNAVAILABLE_TEXT: &str = "Unavailable";
    pub const HINT_SELECT_WORLD_TEXT: &str = "Select a world and click Sign In to authenticate";
    pub const HINT_NO_WORLDS_TEXT: &str = "No worlds available in world configuration";
    pub const HINT_CONFIG_ERROR_PREFIX: &str = "World configuration error: ";
    pub const HINT_SELECTED_WORLD_PREFIX: &str = "Selected world: ";
}

pub mod character_select {
    pub const ROOT_ROW_GAP: f32 = 16.0;
    pub const TITLE_FONT_SIZE: f32 = 40.0;
    pub const TITLE_MARGIN_BOTTOM: f32 = 24.0;

    pub const FORM_ROW_GAP: f32 = 14.0;
    pub const CREATE_TITLE_FONT_SIZE: f32 = 28.0;
    pub const CREATE_TITLE_MARGIN_BOTTOM: f32 = 8.0;
    pub const SECTION_LABEL_FONT_SIZE: f32 = 18.0;

    pub const NAME_INPUT_WIDTH: f32 = 280.0;
    pub const NAME_INPUT_HEIGHT: f32 = 42.0;
    pub const NAME_INPUT_PADDING_X: f32 = 12.0;
    pub const NAME_INPUT_BORDER_WIDTH: f32 = 2.0;
    pub const NAME_INPUT_FONT_SIZE: f32 = 16.0;

    pub const GENDER_LABEL_MARGIN_TOP: f32 = 8.0;
    pub const GENDER_ROW_GAP: f32 = 14.0;
    pub const GENDER_BUTTON_WIDTH: f32 = 120.0;
    pub const GENDER_BUTTON_HEIGHT: f32 = 40.0;
    pub const GENDER_BUTTON_FONT_SIZE: f32 = 16.0;

    pub const CREATE_BUTTON_WIDTH: f32 = 220.0;
    pub const CREATE_BUTTON_HEIGHT: f32 = 46.0;
    pub const CREATE_BUTTON_MARGIN_TOP: f32 = 6.0;
    pub const CREATE_BUTTON_FONT_SIZE: f32 = 18.0;

    pub const FORM_STATUS_FONT_SIZE: f32 = 13.0;
    pub const FORM_STATUS_MARGIN_TOP: f32 = 8.0;

    pub const LIST_ROW_GAP: f32 = 10.0;
    pub const LIST_BUTTON_WIDTH: f32 = 380.0;
    pub const LIST_BUTTON_HEIGHT: f32 = 48.0;
    pub const LIST_ITEM_FONT_SIZE: f32 = 22.0;
    pub const LIST_HINT_FONT_SIZE: f32 = 16.0;
    pub const LIST_HINT_MARGIN_TOP: f32 = 14.0;
    pub const EMPTY_STATE_ROW_GAP: f32 = 12.0;
    pub const EMPTY_STATE_TEXT_FONT_SIZE: f32 = 20.0;
    pub const EMPTY_STATE_BUTTON_WIDTH: f32 = 240.0;
    pub const EMPTY_STATE_BUTTON_HEIGHT: f32 = 42.0;
    pub const EMPTY_STATE_BUTTON_FONT_SIZE: f32 = 16.0;

    pub const SCREEN_TITLE_PREFIX: &str = "Character Selection - ";
    pub const CREATE_TITLE_TEXT: &str = "Create Your Character";
    pub const NAME_LABEL_TEXT: &str = "Character Name";
    pub const NAME_PLACEHOLDER_INACTIVE: &str = "(Type name)";
    pub const NAME_PLACEHOLDER_ACTIVE: &str = "(Type name)";
    pub const GENDER_LABEL_TEXT: &str = "Gender";
    pub const GENDER_MALE_TEXT: &str = "Male";
    pub const GENDER_FEMALE_TEXT: &str = "Female";
    pub const RACE_LABEL_TEXT: &str = "Race";
    pub const RACE_LABEL_MARGIN_TOP: f32 = 8.0;
    pub const RACE_ROW_GAP: f32 = 14.0;
    pub const RACE_BUTTON_WIDTH: f32 = 120.0;
    pub const RACE_BUTTON_HEIGHT: f32 = 40.0;
    pub const RACE_BUTTON_FONT_SIZE: f32 = 16.0;
    pub const RACE_HUMAN_TEXT: &str = "Human";
    pub const RACE_ELF_TEXT: &str = "Elf";
    pub const CREATE_BUTTON_TEXT: &str = "Create Character";
    pub const CREATE_HELP_TEXT: &str = "Type a name, choose gender and race, then click Create Character";
    pub const LIST_HINT_TEXT: &str = "Click a character to enter the game";
    pub const LIST_CREATE_BUTTON_TEXT: &str = "Create New Character";
    pub const LIST_CREATE_BUTTON_WIDTH: f32 = 220.0;
    pub const LIST_CREATE_BUTTON_HEIGHT: f32 = 44.0;
    pub const LIST_CREATE_BUTTON_FONT_SIZE: f32 = 16.0;
    pub const LIST_CREATE_BUTTON_MARGIN_TOP: f32 = 8.0;
    pub const EMPTY_STATE_TEXT: &str = "You have no characters on this world.";
    pub const EMPTY_STATE_BUTTON_TEXT: &str = "Click here to create one";
    pub const STATUS_DEFAULT_TEXT: &str = "Type a name, choose gender and race, then click Create Character";
    pub const STATUS_NAME_REQUIRED_TEXT: &str = "Character name is required";
    pub const STATUS_GENDER_REQUIRED_TEXT: &str = "Please choose a gender";
    pub const STATUS_RACE_REQUIRED_TEXT: &str = "Please choose a race";
    pub const STATUS_CREATING_TEXT: &str = "Creating character...";
}

pub mod game {
    pub const WELCOME_FONT_SIZE: f32 = 40.0;
    pub const WELCOME_MARGIN_BOTTOM: f32 = 20.0;
    pub const INFO_FONT_SIZE: f32 = 20.0;

    pub const INFO_TEXT: &str = "Game world placeholder\n\nPress ESC to return to character select";
    pub const WELCOME_PREFIX: &str = "Welcome, ";
    pub const WELCOME_SUFFIX: &str = "!";
}

pub fn color((r, g, b): Rgb) -> Color {
    Color::srgb(r, g, b)
}
