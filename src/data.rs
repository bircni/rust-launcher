use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf, process::Command};

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

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Group {
    pub name: String,
    pub id: i32,
}

impl Group {
    pub const fn new(name: String, id: i32) -> Self {
        Self { name, id }
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CommandItem {
    pub command: String,
    pub id: i32,
    pub group_id: i32,
    pub name: String,
    pub working_dir: String,
    pub linked_command_id: Option<i32>,
}

impl CommandItem {
    pub fn run(&self) -> CommandResult {
        // let dir = PathBuf::from(&self.working_dir);
        #[cfg(target_os = "windows")]
        let mut cmd = Command::new("powershell");
        #[cfg(not(target_os = "windows"))]
        let mut cmd = Command::new("sh");
        if !self.working_dir.is_empty() {
            cmd.current_dir(&self.working_dir);
        }
        cmd.arg("-C").arg(&self.command);
        match cmd.output() {
            Ok(output) => CommandResult::new(
                String::from_utf8_lossy(&output.stdout).to_string(),
                String::from_utf8_lossy(&output.stderr).to_string(),
                output.status.code().unwrap_or(1),
                self.command.clone(),
            ),
            Err(e) => CommandResult::new(String::default(), e.to_string(), 1, self.command.clone()),
        }
    }
}

#[derive(Default)]
pub struct CommandResult {
    pub stdout: String,
    pub stderr: String,
    pub status: i32,
    pub executed_cmd: String,
}

impl CommandResult {
    pub const fn new(stdout: String, stderr: String, status: i32, executed_cmd: String) -> Self {
        Self {
            stdout,
            stderr,
            status,
            executed_cmd,
        }
    }
}
