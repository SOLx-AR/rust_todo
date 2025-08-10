use config::{Config, File};
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::fs;
use std::io;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Tarea {
    descripcion: String,
    completada: bool,
}

impl Tarea {
    fn mostrar(&self, id: usize) {
        println!("{} {}: {}", if self.completada { "[X]" } else { "[ ]" }, id, self.descripcion);
    }
}

#[derive(Debug, Deserialize)]
struct Settings {
    filename_base: String,
    format: String,
}

enum Storage {
    Csv(CsvStorage),
    Json(JsonStorage),
    Txt(TxtStorage), // Agregamos esta variante
}

impl Storage {
    fn save(&self, file_path: &str, tasks: &Vec<Tarea>) -> Result<(), Box<dyn Error>> {
        match self {
            Storage::Csv(s) => s.save(file_path, tasks),
            Storage::Json(j) => j.save(file_path, tasks),
            Storage::Txt(t) => t.save(file_path, tasks), // Agregamos este caso
        }
    }

    fn load(&self, file_path: &str) -> Result<Vec<Tarea>, Box<dyn Error>> {
        match self {
            Storage::Csv(s) => s.load(file_path),
            Storage::Json(j) => j.load(file_path),
            Storage::Txt(t) => t.load(file_path), // Agregamos este caso
        }
    }
}

// --- Implementación para el formato TXT ---
struct TxtStorage;
impl TxtStorage {
    fn save(&self, file_path: &str, tasks: &Vec<Tarea>) -> Result<(), Box<dyn Error>> {
        let mut file_content = String::new();
        for task in tasks {
            let status = if task.completada { "completada" } else { "no completada" };
            file_content.push_str(&format!("{},{}\n", status, task.descripcion));
        }
        Ok(fs::write(file_path, file_content)?)
    }

    fn load(&self, file_path: &str) -> Result<Vec<Tarea>, Box<dyn Error>> {
        if !fs::metadata(file_path).is_ok() { return Ok(Vec::new()); }
        
        let file_content = fs::read_to_string(file_path)?;
        let mut tasks = Vec::new();

        for line in file_content.lines() {
            if let Some((status_str, description)) = line.split_once(',') {
                let completed = status_str == "completada";
                tasks.push(Tarea {
                    descripcion: description.to_string(),
                    completada: completed,
                });
            }
        }
        Ok(tasks)
    }
}

// --- Implementación para CSV ---
// La struct ahora es solo un marcador
struct CsvStorage;
impl CsvStorage {
    // Ahora son métodos comunes, no parte de un trait
    fn save(&self, file_path: &str, tasks: &Vec<Tarea>) -> Result<(), Box<dyn Error>> {
        let mut writer = csv::Writer::from_path(file_path)?;
        for task in tasks {
            writer.write_record(&[task.completada.to_string(), task.descripcion.clone()])?;
        }
        Ok(writer.flush()?)
    }

    fn load(&self, file_path: &str) -> Result<Vec<Tarea>, Box<dyn Error>> {
        if !fs::metadata(file_path).is_ok() { return Ok(Vec::new()); }
        let file = fs::File::open(file_path)?;
        let mut reader = csv::ReaderBuilder::new().has_headers(false).from_reader(file);
        let mut tasks = Vec::new();
        for result in reader.records() {
            let record = result?;
            tasks.push(Tarea {
                completada: record.get(0).unwrap_or("").parse().unwrap_or(false),
                descripcion: record.get(1).unwrap_or("").to_string(),
            });
        }
        Ok(tasks)
    }
}

// --- Implementación para JSON ---
struct JsonStorage;
impl JsonStorage {
    fn save(&self, file_path: &str, tasks: &Vec<Tarea>) -> Result<(), Box<dyn Error>> {
        let json_str = serde_json::to_string_pretty(tasks)?;
        Ok(fs::write(file_path, json_str)?)
    }

    fn load(&self, file_path: &str) -> Result<Vec<Tarea>, Box<dyn Error>> {
        if !fs::metadata(file_path).is_ok() { return Ok(Vec::new()); }
        let json_str = fs::read_to_string(file_path)?;
        if json_str.trim().is_empty() { return Ok(Vec::new()); }
        Ok(serde_json::from_str(&json_str)?)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let builder = Config::builder()
        // Seteamos los valores por defecto ANTES de agregar el archivo.
        .set_default("filename_base", "tareas")?
        .set_default("format", "txt")? // Nuestro nuevo default va a ser 'txt'
        // Ahora le mandamos el archivo. Si existe, sus valores pisan los defaults.
        .add_source(File::with_name("Settings.toml").required(false)); // required(false) permite que el programa ande incluso sin el archivo
    
    let settings: Settings = builder.build()?.try_deserialize()?;
    // --- Fin del bloque de configuración ---
    
    let filename = format!("{}.{}", settings.filename_base, settings.format);
    println!("Usando formato: '{}', Archivo: '{}'", settings.format, filename);
    
     let storage: Storage = match settings.format.as_str() {
        "csv" => Storage::Csv(CsvStorage),
        "json" => Storage::Json(JsonStorage),
        "txt" => Storage::Txt(TxtStorage), // Agregamos esta opción
        _ => {
            eprintln!("Error: Formato no soportado '{}' en Settings.toml.", settings.format);
            // Por las dudas, si el formato es desconocido, usamos 'txt' como salvavidas.
            Storage::Txt(TxtStorage) 
        }
    };

    let mut tareas = storage.load(&filename)?;

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        // Modo para Tests
        let comando = &args[1];
        let resto_args = &args[2..].join(" ");
        procesar_comando(comando, resto_args, &mut tareas);
    } else {
        // Modo Interactivo
        println!("Bienvenido al gestor de tareas");
        loop {
            println!("\nIngresá un comando ('agregar <descripcion>', 'listar', 'completar', 'salir')");
            let mut entrada = String::new();
            io::stdin().read_line(&mut entrada).expect("Error al leer la entrada");
            
            let entrada = entrada.trim();
            let mut parts = entrada.split_whitespace();
            
            if let Some(comando) = parts.next() {
                if comando == "salir" {
                    println!("\nSaliendo del gestor de tareas.");
                    break;
                }
                let resto_args = parts.collect::<Vec<&str>>().join(" ");
                procesar_comando(comando, &resto_args, &mut tareas);
            }
        }
    }

    storage.save(&filename, &tareas)?;

    Ok(())
}

fn procesar_comando(comando: &str, args: &str, tareas: &mut Vec<Tarea>) {
    match comando {
        "agregar" => {
            if !args.is_empty() {
                tareas.push(Tarea {
                    descripcion: args.to_string(),
                    completada: false,
                });
                println!("✅ Tarea agregada.");
            } else {
                println!("❌ La descripción de la tarea no puede estar vacía.");
            }
        }
        "listar" => {
            listar_tareas(tareas);
        }
        "completar" => {
            match args.parse::<usize>() {
                Ok(id) if id > 0 && id <= tareas.len() => {
                    tareas[id - 1].completada = true;
                    println!("✅ Tarea {} marcada como completada.", id);
                }
                _ => println!("❌ ID de tarea no válido."),
            }
        }
        _ => {
            println!("❌ Comando no reconocido.");
        }
    }
}

fn listar_tareas(lista_de_tareas: &Vec<Tarea>) {
    println!("\nLista de Tareas:");
    for (i, tarea) in lista_de_tareas.iter().enumerate() {
        tarea.mostrar(i + 1);
    }
}

/* Desafío 1:
        Refactorizar el código con un match en vez de if, else if, else
    
    Desafío 2:
        Guardar las tareas en un archivo

    Desafío 3:
        Investigar el crate serde y cómo se usaría para serializar y deserializar las tareas
    
    Desafío 4:
        Cargar las tareas desde un archivo al iniciar el programa 
*/

#[cfg(test)]
mod tests {
    use super::*; // Nos da acceso a Tarea, procesar_comando, etc.

    #[test]
    fn crear_y_completar_tarea() {
        let mut tarea = Tarea {
            descripcion: "Aprender a testear en Rust".to_string(),
            completada: false,
        };
        assert_eq!(tarea.completada, false);
        tarea.completada = true;
        assert_eq!(tarea.completada, true);
    }

    #[test]
    fn procesar_agregar_tarea() {
        let mut tareas = Vec::new();
        procesar_comando("agregar", "Esta es una nueva tarea", &mut tareas);
        
        assert_eq!(tareas.len(), 1);
        assert_eq!(tareas[0].descripcion, "Esta es una nueva tarea");
        assert_eq!(tareas[0].completada, false);
    }

    #[test]
    fn procesar_completar_tarea_existente() {
        let mut tareas = vec![
            Tarea { descripcion: "Tarea 1".to_string(), completada: false },
        ];
        procesar_comando("completar", "1", &mut tareas);
        assert_eq!(tareas[0].completada, true);
    }

    #[test]
    fn procesar_completar_tarea_invalida() {
        let mut tareas = vec![
            Tarea { descripcion: "Tarea 1".to_string(), completada: false },
        ];
        // Intentamos completar una tarea que no existe (ID 2)
        procesar_comando("completar", "2", &mut tareas);
        // La tarea original debería seguir incompleta
        assert_eq!(tareas[0].completada, false);

        // Intentamos completar con un ID que no es número
        procesar_comando("completar", "abc", &mut tareas);
        assert_eq!(tareas[0].completada, false);
    }

    #[test]
    fn test_storage_txt_salva_y_carga() {
        // 1. Preparamos el storage y un archivo de prueba
        let storage = TxtStorage;
        let file_path = "test_tareas_temp.txt";

        // Limpiamos antes de arrancar
        fs::remove_file(file_path).ok();

        // 2. Creamos datos de ejemplo
        let tareas_originales = vec![
            Tarea {
                descripcion: "Prima attività".to_string(),
                completada: false,
            },
            Tarea {
                descripcion: "Seconda attività, con virgola".to_string(),
                completada: true,
            },
        ];

        // 3. Guardamos los datos
        storage.save(file_path, &tareas_originales).unwrap();

        // 4. Cargamos los datos
        let tareas_cargadas = storage.load(file_path).unwrap();

        // 5. Verificamos que los datos estén correctos
        assert_eq!(tareas_originales, tareas_cargadas);

        // Limpiamos al terminar el test
        fs::remove_file(file_path).ok();
    }
}