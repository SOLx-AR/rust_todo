use super::*;

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
fn test_importar_tareas_y_fusionar() -> Result<(), Box<dyn Error>> {
    setup();
    Command::cargo_bin("todo")?.arg("añadir").arg("Tarea Original").assert().success();
    let archivo_a_importar = "para_importar.json";
    let contenido_importar = r#"[{"descripcion":"Tarea Importada 1","completada":false,"ultimo_cambio":null,"tags":[]}]"#;
    fs::write(archivo_a_importar, contenido_importar)?;
    Command::cargo_bin("todo")?
        .arg("importar_y_sumar")
        .arg(archivo_a_importar)
        .assert()
        .success();
    Command::cargo_bin("todo")?
        .arg("listar")
        .assert()
        .success()
        .stdout(predicate::str::contains("1: Tarea Original [sin etiquetas]"))
        .stdout(predicate::str::contains("2: Tarea Importada 1 [sin etiquetas]"));
    fs::remove_file(archivo_a_importar)?;
    Ok(())
}

#[test]
fn test_cargar_tareas_y_reemplazar() -> Result<(), Box<dyn Error>> {
    setup();
    Command::cargo_bin("todo")?.arg("añadir").arg("Tarea Vieja que va a desaparecer").assert().success();
    let archivo_a_cargar = "para_cargar.txt";
    let contenido_cargar = "no completada,,Tarea Nueva 1,\ncompletada,2025-01-01T11:00:00Z,Tarea Nueva 2,trabajo;personal";
    fs::write(archivo_a_cargar, contenido_cargar)?;
    Command::cargo_bin("todo")?
        .arg("reemplazar_con_archivo")
        .arg(archivo_a_cargar)
        .assert()
        .success();
    Command::cargo_bin("todo")?
        .arg("listar")
        .assert()
        .success()
        .stdout(predicate::str::contains("1: Tarea Nueva 1 [sin etiquetas]"))
        .stdout(predicate::str::contains("2: Tarea Nueva 2 [trabajo, personal]"))
        .stdout(predicate::str::contains("Tarea Vieja").not());
    fs::remove_file(archivo_a_cargar)?;
    Ok(())
}

#[test]
fn test_eliminar_todas_las_tareas() -> Result<(), Box<dyn Error>> {
    setup();
    Command::cargo_bin("todo")?.arg("añadir").arg("Tarea de prueba").assert().success();
    let settings = toml::from_str::<TestSettings>(&fs::read_to_string("Settings.toml")?)?;
    let filename = format!("{}.{}", settings.storage.filename_base, settings.storage.format);
    assert!(fs::metadata(&filename).is_ok(), "El archivo de tareas no se creó antes de eliminar.");
    Command::cargo_bin("todo")?.arg("eliminar_todas").assert().success();
    assert!(!fs::metadata(&filename).is_ok(), "El archivo de tareas no fue eliminado.");
    Ok(())
}