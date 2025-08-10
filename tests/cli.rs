use assert_cmd::prelude::*;
use predicates::prelude::*;
use serde::Deserialize;
use std::error::Error;
use std::fs;
use std::process::Command;

// Esta struct coincide con el formato actualizado de Settings.toml
#[derive(Deserialize)]
struct TestSettings {
    filename_base: String,
    format: String,
}

fn setup() {
    // Leemos el Settings.toml para armar el nombre completo del archivo y borrarlo
    if let Ok(contents) = fs::read_to_string("Settings.toml") {
        if let Ok(settings) = toml::from_str::<TestSettings>(&contents) {
            // Armamos el nombre completo del archivo, igual que en main.rs
            let filename = format!("{}.{}", settings.filename_base, settings.format);
            fs::remove_file(filename).ok();
            return; // Salimos después de borrarlo con éxito
        }
    }
    
    // Limpieza de respaldo por si no podemos leer el Settings.toml
    fs::remove_file("tareas.csv").ok();
    fs::remove_file("tareas.json").ok();
    fs::remove_file("tareas.txt").ok();
}

#[test]
fn test_configuracion_default_se_il_file_manca() -> Result<(), Box<dyn Error>> {
    // Escondemos temporalmente el archivo de configuración real
    fs::rename("Settings.toml", "Settings.toml.bak").ok();
    // Nos aseguramos de que el archivo default no exista
    fs::remove_file("tareas.txt").ok();

    // Corremos la aplicación
    Command::cargo_bin("todo")?
        .arg("listar")
        .assert()
        .success();

    // Chequeamos que el archivo default ("tareas.txt") se haya creado
    let default_file_exists = fs::metadata("tareas.txt").is_ok();
    
    // Limpieza: restauramos la config y borramos el archivo creado
    fs::rename("Settings.toml.bak", "Settings.toml").ok();
    fs::remove_file("tareas.txt").ok();

    assert!(default_file_exists, "El archivo default 'tareas.txt' no se creó");

    Ok(())
}


#[test]
fn test_flujo_feliz() -> Result<(), Box<dyn Error>> {
    setup();

    Command::cargo_bin("todo")?
        .arg("agregar")
        .arg("Comprar pan")
        .assert()
        .success();

    Command::cargo_bin("todo")?
        .arg("agregar")
        .arg("Pasear al perro")
        .assert()
        .success();

    Command::cargo_bin("todo")?
        .arg("listar")
        .assert()
        .success()
        .stdout(predicate::str::contains("[ ] 1: Comprar pan"))
        .stdout(predicate::str::contains("[ ] 2: Pasear al perro"));

    Command::cargo_bin("todo")?
        .arg("completar")
        .arg("1")
        .assert()
        .success();

    Command::cargo_bin("todo")?
        .arg("listar")
        .assert()
        .success()
        .stdout(predicate::str::contains("[X] 1: Comprar pan"))
        .stdout(predicate::str::contains("[ ] 2: Pasear al perro"));

    Ok(())
}

#[test]
fn test_agregar_sin_descripcion() -> Result<(), Box<dyn Error>> {
    setup();
    Command::cargo_bin("todo")?
        .arg("agregar")
        .assert()
        .success()
        .stdout(predicate::str::contains("La descripción de la tarea no puede estar vacía."));
    Ok(())
}

#[test]
fn test_completar_id_invalido() -> Result<(), Box<dyn Error>> {
    setup();
    Command::cargo_bin("todo")?
        .arg("agregar")
        .arg("Mi tarea")
        .assert()
        .success();

    Command::cargo_bin("todo")?
        .arg("completar")
        .arg("abc")
        .assert()
        .success()
        .stdout(predicate::str::contains("ID de tarea no válido."));

    Command::cargo_bin("todo")?
        .arg("completar")
        .arg("99")
        .assert()
        .success()
        .stdout(predicate::str::contains("ID de tarea no válido."));
    Ok(())
}

#[test]
fn test_comando_desconocido() -> Result<(), Box<dyn Error>> {
    setup();
    Command::cargo_bin("todo")?
        .arg("comando-que-no-existe")
        .assert()
        .success()
        .stdout(predicate::str::contains("Comando no reconocido."));
    Ok(())
}