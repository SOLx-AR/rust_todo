use assert_cmd::prelude::*;
use predicates::prelude::*;
use serde::Deserialize;
use std::error::Error;
use std::fs;
use std::process::Command;

#[derive(Deserialize)]
pub struct TestStorageSettings {
    pub filename_base: String,
    pub format: String,
}
#[derive(Deserialize)]
pub struct TestSettings {
    pub storage: TestStorageSettings,
}

pub fn setup() {
    if std::path::Path::new("Settings.toml.bak").exists() {
        fs::rename("Settings.toml.bak", "Settings.toml").ok();
    }
    if let Ok(contents) = fs::read_to_string("Settings.toml") {
        if let Ok(settings) = toml::from_str::<TestSettings>(&contents) {
            let filename = format!("{}.{}", settings.storage.filename_base, settings.storage.format);
            fs::remove_file(filename).ok();
            return;
        }
    }
    fs::remove_file("tareas.csv").ok();
    fs::remove_file("tareas.json").ok();
    fs::remove_file("tareas.txt").ok();
    fs::remove_file("tareas.bincode").ok();
}

pub mod comandos_basicos;
pub mod etiquetado;
pub mod flujo_feliz;
pub mod manejo_de_archivos;