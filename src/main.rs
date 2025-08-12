
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::io::{self, Write};

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

const RUTA_ARCHIVO: &str = "tareas.txt";

fn main() {
    println!("Bienvenido al gestor de tareas");

    let mut tareas: Vec<Tarea> = leer_tareas(RUTA_ARCHIVO);

    loop {
        println!("\ningresa un comando('agregar <descripcion>', 'listar','salir')");

        let mut entrada = String::new();
        io::stdin()
            .read_line(&mut entrada)
            .expect("Error al leer la entrada");
        let entrada = entrada.trim();

        if entrada == "salir" {
            println!("\nSaliendo del gestor de tareas");
            if let Err(e) = guardar_tareas(RUTA_ARCHIVO, &tareas) {
                eprintln!("No se pudieron guardar las tareas: {e}");
            }
            break;
        } else if entrada.starts_with("agregar ") {
            let descripcion = entrada[8..].trim();
            if !descripcion.is_empty() {
                tareas.push(Tarea {
                    descripcion: descripcion.to_string(),
                    completada: false,
                });
                println!("\nTarea agregada: {}", descripcion);
                if let Err(e) = guardar_tareas(RUTA_ARCHIVO, &tareas) {
                    eprintln!("No se pudieron guardar las tareas: {e}");
                }
            } else {
                println!("\nLa descripción de la tarea no puede estar vacía.");
            }
        } else if entrada == "listar" {
            listar_tareas(&tareas);
        } else if entrada.starts_with("completar ") {
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
                if let Err(e) = guardar_tareas(RUTA_ARCHIVO, &tareas) {
                    eprintln!("No se pudieron guardar las tareas: {e}");
                }
            } else {
                println!("\nID de tarea no válido.");
            }
        } else {
            println!("\nComando no reconocido. Intenta de nuevo.");
        }

    }
}

fn listar_tareas(lista_de_tareas: &Vec<Tarea>) {
    println!("\nLista de Tareas:");
    
    for (i, tarea) in lista_de_tareas.iter().enumerate() {
        tarea.mostrar(i + 1);
    }
}


// Guarda las tareas en un archivo de texto simple.
// Formato por línea: "<0|1>\t<descripcion>"
fn guardar_tareas(ruta: &str, tareas: &[Tarea]) -> io::Result<()> {
    let mut archivo = File::create(ruta)?;
    for t in tareas {
        // Reemplaza saltos de línea en la descripción para mantener una línea por tarea.
        let descripcion_segura = t.descripcion.replace('\n', " ");
        let completada_flag = if t.completada { 1 } else { 0 };
        writeln!(archivo, "{}\t{}", completada_flag, descripcion_segura)?;
    }
    Ok(())
}

fn leer_tareas(ruta: &str) -> Vec<Tarea> {
    let mut tareas: Vec<Tarea> = Vec::new();
    if let Ok(archivo) = File::open(ruta) {
        let reader = BufReader::new(archivo);
        for linea in reader.lines() {
            if let Ok(linea) = linea {
                let campos = linea.split('\t').collect::<Vec<&str>>();
                if campos.len() > 1 {
                    let completada = campos[0] == "1";
                    let descripcion = campos[1..].join("\t");
                    tareas.push(Tarea {
                        descripcion,
                        completada,
                    });
                }
            }
        }
    }
    tareas
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