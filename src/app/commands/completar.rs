use crate::app::task::Tarea;
use chrono::Utc;

pub fn ejecutar(args: &str, tareas: &mut Vec<Tarea>) -> bool {
    if args.is_empty() {
        println!("❌ Error: El comando para completar una tarea necesita un número de ID.");
        false
    } else {
        match args.parse::<usize>() {
            Ok(id) if id > 0 && id <= tareas.len() => {
                let tarea = &mut tareas[id - 1];
                tarea.completada = true;
                tarea.ultimo_cambio = Some(Utc::now());
                println!("✅ Tarea {} marcada como completada.", id);
                true
            }
            _ => {
                println!("❌ ID de tarea no válido.");
                false
            }
        }
    }
}