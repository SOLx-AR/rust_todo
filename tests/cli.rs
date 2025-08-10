use assert_cmd::prelude::*;
use predicates::prelude::*;
use serde::Deserialize;
use std::error::Error;
use std::fs;
use std::process::Command;

// CORRECCIÓN: La struct ahora es anidada para coincidir con Settings.toml
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
    // CORRECCIÓN: Primero, nos aseguramos de que el archivo de config esté en su lugar.
    if std::path::Path::new("Settings.toml.bak").exists() {
        fs::rename("Settings.toml.bak", "Settings.toml").ok();
    }

    // Leemos el Settings.toml para armar el nombre completo del archivo y borrarlo
    if let Ok(contents) = fs::read_to_string("Settings.toml") {
        if let Ok(settings) = toml::from_str::<TestSettings>(&contents) {
            // Usamos la estructura anidada para obtener los datos
            let filename = format!("{}.{}", settings.storage.filename_base, settings.storage.format);
            fs::remove_file(filename).ok();
            return; // Salimos después de borrarlo con éxito
        }
    }
    
    // Limpieza de respaldo por si no podemos leer el Settings.toml
    fs::remove_file("tareas.csv").ok();
    fs::remove_file("tareas.json").ok();
    fs::remove_file("tareas.txt").ok();
    fs::remove_file("tareas.bincode").ok(); // CORRECCIÓN: Agregamos bincode
}

#[test]
fn test_configuracion_default_se_il_file_manca() -> Result<(), Box<dyn Error>> {
    // Escondemos temporalmente el archivo de configuración real
    fs::rename("Settings.toml", "Settings.toml.bak").ok();
    // Nos aseguramos de que el archivo default no exista
    fs::remove_file("tareas.txt").ok();

    // Corremos la aplicación (este test usa "listar" porque es el comando por defecto)
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
        .arg("añadir")
        .arg("Comprar pan")
        .assert()
        .success();

    Command::cargo_bin("todo")?
        .arg("añadir")
        .arg("Pasear al perro")
        .assert()
        .success();

    Command::cargo_bin("todo")?
        .arg("listar") // CORRECCIÓN: Usamos el alias correcto
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
        .arg("listar") // CORRECCIÓN: Usamos el alias correcto
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
        .arg("añadir")
        .assert()
        .success()
        .stdout(predicate::str::contains("La descripción de la tarea no puede estar vacía."));
    Ok(())
}

#[test]
fn test_completar_id_invalido() -> Result<(), Box<dyn Error>> {
    setup();
    Command::cargo_bin("todo")?
        .arg("añadir")
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
        .stdout(predicate::str::contains("no reconocido"));
    Ok(())
}

#[test]
fn test_importar_tareas_y_fusionar() -> Result<(), Box<dyn Error>> {
    setup();
    
    Command::cargo_bin("todo")?
        .arg("añadir")
        .arg("Tarea Original")
        .assert()
        .success();

    let archivo_a_importar = "para_importar.json";
    let contenido_importar = r#"[{"descripcion":"Tarea Importada 1","completada":false},{"descripcion":"Tarea Importada 2","completada":true}]"#;
    fs::write(archivo_a_importar, contenido_importar)?;

    Command::cargo_bin("todo")?
        .arg("importar_y_sumar")
        .arg(archivo_a_importar)
        .assert()
        .success();

    Command::cargo_bin("todo")?
        .arg("listar") // CORRECCIÓN: Usamos el alias correcto
        .assert()
        .success()
        .stdout(predicate::str::contains("1: Tarea Original"))
        .stdout(predicate::str::contains("2: Tarea Importada 1"))
        .stdout(predicate::str::contains("3: Tarea Importada 2"));

    fs::remove_file(archivo_a_importar)?;
    Ok(())
}

#[test]
fn test_cargar_tareas_y_reemplazar() -> Result<(), Box<dyn Error>> {
    setup();

    Command::cargo_bin("todo")?
        .arg("añadir")
        .arg("Tarea Vieja que va a desaparecer")
        .assert()
        .success();

    let archivo_a_cargar = "para_cargar.txt";
    let contenido_cargar = "no completada,Tarea Nueva 1\ncompletada,Tarea Nueva 2";
    fs::write(archivo_a_cargar, contenido_cargar)?;

    Command::cargo_bin("todo")?
        .arg("reemplazar_con_archivo")
        .arg(archivo_a_cargar)
        .assert()
        .success();

    Command::cargo_bin("todo")?
        .arg("listar") // CORRECCIÓN: Usamos el alias correcto
        .assert()
        .success()
        .stdout(predicate::str::contains("1: Tarea Nueva 1"))
        .stdout(predicate::str::contains("2: Tarea Nueva 2"))
        .stdout(predicate::str::contains("Tarea Vieja").not());

    fs::remove_file(archivo_a_cargar)?;
    Ok(())
}