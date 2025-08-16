use super::commands;
use super::task::Tarea;
use crate::settings::Settings;
use crate::storage::Storage;
use console::Term;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use std::collections::HashSet;
use std::error::Error;

pub fn iniciar_menu_interactivo(
    tareas: &mut Vec<Tarea>,
    settings: &Settings,
    storage: &Storage,
    filename: &str,
) -> Result<(), Box<dyn Error>> {
    Term::stdout().clear_screen()?;
    println!("Bienvenido al gestor de tareas.");

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

            let mut continuar_loop = true;

            match comando_interno.map(|s| s.as_str()) {
                Some("explicar") => manejar_ayuda(tareas, settings)?,
                Some("etiquetar") => manejar_etiquetado(tareas, settings, storage, filename)?,
                Some("desetiquetar") => manejar_desetiquetado(tareas, settings, storage, filename)?,
                Some(cmd_int) => {
                    manejar_comando_estandar(cmd_int, eleccion_usuario, tareas, settings, storage, filename)?;
                }
                None => {
                    if eleccion_usuario == "salir" {
                        continuar_loop = false;
                    }
                }
            }

            if !continuar_loop {
                break;
            }

        } else {
            println!("\nSelección cancelada. Saliendo del gestor de tareas.");
            break;
        }
    }
    Ok(())
}

fn manejar_ayuda(tareas: &mut Vec<Tarea>, settings: &Settings) -> Result<(), Box<dyn Error>> {
    let mut topicos_de_ayuda: Vec<String> = settings.explicaciones.keys().cloned().collect();
    topicos_de_ayuda.sort();
    
    if let Some(indice) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("¿De qué comando necesitás ayuda?")
        .items(&topicos_de_ayuda)
        .interact_opt()? 
    {
        let topico_elegido = &topicos_de_ayuda[indice];
        commands::procesar_comando("explicar", topico_elegido, tareas, settings);
    }
    Ok(())
}

fn manejar_etiquetado(
    tareas: &mut Vec<Tarea>,
    settings: &Settings,
    storage: &Storage,
    filename: &str,
) -> Result<(), Box<dyn Error>> {
    if tareas.is_empty() {
        println!("\nNo hay tareas para etiquetar. ¡Agregá una primero!");
        return Ok(());
    }
    let descripciones_tareas: Vec<String> = tareas.iter().map(|t| t.descripcion.clone()).collect();
    if let Some(indice_tarea) = Select::with_theme(&ColorfulTheme::default()).with_prompt("¿Qué tarea querés etiquetar?").items(&descripciones_tareas).interact_opt()? {
        if let Some(indice_tag) = Select::with_theme(&ColorfulTheme::default()).with_prompt("¿Qué etiqueta querés agregar?").items(&settings.general.tags_validos).interact_opt()? {
            let tag_elegido = &settings.general.tags_validos[indice_tag];
            let args_etiquetar = format!("{} {}", indice_tarea + 1, tag_elegido);
            if commands::procesar_comando("etiquetar", &args_etiquetar, tareas, settings) {
                storage.save(filename, tareas)?;
            }
        }
    }
    Ok(())
}

fn manejar_desetiquetado(
    tareas: &mut Vec<Tarea>,
    settings: &Settings,
    storage: &Storage,
    filename: &str,
) -> Result<(), Box<dyn Error>> {
    if tareas.is_empty() {
        println!("\nNo hay tareas para desetiquetar.");
        return Ok(());
    }
    let descripciones_tareas: Vec<String> = tareas.iter().map(|t| t.descripcion.clone()).collect();
    if let Some(indice_tarea) = Select::with_theme(&ColorfulTheme::default()).with_prompt("¿A qué tarea le querés sacar una etiqueta?").items(&descripciones_tareas).interact_opt()? {
        if tareas[indice_tarea].tags.is_empty() {
            println!("\nEsa tarea no tiene ninguna etiqueta para eliminar.");
            return Ok(());
        }
        if let Some(indice_tag) = Select::with_theme(&ColorfulTheme::default()).with_prompt("¿Qué etiqueta querés eliminar?").items(&tareas[indice_tarea].tags).interact_opt()? {
            let tag_elegido = &tareas[indice_tarea].tags[indice_tag].clone();
            let args_desetiquetar = format!("{} {}", indice_tarea + 1, tag_elegido);
            if commands::procesar_comando("desetiquetar", &args_desetiquetar, tareas, settings) {
                storage.save(filename, tareas)?;
            }
        }
    }
    Ok(())
}

fn manejar_comando_estandar(
    cmd_int: &str,
    eleccion_usuario: &str,
    tareas: &mut Vec<Tarea>,
    settings: &Settings,
    storage: &Storage,
    filename: &str,
) -> Result<(), Box<dyn Error>> {
    let comandos_con_parametro: HashSet<String> = [
        "agregar".to_string(), "completar".to_string(), "desmarcar".to_string(), 
        "importar".to_string(), "cargar".to_string(), "eliminar".to_string()
    ].iter().cloned().collect();

    let mut args = String::new();
    if comandos_con_parametro.contains(cmd_int) {
        args = Input::with_theme(&ColorfulTheme::default())
            .with_prompt(format!("Ingresá el parámetro para '{}'", eleccion_usuario))
            .interact_text()?;
    }
    
    let hubo_cambios = commands::procesar_comando(cmd_int, &args, tareas, settings);
    if hubo_cambios {
        storage.save(filename, tareas)?;
    }
    Ok(())
}