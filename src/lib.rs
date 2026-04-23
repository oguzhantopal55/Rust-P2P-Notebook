use slint::{ModelRc, SharedString, VecModel};
use rusqlite::Connection;
slint::include_modules!();
pub mod db;
use db::*;

pub fn ui_read_note_names(conn: &Connection, ui: &MainWindow) {
    let names = read_note_names(conn).unwrap_or_default();
    let lines: Vec<SharedString> = names.iter().map(SharedString::from).collect();
    let model = ModelRc::new(VecModel::from(lines));
    ui.set_files(model);
}

pub fn ui_read_current_note(conn: &Connection, ui: &MainWindow) {
    let name = ui.get_current_file().to_string();
    let content = read_note(conn, name).unwrap_or_default().unwrap_or_default();
    ui.set_content(SharedString::from(content));
}

