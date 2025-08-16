use crate::app::task::Tarea;
use crate::settings::Settings;
use crate::storage::{Storage, BincodeStorage, CsvStorage, JsonStorage, TxtStorage};
use std::error::Error;
use std::fs;

pub mod agregar;
pub mod cargar;
pub mod completar;
pub mod desmarcar;
pub mod eliminar;
pub mod eliminar_todas;
pub mod explicar;
pub mod importar;
pub mod listar;
pub mod etiquetar;
pub mod desetiquetar;

pub fn procesar_comando(comando_interno: &str, args: &str, tareas: &mut Vec<Tarea>, settings: &Settings) -> bool {
    match comando_interno {
        "agregar" => agregar::ejecutar(args, tareas),
        "cargar" => cargar::ejecutar(args, tareas),
        "completar" => completar::ejecutar(args, tareas),
        "desmarcar" => desmarcar::ejecutar(args, tareas),
        "eliminar" => eliminar::ejecutar(args, tareas),
        "eliminar_todas" => eliminar_todas::ejecutar(args, tareas, settings),
        "importar" => importar::ejecutar(args, tareas),
        "listar" => listar::ejecutar(args, tareas),
        "explicar" => explicar::ejecutar(args, settings),
        "etiquetar" => etiquetar::ejecutar(args, tareas, settings),
        "desetiquetar" => desetiquetar::ejecutar(args, tareas),
        _ => {
            println!("❌ Error interno: Comando '{}' no reconocido.", comando_interno);
            false
        }
    }
}

pub fn cargar_desde_archivo(path: &str) -> Result<Vec<Tarea>, Box<dyn Error>> {
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

pub fn listar_tareas(lista_de_tareas: &Vec<Tarea>) {
    println!("\nLista de Tareas:");
    for (i, tarea) in lista_de_tareas.iter().enumerate() {
        tarea.mostrar(i + 1);
    }
}