//use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Write;
use slint::{ComponentHandle, ModelRc, SharedString, VecModel};

slint::include_modules!();

fn main() {

    let ui = MainWindow::new().unwrap();

    read_file_names(&ui);

    let ui_weak_current = ui.as_weak();
    ui.on_current(move || {
        if let Some(ui) = ui_weak_current.upgrade() {
            read_current_file(&ui);
        }
    });

    let ui_weak_new = ui.as_weak();
    ui.on_new(move || {
        if let  Some(ui) = ui_weak_new.upgrade()  {
            let result = create_file(&ui);
            if let Err(_e) = result {
                println!("Error Creating File");
            }
            read_file_names(&ui);
        }
    });

    let ui_weak_write = ui.as_weak();
    ui.on_write(move || {
        if let Some(ui) = ui_weak_write.upgrade() {
            let res = write_file(&ui);
            if let Err(_e) = res{
                println!("Error Writing");
            }
        }
    });
    ui.run().unwrap();

}

fn read_file_names(ui: &MainWindow) {
    let content = std::fs::read_to_string("files/filenames.txt").unwrap_or_default();

    let lines: Vec<SharedString> = content
        .lines()
        .map(SharedString::from)
        .collect();

    let model = ModelRc::new(VecModel::from(lines));
    ui.set_files(model);
}

fn read_current_file(ui: &MainWindow) {
    let current_file = ui.get_current_file();
    let curr = current_file.to_string();
    let path = format!("files/{}.txt", curr);

    let contents = std::fs::read_to_string(&path).unwrap_or_default();

    ui.set_content(SharedString::from(contents));
}

fn create_file(ui: &MainWindow) -> std::io::Result<()>{
    let filename = format!("files/{}.txt", ui.get_new_file());
    let mut file = File::create(filename)?;
    file.write_all(b"Selam Sana Imparator")?;
    Ok(())
}

fn write_file(ui: &MainWindow) -> std::io::Result<()> {
    let filename = format!("files/{}.txt", ui.get_written_file());
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)   
        .truncate(true) 
        .open(&filename)?; 

    let overwrite = ui.get_content();
    file.write_all(overwrite.as_bytes())?;
    Ok(())
}