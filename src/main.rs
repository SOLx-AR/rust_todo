use std::io;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write, ErrorKind};
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct Tarea {
    descripcion: String,
    completada: bool,
}

impl Tarea {
    fn mostrar(&self, id: usize) {
        let estado = if self.completada { "[X]" } else { "[ ]" };
        println!("{} {}: {}",estado, id, self.descripcion);
    }
}

fn main() {
    println!("Bienvenido al gestor de tareas");

     let archivo_de_tareas = "task-list.json";

    // Load tasks
    let mut tareas = load_task_list(archivo_de_tareas);
    // let mut tareas: Vec<Tarea> = Vec::new();

    loop {
        println!("\ningresa un comando('agregar <descripcion>', 'completar <numero>', 'listar', 'salir')");

        let mut entrada = String::new();
        io::stdin()
            .read_line(&mut entrada)
            .expect("Error al leer la entrada");
        let entrada = entrada.trim();

        // if entrada == "salir" {
        //     println!("\nSaliendo del gestor de tareas");
        //     break;
        // } else if entrada.starts_with("agregar ") {
        //     let descripcion = entrada[8..].trim();
        //     if !descripcion.is_empty() {
        //         tareas.push(Tarea {
        //             descripcion: descripcion.to_string(),
        //             completada: false,
        //         });
        //         println!("\nTarea agregada: {}", descripcion);
        //     } else {
        //         println!("\nLa descripción de la tarea no puede estar vacía.");
        //     }
        // } else if entrada == "listar" {
        //     listar_tareas(&tareas);
        // } else if entrada.starts_with("completar ") {
        //     let id: usize = match entrada[10..].trim().parse() {
        //         Ok(num) => num,
        //         Err(_) => {
        //             println!("\nID inválido. Debe ser un número.");
        //             continue;
        //         }
        //     };
        //     if id > 0 && id <= tareas.len() {
        //         tareas[id - 1].completada = true;
        //         println!("\nTarea {} marcada como completada.", id);
        //     } else {
        //         println!("\nID de tarea no válido.");
        //     }
        // } else {
        //     println!("\nComando no reconocido. Intenta de nuevo.");
        // }

        match entrada {
        "salir" => {
            println!("\nSaliendo del gestor de tareas");
            save_task_list(archivo_de_tareas, &tareas);
            break;
        }
        "listar" => {
            listar_tareas(&tareas);
        }
        s if s.starts_with("agregar ") => {
            let descripcion = entrada[8..].trim();
            if !descripcion.is_empty() {
                tareas.push(Tarea {
                    descripcion: descripcion.to_string(),
                    completada: false,
                });
                println!("\nTarea agregada: {}", descripcion);
            } else {
                println!("\nLa descripción de la tarea no puede estar vacía.");
            }
        }
        s if s.starts_with("completar ") => {
             let id: usize = match entrada[10..].trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("\nID inválido. Debe ser un número.");
                    continue;
                }
            };
            if id > 0 && id <= tareas.len() {
                tareas[id - 1].completada = true;
                println!("\nTarea {} marcada como completada.", id);
            } else {
                println!("\nID de tarea no válido.");
            }
        }
        _ => {
            println!("Unknown command");
        }
    }

    }
}

fn listar_tareas(lista_de_tareas: &Vec<Tarea>) {
    println!("\nLista de Tareas:");
    
    for (i, tarea) in lista_de_tareas.iter().enumerate() {
        tarea.mostrar(i + 1);
    }
}

fn load_task_list(path: &str) -> Vec<Tarea> {
    let file = File::open(path);


     let mut task_file = match file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(path) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            _ => {
                panic!("Problem opening the file: {error:?}");
            }
        },
    };

    let mut contents = String::new();
    task_file.read_to_string(&mut contents).expect("Failed to read task list file");

    if contents.trim().is_empty() {
        // New file or empty, start with empty vector
        Vec::new()
    } else {
        serde_json::from_str(&contents).unwrap_or_else(|err| {
            eprintln!("Error parsing JSON: {}. Starting with empty list.", err);
            Vec::new()
        })
    }
}

fn save_task_list(path: &str, tasks: &Vec<Tarea>) {
    let json = serde_json::to_string_pretty(tasks).expect("Failed to serialize tasks");
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(path)
        .expect("Failed to open task list file for writing");

    file.write_all(json.as_bytes()).expect("Failed to write task list file");
}




/* 
    Desafio uno:
        Refactorizar el codigo con un match en vez de if, else if, else
    
    Desafio dos:
        Guardar las tareas en un archivo

    Desafio tres:
        Investigar el crate serde y como se usaria para serializar y deserializar las tareas
    
    Desafio cuatro:
        Cargar las tareas desde un archivo al iniciar el programa 

 */