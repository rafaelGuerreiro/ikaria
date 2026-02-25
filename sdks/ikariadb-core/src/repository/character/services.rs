use crate::{
    constants::{
        CHARACTER_NAME_MAX_LEN, CHARACTER_NAME_MIN_LEN, DEFAULT_CHARACTER_CAPACITY, DEFAULT_CHARACTER_EXPERIENCE,
        DEFAULT_CHARACTER_HEALTH, DEFAULT_CHARACTER_LEVEL, DEFAULT_CHARACTER_MANA,
    },
    error::{ErrorMapper, ResultExt, ServiceError, ServiceResult},
    extend::validate::ReducerContextRequirements,
    repository::{
        character::{
            CharacterStatsV1, CharacterV1, CurrentCharacterV1, character_stats_v1, character_v1, current_character_v1,
            types::{ClassV1, GenderV1, RaceV1},
        },
        event::services::EventReducerContext,
    },
};
use spacetimedb::{Identity, ReducerContext, Table};
use std::ops::Deref;
use thiserror::Error;

pub trait CharacterReducerContext {
    fn character_services(&self) -> CharacterServices<'_>;
}

impl CharacterReducerContext for ReducerContext {
    fn character_services(&self) -> CharacterServices<'_> {
        CharacterServices { ctx: self }
    }
}

pub struct CharacterServices<'a> {
    ctx: &'a ReducerContext,
}

impl Deref for CharacterServices<'_> {
    type Target = ReducerContext;

    fn deref(&self) -> &Self::Target {
        self.ctx
    }
}

impl CharacterServices<'_> {
    pub fn get(&self, character_id: u64) -> ServiceResult<CharacterV1> {
        let character = self
            .db
            .character_v1()
            .character_id()
            .find(character_id)
            .ok_or_else(|| CharacterError::character_not_found(character_id))?;

        Ok(character)
    }

    pub fn get_current(&self, user_id: Identity) -> ServiceResult<CharacterV1> {
        let current = self
            .db
            .current_character_v1()
            .user_id()
            .find(user_id)
            .ok_or_else(|| CharacterError::character_not_selected(user_id))?;

        let character = self.get(current.character_id)?;
        if character.user_id != user_id {
            return Err(CharacterError::character_ownership_mismatch(current.character_id, user_id));
        }

        Ok(character)
    }

    pub fn create_character(
        &self,
        user_id: Identity,
        display_name: String,
        gender: GenderV1,
        race: RaceV1,
    ) -> ServiceResult<()> {
        let (display_name, canonical_name) = self.prepare_character_names(display_name)?;

        let character = self.db.character_v1().try_insert(CharacterV1 {
            character_id: 0,
            user_id,
            name: canonical_name,
            display_name: display_name.clone(),
            class: ClassV1::None,
            race,
            gender,
            created_at: self.timestamp,
        });

        let character = match character {
            Ok(character) => character,
            Err(_err) => {
                return Err(CharacterError::name_taken(display_name));
            },
        };

        self.db
            .character_stats_v1()
            .try_insert(CharacterStatsV1::new(&character))
            .map_conflict()?;

        self.publish().character_created(user_id, character.character_id)?;
        self.select_character(user_id, character.character_id)?;

        Ok(())
    }

    pub fn select_character(&self, user_id: Identity, character_id: u64) -> ServiceResult<()> {
        let character = self.get(character_id)?;
        if character.user_id != user_id {
            return Err(CharacterError::character_ownership_mismatch(character_id, user_id));
        }

        self.db.current_character_v1().user_id().insert_or_update(CurrentCharacterV1 {
            user_id,
            character_id,
            signed_in_at: self.timestamp,
        });

        self.publish().character_selected(user_id, character_id)?;
        Ok(())
    }

    pub fn clear_current_character(&self, user_id: Identity) {
        self.db.current_character_v1().user_id().delete(user_id);
    }

    fn prepare_character_names(&self, display_name: String) -> ServiceResult<(String, String)> {
        self.validate_str(
            &display_name,
            "display_name",
            CHARACTER_NAME_MIN_LEN as u64,
            CHARACTER_NAME_MAX_LEN as u64,
        )?;

        let mut display_name = display_name.trim().to_string();
        while display_name.contains("  ") {
            display_name = display_name.replace("  ", " ");
        }

        if display_name
            .chars()
            .any(|character| !character.is_ascii_alphabetic() && character != ' ')
        {
            return Err(CharacterError::name_invalid_characters());
        }

        let canonical_name: String = display_name
            .chars()
            .filter(|character| !character.is_whitespace())
            .map(|character| character.to_ascii_lowercase())
            .collect();

        if canonical_name.is_empty() {
            return Err(CharacterError::name_without_letters());
        }

        self.validate_str(
            &canonical_name,
            "canonical_name",
            CHARACTER_NAME_MIN_LEN as u64,
            CHARACTER_NAME_MAX_LEN as u64,
        )?;

        Ok((display_name, canonical_name))
    }
}

impl CharacterStatsV1 {
    pub fn new(character: &CharacterV1) -> Self {
        Self {
            character_id: character.character_id,
            user_id: character.user_id,
            level: DEFAULT_CHARACTER_LEVEL,
            experience: DEFAULT_CHARACTER_EXPERIENCE,
            health: DEFAULT_CHARACTER_HEALTH,
            mana: DEFAULT_CHARACTER_MANA,
            capacity: DEFAULT_CHARACTER_CAPACITY,
        }
    }
}

#[derive(Debug, Error)]
enum CharacterError {
    #[error("No current character selected for user {0}")]
    CharacterNotSelected(Identity),

    #[error("Character {0} was not found")]
    CharacterNotFound(u64),

    #[error("Character {character_id} does not belong to user {user_id}")]
    CharacterOwnershipMismatch { character_id: u64, user_id: Identity },

    #[error("Character name '{0}' is already taken")]
    NameTaken(String),

    #[error("Character name must contain only letters and spaces")]
    NameInvalidCharacters,

    #[error("Character name must contain at least one letter")]
    NameWithoutLetters,
}

impl CharacterError {
    fn character_not_selected(user_id: Identity) -> ServiceError {
        Self::CharacterNotSelected(user_id).map_forbidden_error()
    }

    fn character_not_found(character_id: u64) -> ServiceError {
        Self::CharacterNotFound(character_id).map_not_found_error()
    }

    fn character_ownership_mismatch(character_id: u64, user_id: Identity) -> ServiceError {
        Self::CharacterOwnershipMismatch { character_id, user_id }.map_forbidden_error()
    }

    fn name_taken(display_name: String) -> ServiceError {
        Self::NameTaken(display_name).map_conflict_error()
    }

    fn name_invalid_characters() -> ServiceError {
        Self::NameInvalidCharacters.map_validation_error()
    }

    fn name_without_letters() -> ServiceError {
        Self::NameWithoutLetters.map_validation_error()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use spacetimedb::ReducerContext;

    #[test]
    fn prepare_character_names_returns_display_and_canonical() {
        let dummy = ReducerContext::__dummy();
        let services = CharacterServices { ctx: &dummy };

        assert_eq!(
            services.prepare_character_names("  Sir     Galahad  ".to_string()).ok(),
            Some(("Sir Galahad".to_string(), "sirgalahad".to_string()))
        );
    }

    #[test]
    fn prepare_character_names_rejects_non_letter_characters() {
        let dummy = ReducerContext::__dummy();
        let services = CharacterServices { ctx: &dummy };

        assert!(services.prepare_character_names("Name123".to_string()).is_err());
    }

    #[test]
    fn prepare_character_names_rejects_small_after_trimmed() {
        let dummy = ReducerContext::__dummy();
        let services = CharacterServices { ctx: &dummy };

        assert!(services.prepare_character_names(" ab ".to_string()).is_err());
        assert!(services.prepare_character_names("a b".to_string()).is_err());
    }

    #[test]
    fn prepare_character_names_rejects_space_only_input() {
        let dummy = ReducerContext::__dummy();
        let services = CharacterServices { ctx: &dummy };

        assert!(services.prepare_character_names("   ".to_string()).is_err());
    }
}
