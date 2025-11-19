use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct StorageSettings {
    pub filename_base: String,
    pub format: String,
}

#[derive(Debug, Deserialize)]
pub struct GeneralSettings {
    #[serde(default = "default_tags")]
    pub tags_validos: Vec<String>,
}

fn default_general() -> GeneralSettings {
    GeneralSettings {
        tags_validos: default_tags(),
    }
}

fn default_tags() -> Vec<String> {
    vec!["trabajo".to_string(), "personal".to_string()]
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
        ("eliminar".to_string(), "eliminar".to_string()),
        ("eliminar_todas".to_string(), "eliminar_todas".to_string()),
        ("etiquetar".to_string(), "etiquetar".to_string()),
        ("desetiquetar".to_string(), "desetiquetar".to_string()),
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
        ("eliminar".to_string(), "Uso: eliminar <número>. Elimina una tarea de la lista.".to_string()),
        ("eliminar_todas".to_string(), "Uso: eliminar_todas. Elimina todas las tareas.".to_string()),
        ("etiquetar".to_string(), "Uso: etiquetar <número> <tag>. Agrega una etiqueta a una tarea.".to_string()),
        ("desetiquetar".to_string(), "Uso: desetiquetar <número> <tag>. Elimina una etiqueta de una tarea.".to_string()),
    ])
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub storage: StorageSettings,
    #[serde(default = "default_general")]
    pub general: GeneralSettings,
    #[serde(default = "default_comandos")]
    pub comandos: HashMap<String, String>,
    #[serde(default = "default_explicaciones")]
    pub explicaciones: HashMap<String, String>,
}

impl Default for GeneralSettings {
    fn default() -> Self {
        Self {
            tags_validos: default_tags(),
        }
    }
}
impl Default for StorageSettings {
    fn default() -> Self {
        Self {
            filename_base: "tareas".to_string(),
            format: "txt".to_string(),
        }
    }
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            storage: StorageSettings::default(),
            general: GeneralSettings::default(),
            comandos: default_comandos(),
            explicaciones: default_explicaciones(),
        }
    }
}