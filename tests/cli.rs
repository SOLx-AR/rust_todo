use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::error::Error;
use std::fs;
use std::process::Command;


fn setup() {
    fs::remove_file("tareas.csv").ok();
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