use config::{Config, File};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::env;
use std::error::Error;
use std::fs;
use dialoguer::{theme::ColorfulTheme, Select, Input};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Tarea {
    descripcion: String,
    completada: bool,
    ultimo_cambio: Option<DateTime<Utc>>, 
}

impl Tarea {
    fn mostrar(&self, id: usize) {
        let estado = if self.completada { "[X]" } else { "[ ]" };
        let timestamp_str = if self.completada {
            self.ultimo_cambio.map_or("".to_string(), |ts| format!("({}) ", ts.format("%d/%m/%Y %H:%M:%S")))
        } else {
            "".to_string()
        };
        println!("{} {}{}: {}", estado, timestamp_str, id, self.descripcion);
    }
}

#[derive(Debug, Deserialize)]
struct StorageSettings {
    filename_base: String,
    format: String,
}

fn default_comandos() -> HashMap<String, String> {
    HashMap::from([
        ("agregar".to_string(), "agregar".to_string()),
        ("listar".to_string(), "listar".to_string()),
        ("completar".to_string(), "completar".to_string()),
        ("desmarcar".to_string(), "desmarcar".to_string()),
        ("importar".to_string(), "importar".to_string()),
        ("cargar".to_string(), "cargar".to_string()),
        ("ayuda".to_string(), "explicar".to_string()),
    ])
}

fn default_explicaciones() -> HashMap<String, String> {
    HashMap::from([
        ("agregar".to_string(), "Uso: agregar <descripción>. Agrega una nueva tarea a la lista.".to_string()),
        ("listar".to_string(), "Uso: listar. Muestra todas las tareas.".to_string()),
        ("completar".to_string(), "Uso: completar <número de tarea>. Marca una tarea como completada.".to_string()),
        ("desmarcar".to_string(), "Uso: desmarcar <número de tarea>. Vuelve a marcar una tarea como pendiente.".to_string()),
        ("importar".to_string(), "Uso: importar <ruta_al_archivo>. Agrega tareas de un archivo a la lista actual.".to_string()),
        ("cargar".to_string(), "Uso: cargar <ruta_al_archivo>. Reemplaza la lista actual con las tareas de un archivo.".to_string()),
        ("explicar".to_string(), "Uso: explicar <comando>. Muestra la ayuda para un comando específico.".to_string()),
    ])
}

#[derive(Debug, Deserialize)]
struct Settings {
    storage: StorageSettings,
    #[serde(default = "default_comandos")]
    comandos: HashMap<String, String>,
    #[serde(default = "default_explicaciones")]
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

struct BincodeStorage;
impl BincodeStorage {
    fn save(&self, file_path: &str, tasks: &Vec<Tarea>) -> Result<(), Box<dyn Error>> {
        let config = bincode::config::standard();
        let file = fs::File::create(file_path)?;
        bincode::serde::encode_into_std_write(tasks, &mut std::io::BufWriter::new(file), config)?;
        Ok(())
    }
    fn load(&self, file_path: &str) -> Result<Vec<Tarea>, Box<dyn Error>> {
        if !fs::metadata(file_path).is_ok() { return Ok(Vec::new()); }
        let config = bincode::config::standard();
        let file = fs::File::open(file_path)?;
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
            let timestamp_str = task.ultimo_cambio.map_or(String::new(), |ts| ts.to_rfc3339());
            file_content.push_str(&format!("{},{},{}\n", status, timestamp_str, task.descripcion));
        }
        Ok(fs::write(file_path, file_content)?)
    }
    fn load(&self, file_path: &str) -> Result<Vec<Tarea>, Box<dyn Error>> {
        if !fs::metadata(file_path).is_ok() { return Ok(Vec::new()); }
        let file_content = fs::read_to_string(file_path)?;
        let mut tasks = Vec::new();
        for line in file_content.lines() {
            if let Some((status_str, rest)) = line.split_once(',') {
                if let Some((timestamp_str, description)) = rest.split_once(',') {
                    let timestamp_opt = if timestamp_str.is_empty() {
                        None
                    } else {
                        DateTime::parse_from_rfc3339(timestamp_str).ok().map(|dt| dt.with_timezone(&Utc))
                    };
                    tasks.push(Tarea {
                        descripcion: description.to_string(),
                        completada: status_str == "completada",
                        ultimo_cambio: timestamp_opt,
                    });
                }
            }
        }
        Ok(tasks)
    }
}

struct CsvStorage;
impl CsvStorage {
    fn save(&self, file_path: &str, tasks: &Vec<Tarea>) -> Result<(), Box<dyn Error>> {
        let mut writer = csv::Writer::from_path(file_path)?;
        writer.write_record(&["completada", "ultimo_cambio", "descripcion"])?;
        for task in tasks {
            let timestamp_str = task.ultimo_cambio.map_or(String::new(), |ts| ts.to_rfc3339());
            writer.write_record(&[
                task.completada.to_string(),
                timestamp_str,
                task.descripcion.clone(),
            ])?;
        }
        Ok(writer.flush()?)
    }
    fn load(&self, file_path: &str) -> Result<Vec<Tarea>, Box<dyn Error>> {
        if !fs::metadata(file_path).is_ok() { return Ok(Vec::new()); }
        let file = fs::File::open(file_path)?;
        let mut reader = csv::ReaderBuilder::new().has_headers(true).from_reader(file);
        let mut tasks = Vec::new();
        for result in reader.records() {
            let record = result?;
            let timestamp_opt = record.get(1)
                .and_then(|ts_str| DateTime::parse_from_rfc3339(ts_str).ok())
                .map(|dt| dt.with_timezone(&Utc));
            tasks.push(Tarea {
                completada: record.get(0).unwrap_or("").parse().unwrap_or(false),
                ultimo_cambio: timestamp_opt,
                descripcion: record.get(2).unwrap_or("").to_string(),
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
            procesar_comando(comando_interno, resto_args, &mut tareas, &settings);
            storage.save(&filename, &tareas)?;
        } else {
             println!("❌ Comando '{}' no reconocido.", comando_usuario);
        }
    } else {
        // --- MODO INTERACTIVO CON SUBMENÚ DE AYUDA ---
        println!("Bienvenido al gestor de tareas.");

        // Sacamos "explicar" de acá porque lo manejaremos de forma especial
        let comandos_con_parametro: HashSet<String> = [
            "agregar".to_string(), "completar".to_string(), "desmarcar".to_string(), "importar".to_string(), "cargar".to_string()
        ].iter().cloned().collect();

        let mut opciones_menu: Vec<String> = settings.comandos.keys().cloned().collect();
        opciones_menu.sort();
        opciones_menu.push("salir".to_string());

        loop {
            let seleccion = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("¿Qué querés hacer?")
                .items(&opciones_menu)
                .default(0)
                .interact_opt()?;

            if let Some(indice_seleccionado) = seleccion {
                let eleccion_usuario = &opciones_menu[indice_seleccionado];
                
                if eleccion_usuario == "salir" {
                    println!("\nSaliendo del gestor de tareas.");
                    break;
                }
                
                let comando_interno = settings.comandos.get(eleccion_usuario.as_str());

                // --- LÓGICA DEL SUBMENÚ ---
                if let Some("explicar") = comando_interno.map(|s| s.as_str()) {
                    let mut topicos_de_ayuda: Vec<String> = settings.explicaciones.keys().cloned().collect();
                    topicos_de_ayuda.sort();

                    let seleccion_ayuda = Select::with_theme(&ColorfulTheme::default())
                        .with_prompt("¿De qué comando necesitás ayuda?")
                        .items(&topicos_de_ayuda)
                        .interact_opt()?;

                    if let Some(indice) = seleccion_ayuda {
                        let topico_elegido = &topicos_de_ayuda[indice];
                        procesar_comando("explicar", topico_elegido, &mut tareas, &settings);
                    }
                    
                    continue; 
                }
                
                // --- Lógica para todos los demás comandos ---
                if let Some(cmd_int) = comando_interno {
                    let mut args = String::new();
                    if comandos_con_parametro.contains(cmd_int) {
                        args = Input::with_theme(&ColorfulTheme::default())
                            .with_prompt(format!("Ingresá el parámetro para '{}'", eleccion_usuario))
                            .interact_text()?;
                    }
                    let hubo_cambios = procesar_comando(cmd_int, &args, &mut tareas, &settings);
                    if hubo_cambios {
                        if let Err(e) = storage.save(&filename, &tareas) {
                            eprintln!("Error al guardar tareas: {}", e);
                        }
                    }
                }
            } else {
                println!("\nSelección cancelada. Saliendo del gestor de tareas.");
                break;
            }
        }
    }
    Ok(())
}

fn cargar_desde_archivo(path: &str) -> Result<Vec<Tarea>, Box<dyn Error>> {
    if !fs::metadata(path).is_ok() {
        return Err(format!("No se pudo encontrar el archivo en la ruta '{}'", path).into());
    }
    let extension = std::path::Path::new(path)
        .extension()
        .and_then(std::ffi::OsStr::to_str)
        .unwrap_or("");

    let storage: Storage = match extension {
        "csv" => Storage::Csv(CsvStorage),
        "json" => Storage::Json(JsonStorage),
        "txt" => Storage::Txt(TxtStorage),
        "bincode" => Storage::Bincode(BincodeStorage),
        _ => return Err("Formato de archivo no soportado. Usá .csv, .json, .txt, o .bincode.".into()),
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
                        tarea.ultimo_cambio = Some(Utc::now());
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
                        tarea.ultimo_cambio = Some(Utc::now());
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
            // Lógica simplificada: ahora solo muestra la ayuda para un comando específico.
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

fn listar_tareas(lista_de_tareas: &Vec<Tarea>) {
    println!("\nLista de Tareas:");
    for (i, tarea) in lista_de_tareas.iter().enumerate() {
        tarea.mostrar(i + 1);
    }
}

// Los tests unitarios los borramos porque ahora la lógica está en módulos
// y los tests de integración cubren el comportamiento.
// Podemos volver a agregarlos cuando hagamos el refactor final a módulos.