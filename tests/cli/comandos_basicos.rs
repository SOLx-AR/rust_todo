use super::*;

#[test]
fn test_agregar_sin_descripcion() -> Result<(), Box<dyn Error>> {
    setup();
    Command::cargo_bin("todo")?
        .arg("añadir")
        .assert()
        .success()
        .stdout(predicate::str::contains("Error: El comando para agregar una tarea necesita una descripción."));
    Ok(())
}

#[test]
fn test_completar_id_invalido() -> Result<(), Box<dyn Error>> {
    setup();
    Command::cargo_bin("todo")?.arg("añadir").arg("Mi tarea").assert().success();
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
fn test_eliminar_tarea() -> Result<(), Box<dyn Error>> {
    setup();
    Command::cargo_bin("todo")?.arg("añadir").arg("Tarea 1").assert().success();
    Command::cargo_bin("todo")?.arg("añadir").arg("Tarea 2").assert().success();

    Command::cargo_bin("todo")?.arg("eliminar").arg("1").assert().success();

    Command::cargo_bin("todo")?
        .arg("listar")
        .assert()
        .success()
        .stdout(predicate::str::contains("[ ] 1: Tarea 2 [sin etiquetas]"))
        .stdout(predicate::str::contains("Tarea 1").not());

    Ok(())
}