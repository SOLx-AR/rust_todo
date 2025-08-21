use std::io;

mod model;

use crate::model::Tareas;

fn main() {
    println!("=== Bienvenido al gestor de tareas ===");

    let mut tareas = Tareas::cargar();
    tareas.listar();

    loop {
        println!("\n* Ingrese comando (h para ayuda) ");

        let mut entrada = String::new();
        io::stdin()
            .read_line(&mut entrada)
            .expect("Error al leer la entrada");

        let entrada: Vec<&str> = entrada.trim().split_whitespace().collect();

        if entrada.is_empty() {
            continue;
        }
        let comando = *entrada.first().unwrap_or(&"");
        match comando {
            "h" => {
                println!(
                    "\nComandos disponibles:
  agregar          <descripcion>
  completar        <id>
  etiquetar        <id> <etiqueta>
  priorizar        <id> <prioridad>
  agregar_subtarea <parent id> <descripcion>
  listar
  reordenar
  guardar
  salir"
                );
            }
            "salir" => {
                tareas.guardar();
                println!("\nSaliendo del gestor de tareas");
                break;
            }
            "listar" => {
                tareas.listar();
            }
            "guardar" => {
                tareas.guardar();
            }
            "cargar" => {
                tareas = Tareas::cargar();
            }
            "agregar" => {
                if entrada.len() < 2 {
                    println!("\nLa descripción es requerida.");
                    continue;
                }
                let descripcion = entrada[1..].join(" ");
                tareas.agregar(&descripcion);
            }
            "agregar_subtarea" => {
                if entrada.len() < 3 {
                    println!("\nID de la tarea padre y descripción son requeridos.");
                    continue;
                }
                let id_padre: usize = match entrada[1].parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("\nID de la tarea padre inválido. Debe ser un número.");
                        continue;
                    }
                };
                let descripcion = entrada[2..].join(" ");
                tareas.agregar_subtarea(id_padre, &descripcion);
            }
            "completar" => {
                if entrada.len() < 2 {
                    println!("\nID inválido. Debe ser un número.");
                    continue;
                }
                let id: usize = match entrada[1].parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("\nID inválido. Debe ser un número.");
                        continue;
                    }
                };
                tareas.completar(id);
            }
            "etiquetar" => {
                if entrada.len() < 3 {
                    println!("\nID y etiqueta son requeridos.");
                    continue;
                }
                let id: usize = match entrada[1].parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("\nID inválido. Debe ser un número.");
                        continue;
                    }
                };
                let etiqueta = entrada[2..].join(" ");
                tareas.etiquetar(id, &etiqueta);
            }
            "reordenar" => {
                tareas.tareas.sort();
                tareas.listar();
            }
            "priorizar" => {
                if entrada.len() < 3 {
                    println!("\nID y prioridad son requeridos.");
                    continue;
                }
                let id: usize = match entrada[1].parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("\nID inválido. Debe ser un número.");
                        continue;
                    }
                };
                let prioridad: u32 = match entrada[2].parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("\nPrioridad inválida. Debe ser un número.");
                        continue;
                    }
                };
                tareas.priorizar(id, prioridad);
            }
            _ => {
                println!("\nComando no reconocido. Intenta de nuevo.");
            }
        }
    }
}