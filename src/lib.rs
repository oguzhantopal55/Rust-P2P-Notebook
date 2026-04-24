use anyhow::Result;
use iroh::{Endpoint, SecretKey, endpoint::presets};
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

pub async fn create_endpoint(conn: &Connection) -> anyhow::Result<Endpoint> {
    let secret_key = match get_user_secret(conn)? {
        Some(stored) => {
            let bytes = hex::decode(&stored)?;
            SecretKey::try_from(bytes.as_slice())?
        }
        None => SecretKey::generate(),
    };

    let endpoint = Endpoint::builder(presets::N0)
        .secret_key(secret_key.clone())
        .bind()
        .await?;

    if get_user(conn)?.is_none() {
        let secret_hex = hex::encode(secret_key.to_bytes());
        let node_id = endpoint.id().to_string(); // verify method name
        create_user(conn, "default".to_string(), secret_hex, node_id)?;
    }

    Ok(endpoint)
}

