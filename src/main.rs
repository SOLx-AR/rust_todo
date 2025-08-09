use std::env;
use std::error::Error;
use std::fs; 
use std::io::{self, Read, Write}; 

#[derive(Debug, Clone, PartialEq)] 
struct Tarea {
    descripcion: String,
    completada: bool,
}

impl Tarea {
    fn mostrar(&self, id: usize) {
        let estado = if self.completada { "[X]" } else { "[ ]" };
        println!("{} {}: {}", estado, id, self.descripcion);
    }
}


fn procesar_comando(comando: &str, args: &str, tareas: &mut Vec<Tarea>) {
    match comando {
        "agregar" => {
            if !args.is_empty() {
                tareas.push(Tarea {
                    descripcion: args.to_string(),
                    completada: false,
                });
                println!("✅ Tarea agregada.");
            } else {
                println!("❌ La descripción de la tarea no puede estar vacía.");
            }
        }
        "listar" => {
            listar_tareas(tareas);
        }
        "completar" => {
            match args.parse::<usize>() {
                Ok(id) if id > 0 && id <= tareas.len() => {
                    tareas[id - 1].completada = true;
                    println!("✅ Tarea {} marcada como completada.", id);
                }
                _ => println!("❌ ID de tarea no válido."),
            }
        }
        _ => {
            println!("❌ Comando no reconocido.");
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut tareas: Vec<Tarea> = {
        let file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open("tareas.csv")?;
        cargar_tareas(file).unwrap_or_else(|_| Vec::new())
    };

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        // --- Test Mode ---
        let comando = &args[1];
        let resto_args = &args[2..].join(" ");
        procesar_comando(comando, resto_args, &mut tareas);

        if let Err(e) = {
            let file = fs::File::create("tareas.csv")?;
            guardar_tareas(file, &tareas)
        } {
            eprintln!("Error al guardar tareas: {}", e);
        }

    } else {
        // --- Interactive Mode ---
        println!("Bienvenido al gestor de tareas");
        loop {
            println!("\nIngresa un comando ('agregar <descripcion>', 'listar', 'completar', 'salir')");
            let mut entrada = String::new();
            io::stdin()
                .read_line(&mut entrada)
                .expect("Error al leer la entrada");
            
            let entrada = entrada.trim();
            let mut parts = entrada.split_whitespace();
            
            if let Some(comando) = parts.next() {
                if comando == "salir" {
                    println!("\nSaliendo del gestor de tareas.");
                    break;
                }
                let resto_args = parts.collect::<Vec<&str>>().join(" ");

                procesar_comando(comando, &resto_args, &mut tareas);
                
                if let Err(e) = {
                    let file = fs::File::create("tareas.csv")?;
                    guardar_tareas(file, &tareas)
                } {
                    eprintln!("Error al guardar tareas: {}", e);
                }
            }
        }
    }
    Ok(())
}

fn listar_tareas(lista_de_tareas: &Vec<Tarea>) {
    println!("\nLista de Tareas:");
    for (i, tarea) in lista_de_tareas.iter().enumerate() {
        tarea.mostrar(i + 1);
    }
}

fn guardar_tareas<W: Write>(writer: W, lista_de_tareas: &Vec<Tarea>) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_writer(writer);
    for tarea in lista_de_tareas {
        wtr.write_record(&[
            tarea.completada.to_string(),
            tarea.descripcion.clone(),
        ])?;
    }
    wtr.flush()?;
    Ok(())
}

fn cargar_tareas<R: Read>(reader: R) -> Result<Vec<Tarea>, Box<dyn Error>> {
    let mut tareas = Vec::new();
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(reader);

    for result in rdr.records() {
        let record = result?;
        let completada: bool = record.get(0).unwrap_or("").parse().unwrap_or(false);
        let descripcion: String = record.get(1).unwrap_or("").to_string();

        if !descripcion.is_empty() {
            tareas.push(Tarea {
                descripcion,
                completada,
            });
        }
    }
    Ok(tareas)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crear_tarea_nueva() {
        let tarea = Tarea {
            descripcion: "Aprender Rust".to_string(),
            completada: false,
        };
        assert_eq!(tarea.descripcion, "Aprender Rust");
        assert_eq!(tarea.completada, false);
    }

    #[test]
    fn marcar_tarea_como_completada() {
        let mut tarea = Tarea {
            descripcion: "Hacer tests".to_string(),
            completada: false,
        };
        tarea.completada = true;
        assert_eq!(tarea.completada, true);
    }

    #[test]
    fn test_guardar_y_cargar_en_memoria() {
        let tareas_originales = vec![
            Tarea {
                descripcion: "Tarea de prueba 1".to_string(),
                completada: false,
            },
            Tarea {
                descripcion: "Tarea de prueba 2".to_string(),
                completada: true,
            },
        ];

        let mut buffer: Vec<u8> = Vec::new();
        guardar_tareas(&mut buffer, &tareas_originales).unwrap();
        
        let tareas_cargadas = cargar_tareas(&*buffer).unwrap();
        
        assert_eq!(tareas_originales, tareas_cargadas);
    }
}


/* Desafio uno:
        Refactorizar el codigo con un match en vez de if, else if, else
    
    Desafio dos:
        Guardar las tareas en un archivo

    Desafio tres:
        Investigar el crate serde y como se usaria para serializar y deserializar las tareas
    
    Desafio cuatro:
        Cargar las tareas desde un archivo al iniciar el programa 

 */