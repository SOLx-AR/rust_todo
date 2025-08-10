use config::{Config, File};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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

// Struct para la sección [storage]
#[derive(Debug, Deserialize)]
struct StorageSettings {
    filename_base: String,
    format: String,
}

fn default_comandos() -> HashMap<String, String> {
    // Creamos un mapa de comandos por defecto
    HashMap::from([
        ("agregar".to_string(), "agregar".to_string()),
        ("listar".to_string(), "listar".to_string()),
        ("completar".to_string(), "completar".to_string()),
        ("importar".to_string(), "importar".to_string()),
        ("cargar".to_string(), "cargar".to_string()),
        ("ayuda".to_string(), "explicar".to_string()),
    ])
}

fn default_explicaciones() -> HashMap<String, String> {
    // Creamos un mapa de explicaciones por defecto
    HashMap::from([
        ("agregar".to_string(), "Uso: agregar <descripción>. Agrega una nueva tarea a la lista.".to_string()),
        ("listar".to_string(), "Uso: listar. Muestra todas las tareas.".to_string()),
        ("completar".to_string(), "Uso: completar <número de tarea>. Marca una tarea como completada.".to_string()),
        ("importar".to_string(), "Uso: importar <ruta_al_archivo>. Agrega tareas de un archivo a la lista actual.".to_string()),
        ("cargar".to_string(), "Uso: cargar <ruta_al_archivo>. Reemplaza la lista actual con las tareas de un archivo.".to_string()),
        ("explicar".to_string(), "Uso: explicar <comando>. Muestra la ayuda para un comando específico.".to_string()),
    ])
}

// Struct principal que contiene todas las demás configuraciones
#[derive(Debug, Deserialize)]
struct Settings {
    storage: StorageSettings,
    #[serde(default = "default_comandos")] // Si falta `comandos` en el TOML, usá esta función
    comandos: HashMap<String, String>,
    #[serde(default = "default_explicaciones")] // Si falta `explicaciones`, usá esta
    explicaciones: HashMap<String, String>,
}

enum Storage {
    Csv(CsvStorage),
    Json(JsonStorage),
    Txt(TxtStorage),
    Bincode(BincodeStorage),
}

impl Storage {
    fn save(&self, file_path: &str, tasks: &Vec<Tarea>) -> Result<(), Box<dyn Error>> {
        match self {
            Storage::Csv(s) => s.save(file_path, tasks),
            Storage::Json(j) => j.save(file_path, tasks),
            Storage::Txt(t) => t.save(file_path, tasks),
            Storage::Bincode(b) => b.save(file_path, tasks), 
        }
    }
    fn load(&self, file_path: &str) -> Result<Vec<Tarea>, Box<dyn Error>> {
        match self {
            Storage::Csv(s) => s.load(file_path),
            Storage::Json(j) => j.load(file_path),
            Storage::Txt(t) => t.load(file_path),
            Storage::Bincode(b) => b.load(file_path),
        }
    }
}

// --- Implementazione per Bincode ---
struct BincodeStorage;
impl BincodeStorage {
    fn save(&self, file_path: &str, tasks: &Vec<Tarea>) -> Result<(), Box<dyn Error>> {
        // 1. Definisci la configurazione (standard va benissimo)
        let config = bincode::config::standard();
        let file = fs::File::create(file_path)?;
        // 2. Chiama il nuovo metodo `encode_into_std_write`
        bincode::serde::encode_into_std_write(tasks, &mut std::io::BufWriter::new(file), config)?;
        Ok(())
    }

    fn load(&self, file_path: &str) -> Result<Vec<Tarea>, Box<dyn Error>> {
        if !fs::metadata(file_path).is_ok() { return Ok(Vec::new()); }
        
        // 1. Definisci la configurazione (deve corrispondere a quella di salvataggio)
        let config = bincode::config::standard();
        let file = fs::File::open(file_path)?;
        // 2. Chiama il nuovo metodo `decode_from_std_read`
        let tasks = bincode::serde::decode_from_std_read(&mut std::io::BufReader::new(file), config)?;
        Ok(tasks)
    }
}

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
                tasks.push(Tarea {
                    descripcion: description.to_string(),
                    completada: status_str == "completada",
                });
            }
        }
        Ok(tasks)
    }
}

struct CsvStorage;
impl CsvStorage {
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
        .set_default("storage.filename_base", "tareas")?
        .set_default("storage.format", "txt")?
        .add_source(File::with_name("Settings.toml").required(false));

    let settings: Settings = builder.build()?.try_deserialize()?;
    
    // CORRECCIÓN: Usamos `settings.storage` para acceder a los campos anidados
    let filename = format!("{}.{}", settings.storage.filename_base, settings.storage.format);
    println!("Usando formato: '{}', Archivo: '{}'", settings.storage.format, filename);

    let storage: Storage = match settings.storage.format.as_str() {
        "csv" => Storage::Csv(CsvStorage),
        "json" => Storage::Json(JsonStorage),
        "txt" => Storage::Txt(TxtStorage),
        "bincode" => Storage::Bincode(BincodeStorage),
        _ => {
            eprintln!("Error: Formato no soportado '{}' en Settings.toml.", settings.storage.format);
            Storage::Txt(TxtStorage)
        }
    };

    let mut tareas = storage.load(&filename)?;
    
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        // Modo para Tests
        let comando_usuario = &args[1];
        if let Some(comando_interno) = settings.comandos.get(comando_usuario) {
            let resto_args = &args[2..].join(" ");
            // CORRECCIÓN: Pasamos `&settings` a la función
            procesar_comando(comando_interno, resto_args, &mut tareas, &settings);
            storage.save(&filename, &tareas)?;
        } else {
             println!("❌ Comando '{}' no reconocido.", comando_usuario);
        }
    } else {
        // --- Modo Interactivo ---
        println!("Bienvenido al gestor de tareas.");

        // --- CONSTRUCCIÓN DEL PROMPT DINÁMICO ---
        // 1. Tomamos los alias de la configuración y los metemos en un vector.
        let mut comandos_disponibles: Vec<String> = settings.comandos.keys().cloned().collect();
        // 2. Agregamos el comando 'salir', que se maneja por separado.
        comandos_disponibles.push("salir".to_string());
        // 3. Los ordenamos para que el prompt sea siempre igual.
        comandos_disponibles.sort();
        // 4. Los unimos en un solo string para mostrarlo.
        let prompt_string = comandos_disponibles.join("', '");

        loop {
            // 5. Usamos el string que acabamos de armar en el print.
            println!("\nIngresá un comando ('{}')", prompt_string);
            
            let mut entrada = String::new();
            io::stdin().read_line(&mut entrada).expect("Error al leer la entrada");
            
            let entrada = entrada.trim();
            let mut parts = entrada.split_whitespace();
            
            if let Some(comando_usuario) = parts.next() {
                // El comando "salir" lo manejamos acá directamente
                if comando_usuario == "salir" {
                    println!("\nSaliendo del gestor de tareas.");
                    break;
                }
                
                // Para todos los demás comandos, buscamos en la configuración
                if let Some(comando_interno) = settings.comandos.get(comando_usuario) {
                    let resto_args = parts.collect::<Vec<&str>>().join(" ");
                    let hubo_cambios = procesar_comando(comando_interno, &resto_args, &mut tareas, &settings);
                    if hubo_cambios {
                        if let Err(e) = storage.save(&filename, &tareas) {
                            eprintln!("Error al guardar tareas: {}", e);
                        }
                    }
                } else {
                    println!("❌ Comando '{}' no reconocido.", comando_usuario);
                }
            }
        }
    }
    Ok(())
}

fn cargar_desde_archivo(path: &str) -> Result<Vec<Tarea>, Box<dyn Error>> {
    let extension = std::path::Path::new(path)
        .extension()
        .and_then(std::ffi::OsStr::to_str)
        .unwrap_or("");

    let storage: Storage = match extension {
        "csv" => Storage::Csv(CsvStorage),
        "json" => Storage::Json(JsonStorage),
        "txt" => Storage::Txt(TxtStorage),
        "bincode" => Storage::Bincode(BincodeStorage),
        _ => return Err("Formato de archivo no soportado. Usá .csv, .json, o .txt.".into()),
    };
    storage.load(path)
}

fn procesar_comando(comando_interno: &str, args: &str, tareas: &mut Vec<Tarea>, settings: &Settings) -> bool {
    let mut cambio_realizado = false;
    match comando_interno {
        "agregar" => {
            if !args.is_empty() {
                tareas.push(Tarea {
                    descripcion: args.to_string(),
                    completada: false,
                });
                println!("✅ Tarea agregada.");
                cambio_realizado = true;
            } else {
                println!("❌ La descripción de la tarea no puede estar vacía.");
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
                        tareas[id - 1].completada = true;
                        println!("✅ Tarea {} marcada como completada.", id);
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
            if !args.is_empty() {
                if let Some(explicacion) = settings.explicaciones.get(args) {
                    println!("\nAyuda para '{}':\n{}", args, explicacion);
                } else {
                    println!("No encontré ayuda para el comando '{}'.", args);
                }
            } else {
                println!("\nComandos disponibles:");
                for (alias, _) in &settings.comandos {
                    println!("- {}", alias);
                }
                println!("\nEscribí 'ayuda <comando>' para más detalles.");
            }
        }
        _ => {
            println!("❌ Error interno: Comando '{}' no reconocido.", comando_interno);
        }
    }
    cambio_realizado
}

fn listar_tareas(lista_de_tareas: &Vec<Tarea>) {
    println!("\nLista de Tareas:");
    for (i, tarea) in lista_de_tareas.iter().enumerate() {
        tarea.mostrar(i + 1);
    }
}