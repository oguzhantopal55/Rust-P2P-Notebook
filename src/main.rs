use std::sync::{Arc, Mutex};
use slint::ComponentHandle;
use ttt::*;
use ttt::db::{open_db, create_note, write_note};

fn main() {
    let conn = open_db().unwrap();
    let conn = Arc::new(Mutex::new(conn));

    let ui = MainWindow::new().unwrap();

    ui_read_note_names(&conn.lock().unwrap(), &ui);

    let conn_current = conn.clone();
    let ui_weak_current = ui.as_weak();
    ui.on_current(move || {
        if let Some(ui) = ui_weak_current.upgrade() {
            let conn = conn_current.lock().unwrap();
            ui_read_current_note(&conn, &ui);
        }
    });

    let conn_new = conn.clone();
    let ui_weak_new = ui.as_weak();
    ui.on_new(move || {
        if let Some(ui) = ui_weak_new.upgrade() {
            let conn = conn_new.lock().unwrap();
            if let Err(_) = create_note(&conn, ui.get_new_file().to_string()) {
                println!("Error Creating Note");
            }
            ui_read_note_names(&conn, &ui);
        }
    });

    let conn_write = conn.clone();
    let ui_weak_write = ui.as_weak();
    ui.on_write(move || {
        if let Some(ui) = ui_weak_write.upgrade() {
            let conn = conn_write.lock().unwrap();
            if let Err(_) = write_note(&conn, ui.get_content().to_string(), ui.get_current_file().to_string()) {
                println!("Error Writing Note");
            }
        }
    });
    
    ui.run().unwrap();
}