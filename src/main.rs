use serde::{Deserialize, Serialize};
use std::io::{BufReader, Write};
use std::path::Path;
use std::{fs::File, io};

#[derive(Serialize, Deserialize, Debug)]
enum Tags {
    Cocina,
    Trabajo,
    Educacion,
    Hobbie,
    Social,
    Vacio,
}

#[derive(Serialize, Deserialize, Debug)]
struct Tarea {
    descripcion: String,
    completada: bool,
    prioridad: u8,
    etiquetas: Tags,
}

impl Tarea {
    fn mostrar(&self, id: usize) {
        let estado = if self.completada { "[X]" } else { "[ ]" };
        let etiqueta = match self.etiquetas {
            Tags::Cocina => "Cocina",
            Tags::Trabajo => "Trabajo",
            Tags::Educacion => "Educacion",
            Tags::Hobbie => "Hobbie",
            Tags::Social => "Social",
            Tags::Vacio => "Vacio",
        };
        println!(
            "{} | {} | {} | {} | {}",
            estado, id, self.descripcion, self.prioridad, etiqueta
        );
    }
}

fn main() {
    println!("Bienvenido al gestor de tareas");
    let nombre_archivo = "tareas.json";
    let mut tareas = cargar_tareas(nombre_archivo).unwrap_or_default();

    loop {
        println!(
            "\ningresa un comando('agregar <descripcion>', 'completar <id>', 'listar','salir', 'prioridad <id>', etiquetar <id>)"
        );
        let mut entrada = String::new();
        io::stdin()
            .read_line(&mut entrada)
            .expect("Error al leer la entrada");
        let entrada = entrada.trim();

        match entrada {
            "salir" => {
                println!("\nSaliendo del gestor de tareas");
                guardar_tareas(tareas, nombre_archivo);
                break;
            }
            "listar" => {
                listar_tareas(&tareas);
            }
            _ if entrada.starts_with("agregar ") => {
                let descripcion = entrada[8..].trim();
                if !descripcion.is_empty() {
                    tareas.push(Tarea {
                        descripcion: descripcion.to_string(),
                        completada: false,
                        prioridad: 3,
                        etiquetas: Tags::Vacio,
                    });
                    println!("\nTarea agregada: {descripcion}");
                } else {
                    println!("\nLa descripción de la tarea no puede estar vacía.");
                }
            }
            _ if entrada.starts_with("completar ") => {
                let id: usize = match entrada[10..].trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("\nID inválido. Debe ser un número.");
                        continue;
                    }
                };
                if id > 0 && id <= tareas.len() {
                    tareas[id - 1].completada = true;
                    println!("\nTarea {id} marcada como completada.");
                } else {
                    println!("\nID de tarea no válido.");
                }
            }
            _ if entrada.starts_with("prioridad ") => {
                let id: usize = match entrada[10..].trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("\nID inválido. Debe ser un número.");
                        continue;
                    }
                };
                println!("Escoja prioridad (1, 2 o 3)");
                let mut priority = String::new();
                io::stdin()
                    .read_line(&mut priority)
                    .expect("Error al leer entrada");
                let priority = priority.trim().parse::<u8>();
                let priority_num = match priority {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Error al castear entrada a u8");
                        continue;
                    }
                };
                if (priority_num > 0 && priority_num <= 3) && (id > 0 && id <= tareas.len()) {
                    tareas[id - 1].prioridad = priority_num;
                    println!("\nTarea {id} ahora tiene una proridad de {priority_num}.");
                } else {
                    println!("\nID de tarea o prioridad no válido.");
                }
            }
            _ if entrada.starts_with("etiquetar ") => {
                let id: usize = match entrada[10..].trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("\nID inválido. Debe ser un número.");
                        continue;
                    }
                };
                println!("Escoja etiqueta (Cocina, Trabajo, Educacion, Hobbie, Social, Vacio)");
                let mut etiqueta = String::new();
                io::stdin()
                    .read_line(&mut etiqueta)
                    .expect("Error al leer entrada");
                // lowercase para que funcione independientemente de la tipografia
                tareas[id - 1].etiquetas = match etiqueta.trim().to_lowercase().as_str() {
                    "cocina" => Tags::Cocina,
                    "trabajo" => Tags::Trabajo,
                    "educacion" => Tags::Educacion,
                    "hobbie" => Tags::Hobbie,
                    "social" => Tags::Social,
                    _ => {
                        println!("opcion no reconocida usando etiqueta default 'Vacio'");
                        Tags::Vacio
                    }
                };
            }
            _ => println!("\nComando no reconocido. Intenta de nuevo."),
        }
    }
}

fn listar_tareas(lista_de_tareas: &[Tarea]) {
    println!("\nLista de Tareas:");
    println!("completada | id | descripcion | prioridad | etiqueta");

    for (i, tarea) in lista_de_tareas.iter().enumerate() {
        tarea.mostrar(i + 1);
    }
}

fn cargar_tareas<P: AsRef<Path>>(direccion: P) -> Result<Vec<Tarea>, Box<dyn std::error::Error>> {
    let archivo_abierto = File::open(direccion);
    let mut tareas: Vec<Tarea> = vec![];
    if let Ok(archivo) = archivo_abierto {
        // Si el archivo existe devuelvo las tareas
        let reader = BufReader::new(archivo);
        tareas = serde_json::from_reader(reader)?;
    }
    Ok(tareas)
}

fn guardar_tareas<P: AsRef<Path>>(lista_tareas: Vec<Tarea>, direccion: P) {
    let mut archivo = File::create(direccion).unwrap();
    let serialized = serde_json::to_string_pretty(&lista_tareas).unwrap();
    archivo.write_all(serialized.as_bytes()).unwrap();
}

/*
    Desafio uno:
        Refactorizar el codigo con un match en vez de if, else if, else (DONE)

    Desafio dos:
        Guardar las tareas en un archivo serializado en formato JSON con el crate serde (DONE)

    Desafio tres:
        Cargar las tareas desde un archivo al iniciar el programa deserializado en formato JSON con el crate serde (DONE)

    Desafio cuatro:
        emitir reportes (TODO)

    Desafio cinco:
        prioridades (DONE)

    Desafio seis:
        etiquetas (DONE)

    Desafio siete:
        subtareas (TODO)
*/
