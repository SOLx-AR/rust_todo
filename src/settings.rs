// src/settings.rs

use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct StorageSettings {
    pub filename_base: String,
    pub format: String,
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
        ("agregar".to_string(), "Uso: agregar <descripción>. Agrega una nueva tarea.".to_string()),
        ("listar".to_string(), "Uso: listar. Muestra todas las tareas.".to_string()),
        ("completar".to_string(), "Uso: completar <número>. Marca una tarea como completada.".to_string()),
        ("desmarcar".to_string(), "Uso: desmarcar <número>. Vuelve a marcar una tarea como pendiente.".to_string()),
        ("importar".to_string(), "Uso: importar <archivo>. Agrega tareas de un archivo a la lista actual.".to_string()),
        ("cargar".to_string(), "Uso: cargar <archivo>. Reemplaza la lista actual con las tareas del archivo.".to_string()),
        ("explicar".to_string(), "Uso: explicar <comando>. Muestra la ayuda para un comando.".to_string()),
    ])
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub storage: StorageSettings,
    #[serde(default = "default_comandos")]
    pub comandos: HashMap<String, String>,
    #[serde(default = "default_explicaciones")]
    pub explicaciones: HashMap<String, String>,
}