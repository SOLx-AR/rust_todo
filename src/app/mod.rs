pub mod commands;
pub mod task;
pub mod ui; 

use crate::settings::Settings;
use crate::storage::{Storage, CsvStorage, JsonStorage, TxtStorage, BincodeStorage};
use commands::procesar_comando;
use config::{Config, File};
use std::env;
use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
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
        _ => Storage::Txt(TxtStorage),
    };

    let mut tareas = storage.load(&filename)?;
    
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        // --- Modo para Tests (Corregido) ---
        let comando_usuario = &args[1];
        if let Some(comando_interno) = settings.comandos.get(comando_usuario) {
            let resto_args = &args[2..].join(" ");

            // Ejecutamos el comando
            procesar_comando(comando_interno, resto_args, &mut tareas, &settings);
            
            // CORRECCIÓN: En modo test, siempre guardamos el resultado final,
            // a menos que el comando sea específicamente para borrar el archivo.
            if comando_interno != "eliminar_todas" {
                storage.save(&filename, &tareas)?;
            }
        } else {
             println!("❌ Comando '{}' no reconocido.", comando_usuario);
        }
    } else {
        // --- Modo Interactivo (sin cambios) ---
        ui::iniciar_menu_interactivo(&mut tareas, &settings, &storage, &filename)?;
    }
    Ok(())
}