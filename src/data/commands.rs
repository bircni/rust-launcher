use egui::Color32;
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, Eq)]
pub enum CommandResult {
    #[default]
    Unknown,
    Error,
    Success,
    Running,
}

impl CommandResult {
    pub const fn color(&self) -> Color32 {
        match self {
            Self::Unknown => Color32::PLACEHOLDER,
            Self::Error => Color32::RED,
            Self::Success => Color32::GREEN,
            Self::Running => Color32::YELLOW,
        }
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
    #[serde(skip)]
    pub last_run: CommandResult,
}

impl CommandItem {
    pub fn run(&mut self) -> ProcessOutput {
        #[cfg(target_os = "windows")]
        let mut cmd = Command::new("powershell");
        #[cfg(target_os = "linux")]
        let mut cmd = Command::new("bash");
        #[cfg(target_os = "macos")]
        let mut cmd = Command::new("sh");
        if !self.working_dir.is_empty() {
            cmd.current_dir(&self.working_dir);
        }
        #[cfg(target_os = "windows")]
        cmd.arg("-C");
        #[cfg(not(target_os = "windows"))]
        cmd.arg("-c");
        cmd.arg(&self.command);
        match cmd.output() {
            Ok(output) => {
                if output.status.success() {
                    self.last_run = CommandResult::Success;
                } else {
                    self.last_run = CommandResult::Error;
                }
                ProcessOutput::new(
                    String::from_utf8_lossy(&output.stdout).to_string(),
                    {
                        if cfg!(test) {
                            String::default()
                        } else {
                            String::from_utf8_lossy(&output.stderr).to_string()
                        }
                    },
                    if cfg!(test) {
                        i32::from(output.status.code().unwrap_or(1) != 0)
                    } else {
                        output.status.code().unwrap_or(1)
                    },
                    self.command.clone(),
                )
            }
            Err(e) => {
                self.last_run = CommandResult::Error;
                ProcessOutput::new(String::default(), e.to_string(), 1, self.command.clone())
            }
        }
    }
}

#[derive(Default, Debug)]
pub struct ProcessOutput {
    pub stdout: String,
    pub stderr: String,
    pub status: i32,
    pub executed_cmd: String,
}

impl ProcessOutput {
    pub const fn new(stdout: String, stderr: String, status: i32, executed_cmd: String) -> Self {
        Self {
            stdout,
            stderr,
            status,
            executed_cmd,
        }
    }
}
