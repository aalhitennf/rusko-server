pub mod command;
pub mod monitor;

use std::path::PathBuf;

pub use command::Command;

use serde::Deserialize;

use crate::{
    errors::ServerError,
    utils::{
        aes::{decrypt, encrypt},
        dirs::Directories,
        read, write,
    },
    Result,
};

#[derive(Clone, Debug, Deserialize)]
pub struct Server {
    pub password: String,
    pub port: Option<String>,
    pub upload_folder: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Commands {
    pub vec: Vec<Command>,
    pub str: String,
}

#[derive(Clone)]
pub struct Config {
    pub server: Server,
    pub commands: Commands,
    pub paired_devices: Vec<String>,
    pub dirs: Directories,
}

impl Config {
    pub fn new(mut dirs: Directories) -> Result<Config> {
        // Config values
        let server: Server = std::fs::read_to_string(&dirs.config_file)
            .map_err(|e| e.to_string())
            .and_then(|s| toml::from_str(&s).map_err(|e| e.to_string()))?;

        if server.password.len() != 16 {
            return Err("Password must be exactly 16 characters!".into());
        }

        // Use user defined upload folder if present and writable
        if let Some(path) = &server.upload_folder {
            if std::fs::create_dir_all(&path).is_ok() {
                dirs.upload_folder = PathBuf::from(path);
            }
        }

        let commands = read::commands(&dirs.commands_file)?;
        let paired_devices = read::paired_devices(&dirs.paired_devices_file)?;

        log::info!("Parsed {} commands", commands.vec.len());
        log::info!("Found {} paired devices", paired_devices.len());

        Ok(Config {
            server,
            commands,
            paired_devices,
            dirs,
        })
    }
    pub fn update_commands(&mut self) -> Result<()> {
        self.commands = read::commands(&self.dirs.commands_file)?;
        Ok(())
    }
    pub fn commands_to_enc_str(&self) -> std::result::Result<String, ServerError> {
        encrypt(&self.commands.str, &self.server.password)
    }
    pub fn find_command(&self, alias: &str) -> Option<&Command> {
        self.commands.vec.iter().find(|c| c.alias.eq(alias.trim()))
    }
    pub async fn pair_device(&mut self, payload: String) -> Result<()> {
        let device = decrypt(&payload, &self.server.password)?;

        // TODO Validate device format

        if self.paired_devices.contains(&device) {
            return Err("Device already paired".into());
        }

        self.paired_devices.push(device);

        write::sync::overwrite(
            self.paired_devices.join("\n"),
            &self.dirs.paired_devices_file,
        )
        .await
    }
    pub async fn unpair_device(&mut self, payload: String) -> Result<()> {
        let device = decrypt(&payload, &self.server.password)?;

        if !self.paired_devices.contains(&device) {
            return Err("Device already paired".into());
        }
        if let Some(index) = self.paired_devices.iter().position(|d| d == &device) {
            self.paired_devices.remove(index);
        }

        write::sync::overwrite(
            self.paired_devices.join("\n"),
            &self.dirs.paired_devices_file,
        )
        .await
    }
    pub fn update_paired_devices(&mut self) -> Result<()> {
        self.paired_devices = read::paired_devices(&self.dirs.paired_devices_file)?;
        Ok(())
    }
    pub fn get_address(&self) -> String {
        let port = self
            .server
            .port
            .clone()
            .unwrap_or_else(|| "6551".to_string());
        format!("0.0.0.0:{}", port)
    }
}
