use super::*; 

#[test]
fn test_flujo_feliz() -> Result<(), Box<dyn Error>> {
    setup();
    Command::cargo_bin("todo")?.arg("añadir").arg("Comprar pan").assert().success();
    Command::cargo_bin("todo")?.arg("añadir").arg("Pasear al perro").assert().success();
    
    Command::cargo_bin("todo")?
        .arg("listar")
        .assert()
        .success()
        .stdout(predicate::str::contains("[ ] 1: Comprar pan [sin etiquetas]"))
        .stdout(predicate::str::contains("[ ] 2: Pasear al perro [sin etiquetas]"));

    Command::cargo_bin("todo")?.arg("completar").arg("1").assert().success();

    let regex_tarea_completada = r"\[X\] \(\d{2}/\d{2}/\d{4} \d{2}:\d{2}:\d{2}\) 1: Comprar pan \[sin etiquetas\]";
    Command::cargo_bin("todo")?
        .arg("listar")
        .assert()
        .success()
        .stdout(predicate::str::is_match(regex_tarea_completada)?)
        .stdout(predicate::str::contains("[ ] 2: Pasear al perro [sin etiquetas]"));
    Ok(())
}