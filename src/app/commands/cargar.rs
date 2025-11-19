use crate::app::task::Tarea;
use super::cargar_desde_archivo;

pub fn ejecutar(args: &str, tareas: &mut Vec<Tarea>) -> bool {
    if !args.is_empty() {
        match cargar_desde_archivo(args) {
            Ok(tareas_cargadas) => {
                let cantidad = tareas_cargadas.len();
                *tareas = tareas_cargadas;
                println!("✅ Se cargaron {} tareas. La lista anterior fue reemplazada.", cantidad);
                true 
            }
            Err(e) => {
                eprintln!("Error al cargar el archivo: {}", e);
                false
            }
        }
    } else {
        println!("❌ El comando para cargar necesita la ruta a un archivo.");
        false
    }
}