// TODO Check this file and logic for the whole
// TODO dirs for config thing its a mess

use std::{
    fs::OpenOptions,
    io::Write,
    path::{Path, PathBuf},
};

use directories::{ProjectDirs, UserDirs};
use serde::Deserialize;

use crate::{errors::ServerError, Result};

#[derive(Clone, Debug, Deserialize)]
pub struct Directories {
    pub config_root_folder: PathBuf,
    pub config_file: PathBuf,
    pub commands_file: PathBuf,
    pub paired_devices_file: PathBuf,
    pub upload_folder: PathBuf,
}

// * We can pass root folder to use for testing
pub fn directories(root: Option<&PathBuf>) -> Result<Directories> {
    let config_root_folder = if let Some(root_dir) = root {
        PathBuf::from(root_dir).join("rusko")
    } else {
        ProjectDirs::from("com", "aalhitennf", "rusko")
            .ok_or(ServerError::ConfigError {
                message: "Can't get project directories".into(),
            })?
            .config_dir()
            .to_path_buf()
    };

    let upload_folder = if let Some(dl_root) = root {
        PathBuf::from(dl_root).join("rusko_dl")
    } else {
        let user_dirs = UserDirs::new().ok_or(ServerError::ConfigError {
            message: "Can't get user directories".into(),
        })?;

        user_dirs
            .download_dir()
            .unwrap_or_else(|| user_dirs.home_dir())
            .to_path_buf()
    };

    let dirs = Directories {
        config_file: config_root_folder.join("config.toml"),
        commands_file: config_root_folder.join("commands"),
        paired_devices_file: config_root_folder.join("paired_devices"),
        config_root_folder,
        upload_folder,
    };

    // Make sure files and folders exists
    std::fs::create_dir_all(&dirs.config_root_folder)?;
    create_default_config(&dirs.config_file)?;
    create_commands_file(&dirs.commands_file)?;
    create_paired_devices_file(&dirs.paired_devices_file)?;

    Ok(dirs)
}

fn create_default_config(path: &Path) -> Result<()> {
    if path.exists() {
        return Ok(());
    }

    let mut file = OpenOptions::new().create_new(true).write(true).open(path)?;

    let default_config = if cfg!(test) {
        "port = \"6551\"\npassword = \"PlsChangeThisNow\""
    } else {
        "port = \"6551\"\npassword = \"\""
    };

    file.write_all(default_config.as_bytes())
        .map_err(Into::into)
}

fn create_commands_file(path: &Path) -> Result<()> {
    if path.exists() {
        return Ok(());
    }

    let mut file = OpenOptions::new().create_new(true).write(true).open(path)?;

    #[cfg(target_os = "linux")]
    let default_commands = if cfg!(test) {
        "Example :: true"
    } else {
        "Volume up :: pactl set-sink-volume @DEFAULT_SINK@ +5%\nVolume down :: pactl set-sink-volume @DEFAULT_SINK@ -5%\nToggle mute :: pactl set-sink-mute @DEFAULT_SINK@ toggle"
    };

    #[cfg(target_os = "windows")]
    let default_commands = if cfg!(test) {
        "Example :: dir"
    } else {
        // ? Don't know what to put in here
        "Example :: dir"
    };

    file.write_all(default_commands.as_bytes())
        .map_err(Into::into)
}

fn create_paired_devices_file(path: &Path) -> Result<()> {
    if path.exists() {
        return Ok(());
    }

    let mut file = OpenOptions::new().create_new(true).write(true).open(path)?;

    let content = if cfg!(test) { "testid" } else { "" };

    file.write_all(content.as_bytes()).map_err(Into::into)
}
