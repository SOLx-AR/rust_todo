use crate::app::task::Tarea;

pub fn ejecutar(args: &str, tareas: &mut Vec<Tarea>) -> bool {
    let mut parts = args.split_whitespace();
    let id_str = parts.next();
    let tag_a_eliminar = parts.next();

    if let (Some(id_str), Some(tag_a_eliminar)) = (id_str, tag_a_eliminar) {
        match id_str.parse::<usize>() {
            Ok(id) if id > 0 && id <= tareas.len() => {
                let tarea = &mut tareas[id - 1];
                
                if let Some(pos) = tarea.tags.iter().position(|t| t == tag_a_eliminar) {
                    tarea.tags.remove(pos);
                    println!("✅ Etiqueta '{}' eliminada de la tarea {}.", tag_a_eliminar, id);
                    true
                } else {
                    println!("👍 La tarea no tenía la etiqueta '{}'.", tag_a_eliminar);
                    false
                }
            }
            _ => {
                println!("❌ ID de tarea no válido.");
                false
            }
        }
    } else {
        println!("❌ Error: El comando 'desetiquetar' necesita un ID de tarea y un nombre de etiqueta.");
        false
    }
}