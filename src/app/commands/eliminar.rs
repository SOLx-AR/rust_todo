use crate::app::task::Tarea;

pub fn ejecutar(args: &str, tareas: &mut Vec<Tarea>) -> bool {
    if args.is_empty() {
        println!("❌ Error: El comando para eliminar necesita un número de ID.");
        false
    } else {
        match args.parse::<usize>() {
            Ok(id) if id > 0 && id <= tareas.len() => {
                let tarea_eliminada = tareas.remove(id - 1);
                println!("✅ Tarea eliminada: '{}'", tarea_eliminada.descripcion);
                true
            }
            _ => {
                println!("❌ ID de tarea no válido.");
                false
            }
        }
    }
}