use std::process::exit;
use crate::viewer::ui::Viewer;

mod todo_db;
mod viewer;
mod todo_core;

fn main() {
    let mut viewer = Viewer::new();
    match viewer.init_db() {
        Ok(_) => {println!("Ok"); viewer.load();},
        Err(e) => {eprintln!("Errore: {}", e); exit(-1);}
    }
    viewer.main_loop();
}
