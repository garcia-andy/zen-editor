use std::path::PathBuf;
use chrono::{DateTime, Local};

#[derive(Debug, Clone)]
pub struct FileInfo {
    pub path: PathBuf,
    pub content: String,
    pub last_mod: DateTime<Local>
}

impl FileInfo {
    pub fn new(path: PathBuf, content: String) -> Self {
        Self { path, content, last_mod: Local::now() }
    }
}