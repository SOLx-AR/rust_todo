use crate::app::task::Tarea;
use super::cargar_desde_archivo;

pub fn ejecutar(args: &str, tareas: &mut Vec<Tarea>) -> bool {
    if !args.is_empty() {
        match cargar_desde_archivo(args) {
            Ok(tareas_importadas) => {
                let cantidad = tareas_importadas.len();
                tareas.extend(tareas_importadas);
                println!("✅ Se importaron y agregaron {} tareas.", cantidad);
                true
            }
            Err(e) => {
                eprintln!("Error al importar el archivo: {}", e);
                false
            }
        }
    } else {
        println!("❌ El comando para importar necesita la ruta a un archivo.");
        false
    }
}