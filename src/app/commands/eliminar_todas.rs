use crate::app::task::Tarea;
use crate::settings::Settings;
use std::fs;

pub fn ejecutar(_args: &str, tareas: &mut Vec<Tarea>, settings: &Settings) -> bool {
    let filename = format!("{}.{}", settings.storage.filename_base, settings.storage.format);
    match fs::remove_file(&filename) {
        Ok(_) => {
            tareas.clear();
            println!("✅ Todas las tareas fueron eliminadas.");
            true
        }
        Err(_) => {
            println!("👍 No había tareas para eliminar.");
            false
        }
    }
}