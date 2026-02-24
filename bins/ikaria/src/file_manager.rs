use crate::{
    constants::{APP_DATA_DIR_NAME, TOKEN_FILE_NAME},
    error::{ClientError, ClientResult, ResultExt},
};
use std::{env, fs, path::PathBuf};

pub fn game_data_dir() -> ClientResult<PathBuf> {
    let base_dir =
        platform_data_dir().ok_or_else(|| ClientError::internal("Unable to resolve a platform data directory for Ikaria"))?;

    let dir = base_dir.join(APP_DATA_DIR_NAME);
    fs::create_dir_all(&dir).map_internal_error()?;
    Ok(dir)
}

pub fn token_file_path() -> ClientResult<PathBuf> {
    Ok(game_data_dir()?.join(TOKEN_FILE_NAME))
}

#[cfg(target_os = "windows")]
fn platform_data_dir() -> Option<PathBuf> {
    env::var_os(crate::constants::ENV_APPDATA).map(PathBuf::from)
}

#[cfg(target_os = "macos")]
fn platform_data_dir() -> Option<PathBuf> {
    env::var_os(crate::constants::ENV_HOME).map(PathBuf::from).map(|home| {
        home.join(crate::constants::MACOS_LIBRARY_DIR)
            .join(crate::constants::MACOS_APPLICATION_SUPPORT_DIR)
    })
}

#[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
fn platform_data_dir() -> Option<PathBuf> {
    env::var_os(crate::constants::ENV_XDG_DATA_HOME)
        .map(PathBuf::from)
        .or_else(|| {
            env::var_os(crate::constants::ENV_HOME).map(PathBuf::from).map(|home| {
                home.join(crate::constants::UNIX_LOCAL_DIR)
                    .join(crate::constants::UNIX_SHARE_DIR)
            })
        })
}
