use crate::app::task::Tarea;

pub fn ejecutar(args: &str, tareas: &mut Vec<Tarea>) -> bool {
    if !args.is_empty() {
        tareas.push(Tarea {
            descripcion: args.to_string(),
            completada: false,
            ultimo_cambio: None,
            tags: Vec::new(),
        });
        println!("✅ Tarea agregada.");
        true
    } else {
        println!("❌ Error: El comando para agregar una tarea necesita una descripción.");
        false
    }
}