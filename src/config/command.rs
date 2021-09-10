use std::str::FromStr;

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Command {
    pub string: String,
    pub alias: String,
    pub command: String,
    pub args: Option<Vec<String>>,
}

// TODO Ugly
impl Command {
    pub fn execute(&self) -> bool {
        if let Some(args) = &self.args {
            if let Err(e) = std::process::Command::new(&self.command)
                .args(args)
                .output()
            {
                log::error!("Stderr running command: {} {:?}", &self.alias, e.kind());
                return false;
            };

            return true;
        }
        if let Err(e) = std::process::Command::new(&self.command).output() {
            log::error!("Stderr running command: {} {:?}", self.alias, e.kind());
            return false;
        };

        true
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
