use crate::{
    constants::WORLDS_CONFIG_PATH,
    error::{ClientError, ClientResult, ResultExt},
};
use serde::Deserialize;
use std::fs;

#[derive(Debug, Clone, Deserialize)]
pub struct WorldDefinition {
    pub name: String,
    pub module_name: String,
}

#[derive(Debug, Deserialize)]
struct WorldsConfig {
    worlds: Vec<WorldDefinition>,
}

pub fn load_worlds() -> ClientResult<Vec<WorldDefinition>> {
    let config_content = fs::read_to_string(WORLDS_CONFIG_PATH).map_internal_error()?;
    let config: WorldsConfig = serde_json::from_str(&config_content).map_internal_error()?;

    if config.worlds.is_empty() {
        return Err(ClientError::user("World config must define at least one world"));
    }

    for world in &config.worlds {
        if world.name.trim().is_empty() {
            return Err(ClientError::user("World config has a world with an empty name"));
        }

        if world.module_name.trim().is_empty() {
            return Err(ClientError::user(format!(
                "World '{}' has an empty module_name in world config",
                world.name
            )));
        }
    }

    Ok(config.worlds)
}
