use slint::ComponentHandle;
use ttt::*;
mod p2p;

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
            let result1 = add_file(&ui);
            if let Err(_e) = result1 {
                println!("Error Writing into filenames File");
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