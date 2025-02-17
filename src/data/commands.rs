use std::process::Command;

use serde::{Deserialize, Serialize};

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
