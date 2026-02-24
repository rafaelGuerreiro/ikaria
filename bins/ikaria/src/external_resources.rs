use crate::{
    constants::{APP_DATA_DIR_NAME, TOKEN_FILE_NAME},
    error::{ClientError, ClientResult, ResultExt},
};
use std::{env, fs, io::ErrorKind, path::PathBuf};

const PROJECT_ROOT_MARKER_FILE: &str = "Cargo.toml";

pub fn game_data_dir() -> ClientResult<PathBuf> {
    let base_dir =
        platform_data_dir().ok_or_else(|| ClientError::internal("Unable to resolve a platform data directory for Ikaria"))?;

    let dir = base_dir.join(APP_DATA_DIR_NAME);
    fs::create_dir_all(&dir).map_internal_error()?;
    Ok(dir)
}

pub fn read_or_create_project_file(relative_path: &str, default_content: &str) -> ClientResult<String> {
    let file_path = project_file_path(relative_path)?;

    match fs::read_to_string(&file_path) {
        Ok(content) => Ok(content),
        Err(err) if err.kind() == ErrorKind::NotFound => {
            if let Some(parent) = file_path.parent() {
                fs::create_dir_all(parent).map_internal_error()?;
            }

            fs::write(&file_path, default_content).map_internal_error()?;
            Ok(default_content.to_owned())
        },
        Err(err) => Err(err).map_internal_error(),
    }
}

pub fn read_saved_token() -> ClientResult<Option<String>> {
    let token_path = token_file_path()?;
    let token_content = match fs::read_to_string(token_path) {
        Ok(content) => content,
        Err(err) if err.kind() == ErrorKind::NotFound => return Ok(None),
        Err(err) => return Err(err).map_internal_error(),
    };

    let token = token_content.trim().to_string();
    if token.is_empty() { Ok(None) } else { Ok(Some(token)) }
}

pub fn save_token(token: &str) -> ClientResult<()> {
    fs::write(token_file_path()?, token).map_internal_error()
}

fn token_file_path() -> ClientResult<PathBuf> {
    Ok(game_data_dir()?.join(TOKEN_FILE_NAME))
}

fn project_file_path(relative_path: &str) -> ClientResult<PathBuf> {
    let exe_path = env::current_exe().map_internal_error()?;
    let exe_dir = exe_path
        .parent()
        .ok_or_else(|| ClientError::internal("Unable to resolve the executable directory"))?;

    let project_dir = exe_dir
        .ancestors()
        .find(|ancestor| ancestor.join(PROJECT_ROOT_MARKER_FILE).exists())
        .unwrap_or(exe_dir);

    Ok(project_dir.join(relative_path))
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
