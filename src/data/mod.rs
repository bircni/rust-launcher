use serde::{Deserialize, Serialize};

pub mod commands;
pub mod config;

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
