// src/app/commands.rs

use super::task::Tarea; // Importa Tarea desde el módulo hermano `task.rs`
use crate::settings::Settings;
use crate::storage::{Storage, BincodeStorage, CsvStorage, JsonStorage, TxtStorage};
use std::error::Error;
use std::fs;

// La función procesar_comando ahora vive acá
pub fn procesar_comando(comando_interno: &str, args: &str, tareas: &mut Vec<Tarea>, settings: &Settings) -> bool {
    let mut cambio_realizado = false;
    match comando_interno {
        "agregar" => {
            if !args.is_empty() {
                tareas.push(Tarea {
                    descripcion: args.to_string(),
                    completada: false,
                    ultimo_cambio: None,
                });
                println!("✅ Tarea agregada.");
                cambio_realizado = true;
            } else {
                println!("❌ Error: El comando para agregar una tarea necesita una descripción.");
            }
        }
        "listar" => {
            if args.is_empty() {
                listar_tareas(tareas);
            } else {
                println!("❌ Error: El comando para listar tareas no acepta argumentos.");
            }
        }
        "completar" => {
            if args.is_empty() {
                println!("❌ Error: El comando para completar una tarea necesita un número de ID.");
            } else {
                match args.parse::<usize>() {
                    Ok(id) if id > 0 && id <= tareas.len() => {
                        let tarea = &mut tareas[id - 1];
                        tarea.completada = true;
                        tarea.ultimo_cambio = Some(chrono::Utc::now());
                        println!("✅ Tarea {} marcada como completada.", id);
                        cambio_realizado = true;
                    }
                    _ => println!("❌ ID de tarea no válido."),
                }
            }
        }
        "desmarcar" => {
            if args.is_empty() {
                println!("❌ Error: El comando para desmarcar una tarea necesita un número de ID.");
            } else {
                match args.parse::<usize>() {
                    Ok(id) if id > 0 && id <= tareas.len() => {
                        let tarea = &mut tareas[id - 1];
                        tarea.completada = false;
                        tarea.ultimo_cambio = Some(chrono::Utc::now());
                        println!("✅ Tarea {} marcada como pendiente.", id);
                        cambio_realizado = true;
                    }
                    _ => println!("❌ ID de tarea no válido."),
                }
            }
        }
        "importar" | "cargar" => {
            if !args.is_empty() {
                match cargar_desde_archivo(args) {
                    Ok(tareas_importadas) => {
                        let cantidad = tareas_importadas.len();
                        if comando_interno == "importar" {
                            tareas.extend(tareas_importadas);
                            println!("✅ Se importaron y agregaron {} tareas.", cantidad);
                        } else {
                            *tareas = tareas_importadas;
                            println!("✅ Se cargaron {} tareas. La lista anterior fue reemplazada.", cantidad);
                        }
                        cambio_realizado = true;
                    }
                    Err(e) => eprintln!("Error al importar el archivo: {}", e),
                }
            } else {
                println!("❌ El comando '{}' necesita la ruta a un archivo.", comando_interno);
            }
        }
        "explicar" => {
            if let Some(explicacion) = settings.explicaciones.get(args) {
                println!("\nAyuda para '{}':\n{}", args, explicacion);
            } else {
                println!("No encontré ayuda para el comando '{}'.", args);
            }
        }
        _ => {
            println!("❌ Error interno: Comando '{}' no reconocido.", comando_interno);
        }
    }
    cambio_realizado
}

// Las funciones auxiliares también viven acá
fn cargar_desde_archivo(path: &str) -> Result<Vec<Tarea>, Box<dyn Error>> {
    if !fs::metadata(path).is_ok() {
        return Err(format!("No se pudo encontrar el archivo en la ruta '{}'", path).into());
    }
    let extension = std::path::Path::new(path).extension().and_then(std::ffi::OsStr::to_str).unwrap_or("");
    let storage: Storage = match extension {
        "csv" => Storage::Csv(CsvStorage),
        "json" => Storage::Json(JsonStorage),
        "txt" => Storage::Txt(TxtStorage),
        "bincode" => Storage::Bincode(BincodeStorage),
        _ => return Err("Formato de archivo no soportado. Usá .csv, .json, .txt, o .bincode.".into()),
    };
    storage.load(path)
}

fn listar_tareas(lista_de_tareas: &Vec<Tarea>) {
    println!("\nLista de Tareas:");
    for (i, tarea) in lista_de_tareas.iter().enumerate() {
        tarea.mostrar(i + 1);
    }
}

// src/app/commands.rs

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings::{Settings, StorageSettings};
    use std::collections::HashMap;

    // Función auxiliar para crear un objeto `Settings` vacío para los tests
    fn mock_settings() -> Settings {
        Settings {
            storage: StorageSettings {
                filename_base: String::new(),
                format: String::new(),
            },
            comandos: HashMap::new(),
            explicaciones: HashMap::new(),
        }
    }

    #[test]
    fn test_procesar_agregar_tarea() {
        let mut tareas = Vec::new();
        let settings = mock_settings();
        procesar_comando("agregar", "Nueva tarea de prueba", &mut tareas, &settings);
        
        assert_eq!(tareas.len(), 1);
        assert_eq!(tareas[0].descripcion, "Nueva tarea de prueba");
        assert_eq!(tareas[0].completada, false);
    }

    #[test]
    fn test_procesar_completar_y_desmarcar_tarea() {
        let mut tareas = vec![
            Tarea { descripcion: "Tarea para testear".to_string(), completada: false, ultimo_cambio: None },
        ];
        let settings = mock_settings();

        // Probar completar
        procesar_comando("completar", "1", &mut tareas, &settings);
        assert_eq!(tareas[0].completada, true);
        assert!(tareas[0].ultimo_cambio.is_some());

        // Probar desmarcar
        procesar_comando("desmarcar", "1", &mut tareas, &settings);
        assert_eq!(tareas[0].completada, false);
    }
}