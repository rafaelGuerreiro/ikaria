use crate::{
    constants::WORLDS_CONFIG_PATH,
    error::{ClientError, ClientResult, ResultExt},
    external_resources,
};
use serde::Deserialize;

const EMBEDDED_WORLDS_CONFIG: &str = include_str!("../config/worlds.json");

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
    let config_content = load_worlds_config_content()?;
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

fn load_worlds_config_content() -> ClientResult<String> {
    external_resources::read_or_create_project_file(WORLDS_CONFIG_PATH, EMBEDDED_WORLDS_CONFIG)
}
