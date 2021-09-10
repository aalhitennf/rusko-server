use std::path::Path;

use crate::{
    config::{Command, Commands},
    Result,
};

pub fn commands<T>(path: T) -> Result<Commands>
where
    T: AsRef<Path>,
{
    use std::cmp::Ordering;
    use std::str::FromStr;

    let content: Vec<Command> = std::fs::read_to_string(path)?
        .lines()
        .flat_map(Command::from_str)
        .collect();

    // Warn about duplicates
    let mut dedups = content.clone();
    dedups.sort_by_key(|c| c.alias.clone());
    dedups.dedup_by_key(|c| c.alias.clone());

    if content.len().cmp(&dedups.len()) == Ordering::Greater {
        log::error!(
            "Found {} commands with same alias! Only first one of these will work.",
            content.len() - dedups.len()
        );
    }

    let as_str: String = content.iter().map(|c| format!("{}\n", c.string)).collect();

    Ok(Commands {
        vec: content,
        str: as_str,
    })
}

pub fn paired_devices<T>(path: T) -> Result<Vec<String>>
where
    T: AsRef<Path>,
{
    Ok(std::fs::read_to_string(path)?
        .split('\n')
        .map(str::to_string)
        .filter(|s| !s.is_empty())
        .collect::<Vec<String>>())
}

#[cfg(test)]
mod tests {

    use std::{fs::File, io::Write};
    use tempdir::TempDir;

    use crate::Result;

    #[test]
    fn read_and_parse_commands() -> Result<()> {
        let temp_dir = TempDir::new("parse_commands")?;
        let file_path = temp_dir.path().join("commands");
        let mut file = File::create(&file_path)?;
        file.write_all(b"Test :: test command\nThis : command fails")?;
        file.sync_all()?;
        let commands = super::commands(file_path)?;
        assert_eq!(commands.vec.len(), 1);
        assert_eq!(commands.str.len(), 21);
        temp_dir.close()?;
        Ok(())
    }
}
