// src/app/mod.rs

// 1. Declara los submódulos de esta carpeta
pub mod commands;
pub mod task;

// 2. Importa todo lo que necesita la función `run`
use crate::settings::Settings;
use crate::storage::{Storage, CsvStorage, JsonStorage, TxtStorage, BincodeStorage};
use commands::procesar_comando;
use config::{Config, File};
use dialoguer::{theme::ColorfulTheme, Select, Input};
use std::collections::HashSet;
use std::env;
use std::error::Error;

// 3. La función `run` ahora vive acá
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
        // Modo Interactivo
        println!("Bienvenido al gestor de tareas.");
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