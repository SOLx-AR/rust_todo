use std::io;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use serde::{Serialize, Deserialize};
use colored::*;

#[derive(Serialize, Deserialize)]
struct Tarea {
    descripcion: String,
    completada: bool,
    prioridad: Prioridad,
    etiqueta: Vec<String>,
    subtareas: Vec<Tarea>
}

#[derive(Serialize, Deserialize)]
enum Prioridad {
    Baja,
    Media,
    Alta
}

impl Tarea {
    fn mostrar(&self, id: usize) {
        let estado = if self.completada { "[X]" } else { "[ ]" };
        println!("{} {}: {}",estado, id, self.descripcion);
    }
}

fn main() {
    println!("{}", "Bienvenido al gestor de tareas".blue().bold());

    let mut tareas: Vec<Tarea> = cargar_tareas().unwrap_or_else(|_| Vec::new());

    loop {
        println!("{}", "\ningresa un comando('agregar (<descripcion> <prioridad> <etiqueta>)', subtarea (<id> <descripcion>), 'listar', 'completar (<id>)', 'reportes', 'salir')".yellow());
        println!("{}", "si desea agregar, primero escriba la descripcion, luego la prioridad del 1 al 3 y la etiqueta opcional precedida por '#'".green().bold());
        let mut entrada = String::new();
        io::stdin()
            .read_line(&mut entrada)
            .expect("Error al leer la entrada");
        let entrada = entrada.trim();

        match entrada {
            "salir" => {
                if let Err(e) = guardar_tareas(&tareas) {
                    eprintln!("Error al guardar las tareas: {}", e);
                }
                println!("{}", "\nSaliendo del gestor de tareas".green());
                break;
            }
            "listar" => listar_tareas(&tareas),
            "reportes" => emitir_reportes(&tareas),
            _ if entrada.starts_with("agregar ") => {
                let descripcion = entrada[8..].trim();
                if !descripcion.is_empty() {
                     let descripcion_real = descripcion
                     .split_whitespace()
                     .filter(|c| !c.starts_with("#"))
                     .map(|c| c.to_string())
                     .collect::<Vec<String>>()
                     .join(" ");

                    let prioridad_real = match descripcion {
                        "1" => Prioridad::Alta,
                        "2" => Prioridad::Media,
                        "3" => Prioridad::Baja,
                        _ => Prioridad::Baja,
                    };
                    let etiqueta_real = descripcion
                        .split_whitespace()
                        .filter(|p| p.starts_with("#"))
                        .map(|s| s.to_string())
                        .collect();


                    tareas.push(Tarea {
                        descripcion: descripcion_real,
                        completada: false,
                        prioridad: prioridad_real,
                        etiqueta: etiqueta_real,
                        subtareas: Vec::new(),
                        });                    
                    println!("\nTarea agregada: {}", descripcion);
                } else {
                    println!("{}","\nLa descripción de la tarea no puede estar vacía.".red());
                }
            }

            _ if entrada.starts_with("subtarea ") => {
                let partes: Vec<&str> = entrada.split_whitespace().collect();

                if partes.len() < 3 {
                    println!("{}", "\nUso incorrecto del comando. Debe ser 'subtarea <id> <descripcion>'".red());
                    continue;
                }

                let id: usize = match partes[1].parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("{}", "\nID inválido. Debe ser un número.".red());
                        continue;
                    }
                };

                if id == 0 || id > tareas.len() {
                    println!("{}", "\nNo existe una tarea con ese ID.".red());
                    continue;
                }

                let desc_real = partes[2..].join(" ");
                
                tareas[id - 1].subtareas.push(Tarea {
                    descripcion: desc_real.clone(),
                    prioridad: Prioridad::Baja,
                    completada: false,
                    etiqueta: Vec::new(),
                    subtareas: Vec::new(),
                });
                println!("\nSubtarea agregada a la tarea {}: {}", id, desc_real);
            }

            _ if entrada.starts_with("completar ") => {
                let id: usize = match entrada[10..].trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("{}", "\nID inválido. Debe ser un número.".red());
                        continue;
                    }
                };
                if id > 0 && id <= tareas.len() {
                    tareas[id - 1].completada = true;
                    println!("\nTarea {} marcada como completada.", id);
                } else {
                    println!("{}", "\nID de tarea no válido.".red());
                }
            }
            _ => println!("{}", "\nComando no reconocido. Intenta de nuevo.".red()),
        }

    }
}

fn listar_tareas(lista_de_tareas: &Vec<Tarea>) {
    println!("\nLista de Tareas:");
    
    for (i, tarea) in lista_de_tareas.iter().enumerate() {
        tarea.mostrar(i + 1);
    }
}

fn guardar_tareas(lista_de_tareas: &Vec<Tarea>) -> io::Result<()> {
    let serializado = serde_json::to_string(lista_de_tareas)?;
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("tareas.json")?;
    file.write_all(serializado.as_bytes())?;
    Ok(())
}

fn cargar_tareas() -> std::io::Result<Vec<Tarea>> {
    let mut archivo = File::open("tareas.json")?;
    let mut contenido = String::new();
    archivo.read_to_string(&mut contenido)?;
    let tareas: Vec<Tarea> = serde_json::from_str(&contenido)?;
    Ok(tareas)  
}

fn emitir_reportes(tareas: &Vec<Tarea>) {
    let total = tareas.len();
    let completadas = tareas.iter().filter(|t| t.completada).count();
    let pendientes = total - completadas;

    println!("\n--- Reporte de Tareas ---");
    println!("Total de tareas: {}", total);
    println!("Tareas completadas: {}", completadas);
    println!("Tareas pendientes: {}", pendientes);
}
