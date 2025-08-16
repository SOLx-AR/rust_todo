use super::*;

#[test]
fn test_desmarcar_tarea() -> Result<(), Box<dyn Error>> {
    setup();
    Command::cargo_bin("todo")?.arg("añadir").arg("Tarea para desmarcar").assert().success();
    Command::cargo_bin("todo")?.arg("completar").arg("1").assert().success();
    let regex_completada = r"\[X\] \(\d{2}/\d{2}/\d{4} \d{2}:\d{2}:\d{2}\) 1: Tarea para desmarcar \[sin etiquetas\]";
    Command::cargo_bin("todo")?
        .arg("listar")
        .assert()
        .success()
        .stdout(predicate::str::is_match(regex_completada)?);
    Command::cargo_bin("todo")?.arg("desmarcar").arg("1").assert().success();
    Command::cargo_bin("todo")?
        .arg("listar")
        .assert()
        .success()
        .stdout(predicate::str::contains("[ ] 1: Tarea para desmarcar [sin etiquetas]"));
    Ok(())
}

#[test]
fn test_etiquetar_tarea() -> Result<(), Box<dyn Error>> {
    setup();
    Command::cargo_bin("todo")?.arg("añadir").arg("Tarea para etiquetar").assert().success();
    Command::cargo_bin("todo")?
        .arg("etiquetar")
        .arg("1")
        .arg("trabajo")
        .assert()
        .success();
    Command::cargo_bin("todo")?
        .arg("listar")
        .assert()
        .success()
        .stdout(predicate::str::contains("[ ] 1: Tarea para etiquetar [trabajo]"));
    Command::cargo_bin("todo")?
        .arg("etiquetar")
        .arg("1")
        .arg("tag_invalido")
        .assert()
        .success()
        .stdout(predicate::str::contains("Error: La etiqueta 'tag_invalido' no es válida."));
    Ok(())
}

#[test]
fn test_etiquetar_y_desetiquetar_tarea() -> Result<(), Box<dyn Error>> {
    setup();
    Command::cargo_bin("todo")?.arg("añadir").arg("Mi tarea para etiquetas").assert().success();
    Command::cargo_bin("todo")?
        .arg("etiquetar")
        .arg("1")
        .arg("trabajo")
        .assert()
        .success();
    Command::cargo_bin("todo")?
        .arg("listar")
        .assert()
        .success()
        .stdout(predicate::str::contains("[ ] 1: Mi tarea para etiquetas [trabajo]"));
    Command::cargo_bin("todo")?
        .arg("desetiquetar")
        .arg("1")
        .arg("trabajo")
        .assert()
        .success();
    Command::cargo_bin("todo")?
        .arg("listar")
        .assert()
        .success()
        .stdout(predicate::str::contains("[ ] 1: Mi tarea para etiquetas [sin etiquetas]"));
    Ok(())
}