use crate::{
    constants::{DEFAULT_CHARACTER_SPEED, DEFAULT_SPAWN_X, DEFAULT_SPAWN_Y, MOVEMENT_COOLDOWN_FACTOR},
    error::{ErrorMapper, ServiceError, ServiceResult},
    repository::{
        character::{character_v1, services::CharacterReducerContext},
        world::{
            CharacterPositionV1, MapV1, MovementCooldownV1, map_v1,
            math::into_map_id,
            movement_cooldown_v1, offline_character_position_v1, online_character_position_v1,
            types::{DirectionV1, MapTileV1, MovementV1},
        },
    },
};
use ikaria_shared::constants::GROUND_LEVEL;
use spacetimedb::{Identity, ReducerContext, Table};
use std::{ops::Deref, time::Duration};
use thiserror::Error;

pub trait WorldReducerContext {
    fn world_services(&self) -> WorldServices<'_>;
}

impl WorldReducerContext for ReducerContext {
    fn world_services(&self) -> WorldServices<'_> {
        WorldServices { ctx: self }
    }
}

pub struct WorldServices<'a> {
    ctx: &'a ReducerContext,
}

impl Deref for WorldServices<'_> {
    type Target = ReducerContext;

    fn deref(&self) -> &Self::Target {
        self.ctx
    }
}

impl WorldServices<'_> {
    pub fn find_online_position(&self, character_id: u64) -> Option<CharacterPositionV1> {
        self.db.online_character_position_v1().character_id().find(character_id)
    }

    pub fn find_offline_position(&self, character_id: u64) -> Option<CharacterPositionV1> {
        self.db.offline_character_position_v1().character_id().find(character_id)
    }

    pub fn find_cooldown(&self, character_id: u64) -> Option<MovementCooldownV1> {
        self.db.movement_cooldown_v1().character_id().find(character_id)
    }

    pub fn is_movement_allowed(&self, character_id: u64) -> bool {
        if let Some(cooldown) = self.find_cooldown(character_id) {
            self.timestamp >= cooldown.can_move_at
        } else {
            true
        }
    }

    pub fn find_map_tile(&self, x: u16, y: u16, z: u16) -> Option<MapV1> {
        self.db.map_v1().map_id().find(into_map_id(x, y, z))
    }

    pub fn get_online_position(&self, character_id: u64) -> ServiceResult<CharacterPositionV1> {
        self.find_online_position(character_id)
            .ok_or_else(|| WorldError::character_position_not_found(character_id))
    }

    pub fn get_offline_position(&self, character_id: u64) -> ServiceResult<CharacterPositionV1> {
        self.find_offline_position(character_id)
            .ok_or_else(|| WorldError::character_position_not_found(character_id))
    }

    pub fn get_map_tile(&self, x: u16, y: u16, z: u16) -> ServiceResult<MapV1> {
        self.find_map_tile(x, y, z)
            .ok_or_else(|| WorldError::map_tile_not_found(x, y, z))
    }

    pub fn spawn_character(&self, user_id: Identity) {
        // Ensure any existing character is despawned for this user.
        self.despawn_character(user_id);

        let Ok(character) = self.character_services().get_current(user_id) else {
            return;
        };

        let character_id = character.character_id;
        let position = self
            .find_offline_position(character_id)
            .or_else(|| self.find_online_position(character_id))
            .unwrap_or_else(|| {
                let (x, y, z) = (DEFAULT_SPAWN_X, DEFAULT_SPAWN_Y, GROUND_LEVEL);
                CharacterPositionV1 {
                    character_id,
                    map_id: into_map_id(x, y, z),
                    x,
                    y,
                    z,
                    movement: MovementV1::default(),
                    direction: DirectionV1::default(),
                }
            });

        self.db.offline_character_position_v1().character_id().delete(character_id);
        self.db
            .online_character_position_v1()
            .character_id()
            .insert_or_update(position);
    }

    pub fn despawn_character(&self, user_id: Identity) {
        for character in self.db.character_v1().user_id().filter(user_id) {
            let character_id = character.character_id;
            if let Some(position) = self.find_online_position(character_id) {
                self.db
                    .offline_character_position_v1()
                    .character_id()
                    .insert_or_update(position);
            }
            self.db.online_character_position_v1().character_id().delete(character_id);
            self.db.movement_cooldown_v1().character_id().delete(character_id);
        }
    }

    pub fn seed_initial_map(&self) {
        let existing_count = self.db.map_v1().count();
        if existing_count > 0 {
            // Already seeded, do not attempt to insert duplicates
            return;
        }

        let water_margin = 32;
        let grass_start = 1024;
        let grass_end = 2048;
        let edge_start = grass_start - water_margin;
        let edge_end = grass_end + water_margin;

        for x in edge_start..edge_end {
            for y in edge_start..edge_end {
                let tile = if x < grass_start || x > grass_end || y < grass_start || y > grass_end {
                    MapTileV1::Water
                } else {
                    MapTileV1::Grass
                };

                let tile = MapV1 {
                    map_id: into_map_id(x, y, GROUND_LEVEL),
                    x,
                    y,
                    z: GROUND_LEVEL,
                    tile,
                };
                self.db.map_v1().insert(tile);
            }
        }
    }

    pub fn move_character(&self, character_id: u64, movement: MovementV1) -> ServiceResult<()> {
        if !self.is_movement_allowed(character_id) {
            return Ok(());
        }

        let character = self.character_services().get_online(character_id)?;
        let Ok(position) = self.get_online_position(character.character_id) else {
            return Ok(()); // No position, cannot move, but also no need to error
        };

        let (target_x, target_y) = movement.translate(position.x, position.y);
        if target_x == position.x && target_y == position.y {
            return Ok(()); // No movement, stay in place
        }

        let target_tile = self.get_map_tile(target_x, target_y, position.z)?;

        if !target_tile.tile.is_walkable() {
            return Err(ServiceError::BadRequest("Target tile is not walkable".into()));
        }

        self.db
            .online_character_position_v1()
            .character_id()
            .update(CharacterPositionV1 {
                x: target_x,
                y: target_y,
                z: position.z,
                movement,
                direction: movement.into(),
                ..position
            });

        self.set_movement_cooldown(character.character_id);
        Ok(())
    }

    fn set_movement_cooldown(&self, character_id: u64) {
        let speed = self
            .character_services()
            .find_stats(character_id)
            .map(|s| s.speed as u64)
            .unwrap_or(DEFAULT_CHARACTER_SPEED as u64);

        let cooldown_ms = MOVEMENT_COOLDOWN_FACTOR / speed;
        self.db
            .movement_cooldown_v1()
            .character_id()
            .insert_or_update(MovementCooldownV1 {
                character_id,
                can_move_at: self.timestamp + Duration::from_millis(cooldown_ms),
            });
    }
}

#[derive(Debug, Error)]
enum WorldError {
    #[error("Map tile does not exist: ({0}, {1}, {2})")]
    MapTileNotFound(u16, u16, u16),

    #[error("Character {0} has no position")]
    CharacterPositionNotFound(u64),
}

impl WorldError {
    fn map_tile_not_found(x: u16, y: u16, z: u16) -> ServiceError {
        Self::MapTileNotFound(x, y, z).map_not_found_error()
    }

    fn character_position_not_found(character_id: u64) -> ServiceError {
        Self::CharacterPositionNotFound(character_id).map_not_found_error()
    }
}
