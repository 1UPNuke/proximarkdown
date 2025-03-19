use std::path::PathBuf;

use tauri::{menu::MenuEvent, AppHandle, Emitter, Wry};
use rfd::FileDialog;

use crate::markdown_file::MarkdownFile;

pub fn new_file(app: &AppHandle<Wry>, _: MenuEvent) {
    app.emit("open-file", MarkdownFile::new(PathBuf::new())).unwrap();
}

pub fn open_file(app: &AppHandle<Wry>, _: MenuEvent) {
    let path = FileDialog::new()
        .add_filter("Markdown", &["md", "txt", ""])
        .pick_file();
    if path.is_none() { return }
    let meta = MarkdownFile::new(path.unwrap());
    app.emit("open-file", meta).unwrap();
}