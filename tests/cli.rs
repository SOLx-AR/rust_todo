use assert_cmd::prelude::*;
use predicates::prelude::*;
use serde::Deserialize;
use std::error::Error;
use std::fs;
use std::process::Command;

#[derive(Deserialize)]
struct TestStorageSettings {
    filename_base: String,
    format: String,
}
#[derive(Deserialize)]
struct TestSettings {
    storage: TestStorageSettings,
}

fn setup() {
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

#[test]
fn test_configuracion_default_se_il_file_manca() -> Result<(), Box<dyn Error>> {
    fs::rename("Settings.toml", "Settings.toml.bak").ok();
    fs::remove_file("tareas.txt").ok();
    Command::cargo_bin("todo")?.arg("listar").assert().success();
    let default_file_exists = fs::metadata("tareas.txt").is_ok();
    fs::rename("Settings.toml.bak", "Settings.toml").ok();
    fs::remove_file("tareas.txt").ok();
    assert!(default_file_exists, "El archivo default 'tareas.txt' no se creó");
    Ok(())
}

#[test]
fn test_flujo_feliz() -> Result<(), Box<dyn Error>> {
    setup();
    Command::cargo_bin("todo")?.arg("añadir").arg("Comprar pan").assert().success();
    Command::cargo_bin("todo")?.arg("añadir").arg("Pasear al perro").assert().success();
    
    Command::cargo_bin("todo")?
        .arg("listar") // Usando tu comando
        .assert()
        .success()
        .stdout(predicate::str::contains("[ ] 1: Comprar pan\n"))
        .stdout(predicate::str::contains("[ ] 2: Pasear al perro"));

    Command::cargo_bin("todo")?.arg("completar").arg("1").assert().success(); // Usando tu comando

    let regex_tarea_completada = r"\[X\] \(\d{2}/\d{2}/\d{4} \d{2}:\d{2}:\d{2}\) 1: Comprar pan";
    Command::cargo_bin("todo")?
        .arg("listar") // Usando tu comando
        .assert()
        .success()
        .stdout(predicate::str::is_match(regex_tarea_completada)?)
        .stdout(predicate::str::contains("[ ] 2: Pasear al perro"));
    Ok(())
}

#[test]
fn test_desmarcar_tarea() -> Result<(), Box<dyn Error>> {
    setup();
    Command::cargo_bin("todo")?.arg("añadir").arg("Tarea para desmarcar").assert().success();
    Command::cargo_bin("todo")?.arg("completar").arg("1").assert().success();

    let regex_completada = r"\[X\] \(\d{2}/\d{2}/\d{4} \d{2}:\d{2}:\d{2}\) 1: Tarea para desmarcar";
    Command::cargo_bin("todo")?
        .arg("listar") // Usando tu comando
        .assert()
        .success()
        .stdout(predicate::str::is_match(regex_completada)?);

    Command::cargo_bin("todo")?.arg("desmarcar").arg("1").assert().success();

    Command::cargo_bin("todo")?
        .arg("listar") // Usando tu comando
        .assert()
        .success()
        .stdout(predicate::str::contains("[ ] 1: Tarea para desmarcar\n"));
    Ok(())
}

#[test]
fn test_agregar_sin_descripcion() -> Result<(), Box<dyn Error>> {
    setup();
    Command::cargo_bin("todo")?
        .arg("añadir")
        .assert()
        .success()
        .stdout(predicate::str::contains("Error: El comando para agregar una tarea necesita una descripción.")); // <-- Texto nuevo y correcto
    Ok(())
}

#[test]
fn test_completar_id_invalido() -> Result<(), Box<dyn Error>> {
    setup();
    Command::cargo_bin("todo")?.arg("añadir").arg("Mi tarea").assert().success();
    Command::cargo_bin("todo")?
        .arg("completar") // Usando tu comando
        .arg("abc")
        .assert()
        .success()
        .stdout(predicate::str::contains("ID de tarea no válido."));
    Command::cargo_bin("todo")?
        .arg("completar") // Usando tu comando
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
        .stdout(predicate::str::contains("no reconocido"));
    Ok(())
}

#[test]
fn test_importar_tareas_y_fusionar() -> Result<(), Box<dyn Error>> {
    setup();
    Command::cargo_bin("todo")?.arg("añadir").arg("Tarea Original").assert().success();
    let archivo_a_importar = "para_importar.json";
    let contenido_importar = r#"[{"descripcion":"Tarea Importada 1","completada":false, "ultimo_cambio": null}]"#;
    fs::write(archivo_a_importar, contenido_importar)?;
    Command::cargo_bin("todo")?
        .arg("importar_y_sumar")
        .arg(archivo_a_importar)
        .assert()
        .success();
    Command::cargo_bin("todo")?
        .arg("listar") // Usando tu comando
        .assert()
        .success()
        .stdout(predicate::str::contains("1: Tarea Original"))
        .stdout(predicate::str::contains("2: Tarea Importada 1"));
    fs::remove_file(archivo_a_importar)?;
    Ok(())
}

#[test]
fn test_cargar_tareas_y_reemplazar() -> Result<(), Box<dyn Error>> {
    setup();
    Command::cargo_bin("todo")?.arg("añadir").arg("Tarea Vieja").assert().success();
    let archivo_a_cargar = "para_cargar.txt";
    let contenido_cargar = "no completada,,Tarea Nueva 1\ncompletada,2025-01-01T11:00:00Z,Tarea Nueva 2";
    fs::write(archivo_a_cargar, contenido_cargar)?;
    Command::cargo_bin("todo")?
        .arg("reemplazar_con_archivo")
        .arg(archivo_a_cargar)
        .assert()
        .success();
    Command::cargo_bin("todo")?
        .arg("listar") // Usando tu comando
        .assert()
        .success()
        .stdout(predicate::str::contains("1: Tarea Nueva 1"))
        .stdout(predicate::str::contains("2: Tarea Nueva 2"))
        .stdout(predicate::str::contains("Tarea Vieja").not());
    fs::remove_file(archivo_a_cargar)?;
    Ok(())
}