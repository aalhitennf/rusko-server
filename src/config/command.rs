use std::str::FromStr;

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Command {
    pub string: String,
    pub alias: String,
    pub command: String,
    pub args: Option<Vec<String>>,
}

// Prevent process lockdown by running command in thread
impl Command {
    pub fn execute(&self) {
        let cmd = self.command.clone();
        let alias = self.alias.clone();
        match &self.args {
            Some(args) => {
                let args = args.clone();
                std::thread::spawn(move || {
                    if let Err(e) = std::process::Command::new(&cmd).args(args).output() {
                        log::error!("Stderr running command: {} {:?}", &alias, e.kind());
                    };
                });
            }
            None => {
                std::thread::spawn(move || {
                    if let Err(e) = std::process::Command::new(&cmd).output() {
                        log::error!("Stderr running command: {} {:?}", alias, e.kind());
                    };
                });
            }
        }
    }
}

impl FromStr for Command {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((mut alias, mut full_command)) = s.split_once("::") {
            alias = alias.trim();
            full_command = full_command.trim();

            let command;
            let args: Option<Vec<String>>;

            if let Some(s) = full_command.split_once(' ') {
                command = s.0;
                args = shlex::split(s.1);
            } else {
                command = full_command;
                args = None;
            }

            return Ok(Command {
                string: s.into(),
                alias: alias.into(),
                command: command.into(),
                args,
            });
        }
        log::error!("Failed to parse command: {}", s);
        Err("Failed to parse command".into())
    }
}
