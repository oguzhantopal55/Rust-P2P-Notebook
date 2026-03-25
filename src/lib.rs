use slint::{ModelRc, SharedString, VecModel};
use std::fs::{File, OpenOptions};
use std::io::Write;
use anyhow::{Result};
use iroh::{protocol::Router, Endpoint, SecretKey, endpoint::presets};
use iroh_blobs::{store::mem::MemStore, BlobsProtocol, ticket::BlobTicket};
use rand::rng;
use tokio;
slint::include_modules!();
mod db;


pub fn read_file_names(ui: &MainWindow) {
    let content = std::fs::read_to_string("files/filenames.txt").unwrap_or_default();

    let lines: Vec<SharedString> = content.lines().map(SharedString::from).collect();

    let model = ModelRc::new(VecModel::from(lines));
    ui.set_files(model);
}

pub fn read_current_file(ui: &MainWindow) {
    let current_file = ui.get_current_file();
    let curr = current_file.to_string();
    let path = format!("files/{}.txt", curr);

    let contents = std::fs::read_to_string(&path).unwrap_or_default();

    ui.set_content(SharedString::from(contents));
}

pub fn create_file(ui: &MainWindow) -> std::io::Result<()> {
    let filename = format!("files/{}.txt", ui.get_new_file());
    let mut file = File::create(filename)?;
    file.write_all(b"Selam Sana Imparator")?;
    Ok(())
} // hazir

pub fn write_file(ui: &MainWindow) -> std::io::Result<()> {
    let filename = format!("files/{}.txt", ui.get_current_file());
    println!("Saving to: {}", filename);
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&filename)?;

    let overwrite = ui.get_content();
    file.write_all(overwrite.as_bytes())?;
    Ok(())
}

pub fn add_file(ui: &MainWindow) -> std::io::Result<()> {
    let filename = "files/filenames.txt";
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(&filename)?;
    file.write(b"\n")?;
    file.write(ui.get_new_file().as_bytes())?;
    Ok(())
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

pub async fn send_file(ep: &Endpoint) -> Result<()>{
    let store = MemStore::new();
    let tag = store.add_slice(b"Hello world").await?;
  
    let _ = ep.online().await;
    let addr = ep.addr();
    let ticket = BlobTicket::new(addr, tag.hash, tag.format);

    // build the router
    let blobs = BlobsProtocol::new(&store, None);
    let router = Router::builder(ep.clone())
        .accept(iroh_blobs::ALPN, blobs)
        .spawn();

    println!("We are now serving {}", ticket);

    // wait for control-c
    tokio::signal::ctrl_c().await?;

    // clean shutdown of router and store
    router.shutdown().await?;
    Ok(())
}