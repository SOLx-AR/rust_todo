use crate::app::task::Tarea;
use super::listar_tareas;

pub fn ejecutar(args: &str, tareas: &mut Vec<Tarea>) -> bool {
    if args.is_empty() {
        listar_tareas(tareas);
    } else {
        println!("❌ Error: El comando para listar tareas no acepta argumentos.");
    }
    false
}