use crate::{
    constants::{DEFAULT_SPAWN_X, DEFAULT_SPAWN_Y, INITIAL_MAP_EDGE},
    repository::{
        character::services::CharacterReducerContext,
        world::{
            CharacterPositionV1, MapV1, character_position_v1, map_v1, math,
            types::{DirectionV1, MapTileV1},
        },
    },
};
use ikaria_shared::constants::GROUND_LEVEL;
use spacetimedb::{Identity, ReducerContext, Table};
use std::ops::Deref;

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
    pub fn initial_spawn_character(&self, user_id: Identity) {
        let Ok(character) = self.character_services().get_current(user_id) else {
            return;
        };

        let existing = self.db.character_position_v1().character_id().find(character.character_id);
        if existing.is_some() {
            return;
        }

        let (x, y, z) = self.default_spawn_position();
        let map_id = math::into_map_id(x, y, z);
        if self.db.map_v1().map_id().find(map_id).is_none() {
            return;
        }

        let position = CharacterPositionV1 {
            character_id: character.character_id,
            x,
            y,
            z,
            direction: DirectionV1::South,
            updated_at: self.timestamp,
        };
        self.db.character_position_v1().insert(position);
    }

    pub fn seed_initial_map(&self) {
        let existing_count = self.db.map_v1().count();
        if existing_count > 0 {
            // Already seeded, do not attempt to insert duplicates
            return;
        }

        for x in 0..INITIAL_MAP_EDGE {
            for y in 0..INITIAL_MAP_EDGE {
                let z = GROUND_LEVEL;
                let tile = MapV1 {
                    map_id: math::into_map_id(x, y, z),
                    x,
                    y,
                    z,
                    tile: MapTileV1::Grass,
                };
                self.db.map_v1().insert(tile);
            }
        }
    }

    fn default_spawn_position(&self) -> (u16, u16, u16) {
        (DEFAULT_SPAWN_X, DEFAULT_SPAWN_Y, GROUND_LEVEL)
    }
}
