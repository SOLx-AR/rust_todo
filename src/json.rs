//! ## Funciones de persistencia de datos.
//! 
//! Este módulo serializa y deserializa mediante la biblioteca **serde_json** para guardar y recuperar las tareas, etiquetas y estadísticas en archivos JSON.

use crate::data::*;
use std::fs;

/// ### Guarda las tareas. 
/// 
/// Las tareas se guardan en el archivo **tasks.json**.
///
/// # Arguments
///
/// * `tasks` - Una referencia a un vector de `Task` a guardar.
pub fn save_tasks(tasks: &Vec<Task>) {
    let serialized = serde_json::to_string_pretty(tasks).expect("Error al serializar las tareas.");
    fs::write("tasks.json", serialized).expect("Error al escribir el archivo.");
}

/// ### Recupera las tareas. 
/// 
/// Las tareas se recuperan desde el archivo **tasks.json** en un vector de structs. Si no existen datos o hay un error de deserialización, retorna un vector vacío.
pub fn load_tasks() -> Vec<Task> {
    match fs::read_to_string("tasks.json") {
        Ok(data) => serde_json::from_str(&data).unwrap_or_else(|_| {
            Vec::new()
        }),
        Err(_) => {
            Vec::new()
        }
    }
}

/// ### Guarda las etiquetas. 
/// 
/// Las etiquetas se guardan en el archivo **tags.json**.
pub fn save_tags(tags: &Vec<String>) {
    let serialized = serde_json::to_string_pretty(tags).expect("Error al serializar los tags.");
    fs::write("tags.json", serialized).expect("Error al escribir el archivo.");
}

/// ### Recupera las etiquetas. 
/// 
/// Retorna un vector de Strings con los datos cargados o un vector vacío si el archivo **tags.json** no existe, o si hay un error de deserialización.
pub fn load_tags() -> Vec<String> {
    match fs::read_to_string("tags.json") {
        Ok(data) => serde_json::from_str(&data).unwrap_or_else(|_| {
            Vec::new()
        }),
        Err(_) => {
            Vec::new()
        }
    }
}

/// ### Guarda las estadísticas 
/// 
/// Las estadísticas de uso se guardan en el archivo **stats.json**.
pub fn save_stats(stats: &Statistics) {
    let serialized = serde_json::to_string_pretty(stats).expect("Error al serializar las estadísticas.");
    fs::write("stats.json", serialized).expect("Error al escribir el archivo.");
}

/// ### Recupera las estadísticas
/// 
/// Retorna un struct **Statistics** con los datos cargados o una instancia por defecto si el archivo **stats.json** no existe, o si hay un error de deserialización.
pub fn load_stats() -> Statistics {
    match fs::read_to_string("stats.json") {
        Ok(data) => serde_json::from_str(&data).unwrap_or_else(|_| {
            Statistics::default()
        }),
        Err(_) => {
            Statistics::default()
        }
    }
}
