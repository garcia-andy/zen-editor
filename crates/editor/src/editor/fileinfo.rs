use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct FileInfo {
    pub path: PathBuf,
    pub content: String,
}

impl FileInfo {
    pub fn new(path: PathBuf, content: String) -> Self {
        Self { path, content }
    }
}