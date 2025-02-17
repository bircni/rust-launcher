use anyhow::Context as _;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

use super::{commands::CommandItem, Group};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub groups: Vec<Group>,
    pub commands: Vec<CommandItem>,
    #[serde(skip)]
    pub path: PathBuf,
}

impl Config {
    pub fn load(path: &PathBuf) -> anyhow::Result<Self> {
        let mut config = if fs::exists(path)? {
            let config = fs::read_to_string(path)?;
            serde_json::from_str::<Self>(&config)?
        } else {
            Self::default()
        };
        config.path.clone_from(path);

        Ok(config)
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let config = serde_json::to_string_pretty(self)?;
        fs::write(&self.path, config).context("Failed to write config file")
    }

    pub fn add_group(&mut self, group: Group) {
        let id = self.groups.len() as i32;
        let mut grp = group;
        grp.id = id;
        self.groups.push(grp);
    }

    pub fn add_command(&mut self, cmd: CommandItem) {
        self.commands.push(cmd);
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            groups: vec![Group::new("Default".to_owned(), 0)],
            commands: Vec::default(),
            path: PathBuf::default(),
        }
    }
}
