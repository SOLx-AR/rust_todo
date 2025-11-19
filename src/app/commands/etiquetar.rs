use crate::app::task::Tarea;
use crate::settings::Settings;

pub fn ejecutar(args: &str, tareas: &mut Vec<Tarea>, settings: &Settings) -> bool {
    let mut parts = args.split_whitespace();
    let id_str = parts.next();
    let tag = parts.next();

    if let (Some(id_str), Some(tag)) = (id_str, tag) {
        if !settings.general.tags_validos.contains(&tag.to_string()) {
            println!("❌ Error: La etiqueta '{}' no es válida.", tag);
            return false;
        }

        match id_str.parse::<usize>() {
            Ok(id) if id > 0 && id <= tareas.len() => {
                let tarea = &mut tareas[id - 1];
                if !tarea.tags.contains(&tag.to_string()) {
                    tarea.tags.push(tag.to_string());
                    println!("✅ Etiqueta '{}' agregada a la tarea {}.", tag, id);
                    true
                } else {
                    println!("👍 La tarea ya tenía la etiqueta '{}'.", tag);
                    false
                }
            }
            _ => {
                println!("❌ ID de tarea no válido.");
                false
            }
        }
    } else {
        println!("❌ Error: El comando 'etiquetar' necesita un ID de tarea y un nombre de etiqueta.");
        false
    }
}