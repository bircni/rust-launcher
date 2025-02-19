#![expect(
    clippy::unwrap_used,
    clippy::redundant_closure_call,
    reason = "this is a test"
)]
use std::{fs, path::PathBuf};

use crate::{
    data::{commands::CommandItem, config::Config, Group},
    utils,
};

mod ui_tests;

#[test]
fn test_get_config_path() {
    utils::get_config_path().unwrap();
}

#[test]
fn test_config() {
    let path = PathBuf::from("test.json");
    let result: anyhow::Result<()> = (|| {
        let mut config = Config::load(&path).unwrap();
        assert_eq!(config.groups.len(), 1);
        assert_eq!(config.commands.len(), 0);

        let group = Group::new("Test".to_owned(), 1);
        config.add_group(group);
        assert_eq!(config.groups.len(), 2);

        let cmd = CommandItem {
            command: "echo 'Hello World'".to_owned(),
            id: 0,
            group_id: 1,
            name: "Test Command".to_owned(),
            working_dir: String::default(),
            linked_command_id: None,
        };
        config.add_command(cmd);
        assert_eq!(config.commands.len(), 1);

        config.save().unwrap();
        let config = Config::load(&path).unwrap();
        assert_eq!(config.groups.len(), 2);
        assert_eq!(config.commands.len(), 1);

        Ok(())
    })();

    // Ensure the file is always removed
    match fs::remove_file(&path) {
        Ok(()) => (),
        Err(err) => eprintln!("Failed to remove file: {err}"),
    }

    // Propagate the error if any
    result.unwrap();
}

#[test]
fn test_command_run() {
    let cmd = CommandItem {
        command: "echo 'Hello World'".to_owned(),
        id: 0,
        group_id: 1,
        name: "Test Command".to_owned(),
        working_dir: String::default(),
        linked_command_id: None,
    };
    let result = cmd.run();
    assert_eq!(result.status, 0);
    assert_eq!(result.stdout.trim_ascii(), "Hello World");
    assert!(result.stderr.is_empty());
}
