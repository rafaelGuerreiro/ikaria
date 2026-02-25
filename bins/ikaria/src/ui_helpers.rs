use crate::ui_style::{self, palette};
use bevy::prelude::{BackgroundColor, Interaction};

pub type Rgb = (f32, f32, f32);

#[derive(Clone, Copy)]
pub struct InteractionPalette {
    pub idle: Rgb,
    pub hover: Rgb,
    pub pressed: Rgb,
}

impl InteractionPalette {
    pub const fn new(idle: Rgb, hover: Rgb, pressed: Rgb) -> Self {
        Self { idle, hover, pressed }
    }
}

pub const PRIMARY_BUTTON: InteractionPalette = InteractionPalette::new(
    palette::BUTTON_PRIMARY,
    palette::BUTTON_PRIMARY_HOVER,
    palette::BUTTON_PRIMARY_PRESSED,
);

pub const DANGER_BUTTON: InteractionPalette = InteractionPalette::new(
    palette::BUTTON_DANGER,
    palette::BUTTON_DANGER_HOVER,
    palette::BUTTON_DANGER_PRESSED,
);

pub const SELECTOR_BUTTON: InteractionPalette = InteractionPalette::new(
    palette::BUTTON_SELECTOR,
    palette::BUTTON_SELECTOR_HOVER,
    palette::BUTTON_SELECTOR_PRESSED,
);

pub const CHARACTER_BUTTON: InteractionPalette = InteractionPalette::new(
    palette::BUTTON_CHARACTER_DEFAULT,
    palette::BUTTON_CHARACTER_HOVER,
    palette::BUTTON_CHARACTER_PRESSED,
);

pub const GENDER_BUTTON: InteractionPalette = InteractionPalette::new(
    palette::BUTTON_GENDER_DEFAULT,
    palette::BUTTON_GENDER_HOVER,
    palette::BUTTON_GENDER_PRESSED,
);

pub const GENDER_SELECTED_BUTTON: InteractionPalette = InteractionPalette::new(
    palette::BUTTON_GENDER_SELECTED,
    palette::BUTTON_GENDER_SELECTED_HOVER,
    palette::BUTTON_GENDER_PRESSED,
);

pub const RACE_BUTTON: InteractionPalette = InteractionPalette::new(
    palette::BUTTON_RACE_DEFAULT,
    palette::BUTTON_RACE_HOVER,
    palette::BUTTON_RACE_PRESSED,
);

pub const RACE_SELECTED_BUTTON: InteractionPalette = InteractionPalette::new(
    palette::BUTTON_RACE_SELECTED,
    palette::BUTTON_RACE_SELECTED_HOVER,
    palette::BUTTON_RACE_PRESSED,
);

pub const NAME_INPUT_ACTIVE: InteractionPalette =
    InteractionPalette::new(palette::SURFACE_DEFAULT, palette::INPUT_HOVER, palette::INPUT_FOCUS);

pub const NAME_INPUT_INACTIVE: InteractionPalette =
    InteractionPalette::new(palette::SURFACE_MUTED, palette::INPUT_HOVER, palette::INPUT_FOCUS);

pub fn interaction_background(interaction: Interaction, palette: InteractionPalette) -> BackgroundColor {
    let color = match interaction {
        Interaction::Pressed => palette.pressed,
        Interaction::Hovered => palette.hover,
        Interaction::None => palette.idle,
    };

    BackgroundColor(ui_style::color(color))
}
