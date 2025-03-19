use std::{fs::{metadata, read_to_string}, path::PathBuf, time::SystemTime};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct MarkdownFile {
    pub path: PathBuf,
    pub name: String,
    pub last_modified: Option<SystemTime>,
    pub is_read_only: bool,
    pub content: String
}

impl MarkdownFile {
    pub fn new(path: PathBuf) -> Self {
        let meta = metadata(&path).ok();
        let name = path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("")
            .to_string();
        
        Self {
            path: path.clone(),
            name,
            last_modified: meta.as_ref().map(|m| m.modified().ok()).unwrap(),
            is_read_only: meta.map(|m| m.permissions().readonly()).unwrap_or(false),
            content: read_to_string(path).unwrap_or("".to_string()).to_string()
        }
    }
}
