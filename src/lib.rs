use slint::{ModelRc, SharedString, VecModel};
use anyhow::Result;
use iroh::{protocol::Router, Endpoint, SecretKey, endpoint::presets};
use iroh_blobs::{store::mem::MemStore, BlobsProtocol, ticket::BlobTicket};
use rand::rng;
use rusqlite::Connection;
use tokio;
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

pub async fn iroh_end_point() -> Result<()> {
    let sec_key = SecretKey::generate(&mut rng());
    let end_point = Endpoint::builder()
        .secret_key(sec_key)
        .bind()
        .await?;
    println!("> our node id: {}", end_point.id());
    Ok(())
}

pub async fn send_file(ep: &Endpoint) -> Result<()> {
    let store = MemStore::new();
    let tag = store.add_slice(b"Hello world").await?;
    let _ = ep.online().await;
    let addr = ep.addr();
    let ticket = BlobTicket::new(addr, tag.hash, tag.format);
    let blobs = BlobsProtocol::new(&store, None);
    let router = Router::builder(ep.clone())
        .accept(iroh_blobs::ALPN, blobs)
        .spawn();
    println!("We are now serving {}", ticket);
    tokio::signal::ctrl_c().await?;
    router.shutdown().await?;
    Ok(())
}