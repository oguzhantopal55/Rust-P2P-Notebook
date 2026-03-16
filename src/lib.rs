mod p2p;

use slint::{ModelRc, SharedString, VecModel};
use std::fs::{File, OpenOptions};
use std::io::Write;

slint::include_modules!();

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
}

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
