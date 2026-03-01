use crate::{
    constants::{CHAT_MESSAGE_MAX_LEN, CHAT_MESSAGE_MIN_LEN},
    error::{ErrorMapper, ServiceError, ServiceResult},
    extend::validate::ReducerContextRequirements,
    repository::{
        character::{character_stats_v1, character_v1},
        chat::{ChatBubbleV1, chat_bubble_v1},
        world::online_character_position_v1,
    },
};
use spacetimedb::{ReducerContext, Table};
use std::ops::Deref;
use thiserror::Error;

pub trait ChatReducerContext {
    fn chat_services(&self) -> ChatServices<'_>;
}

impl ChatReducerContext for ReducerContext {
    fn chat_services(&self) -> ChatServices<'_> {
        ChatServices { ctx: self }
    }
}

pub struct ChatServices<'a> {
    ctx: &'a ReducerContext,
}

impl Deref for ChatServices<'_> {
    type Target = ReducerContext;

    fn deref(&self) -> &Self::Target {
        self.ctx
    }
}

impl ChatServices<'_> {
    pub fn send_message(&self, character_id: u64, content: String) -> ServiceResult<()> {
        let content = content.trim().to_string();
        if content.is_empty() {
            return Err(ChatError::message_empty());
        }
        self.validate_str(&content, "message", CHAT_MESSAGE_MIN_LEN as u64, CHAT_MESSAGE_MAX_LEN as u64)?;

        let position = self
            .db
            .online_character_position_v1()
            .character_id()
            .find(character_id)
            .ok_or_else(ChatError::position_not_found)?;

        let character = self
            .db
            .character_v1()
            .character_id()
            .find(character_id)
            .ok_or_else(ChatError::character_not_found)?;

        let stats = self
            .db
            .character_stats_v1()
            .character_id()
            .find(character_id)
            .ok_or_else(ChatError::stats_not_found)?;

        self.db.chat_bubble_v1().insert(ChatBubbleV1 {
            bubble_id: 0,
            character_name: character.display_name,
            character_level: stats.level,
            content,
            x: position.x,
            y: position.y,
            sent_at: self.timestamp,
        });

        Ok(())
    }
}

#[derive(Debug, Error)]
enum ChatError {
    #[error("Chat message cannot be empty")]
    MessageEmpty,
    #[error("Character position not found")]
    PositionNotFound,
    #[error("Character not found")]
    CharacterNotFound,
    #[error("Character stats not found")]
    StatsNotFound,
}

impl ChatError {
    fn message_empty() -> ServiceError {
        Self::MessageEmpty.map_validation_error()
    }

    fn position_not_found() -> ServiceError {
        Self::PositionNotFound.map_validation_error()
    }

    fn character_not_found() -> ServiceError {
        Self::CharacterNotFound.map_validation_error()
    }

    fn stats_not_found() -> ServiceError {
        Self::StatsNotFound.map_validation_error()
    }
}
