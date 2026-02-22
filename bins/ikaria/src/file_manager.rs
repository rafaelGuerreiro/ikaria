use crate::constants::{APP_DATA_DIR_NAME, TOKEN_FILE_NAME};
use std::{env, fs, io, path::PathBuf};

pub fn game_data_dir() -> io::Result<PathBuf> {
    let base_dir = platform_data_dir().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::NotFound,
            "Unable to resolve a platform data directory for Ikaria",
        )
    })?;

    let dir = base_dir.join(APP_DATA_DIR_NAME);
    fs::create_dir_all(&dir)?;
    Ok(dir)
}

pub fn token_file_path() -> io::Result<PathBuf> {
    Ok(game_data_dir()?.join(TOKEN_FILE_NAME))
}

#[cfg(target_os = "windows")]
fn platform_data_dir() -> Option<PathBuf> {
    env::var_os("APPDATA").map(PathBuf::from)
}

#[cfg(target_os = "macos")]
fn platform_data_dir() -> Option<PathBuf> {
    env::var_os("HOME")
        .map(PathBuf::from)
        .map(|home| home.join("Library").join("Application Support"))
}

#[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
fn platform_data_dir() -> Option<PathBuf> {
    env::var_os("XDG_DATA_HOME").map(PathBuf::from).or_else(|| {
        env::var_os("HOME")
            .map(PathBuf::from)
            .map(|home| home.join(".local").join("share"))
    })
}
