use std::io;

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

fn main() {
    println!("Bienvenido al gestor de tareas");

    let mut tareas: Vec<Tarea> = Vec::new();

    loop {
        println!("\ningresa un comando('agregar <descripcion>', 'listar', 'completar <x>', 'salir')");

        let mut entrada = String::new();
        io::stdin()
            .read_line(&mut entrada)
            .expect("Error al leer la entrada");
        let entrada = entrada.trim();

        let mut parts = entrada.splitn(2, ' ');
        let comando = parts.next().unwrap_or("");
        let argumento = parts.next().unwrap_or("").trim();

        match comando {
            "salir" => {
                println!("\nSaliendo del gestor de tareas");
                break;
            }
            "agregar" => {
                if !argumento.is_empty() {
                    tareas.push(Tarea {
                        descripcion: argumento.to_string(),
                        completada: false,
                    });
                    println!("\nTarea agregada: {}", argumento);
                } else {
                    println!("\nLa descripción de la tarea no puede estar vacía.");
                }
            }
            "listar" => {
                listar_tareas(&tareas);
            }
            "completar" => {
                match argumento.parse::<usize>() {
                    Ok(id) if id > 0 && id <= tareas.len() => {
                        tareas[id - 1].completada = true;
                        println!("\nTarea {} marcada como completada.", id);
                    }
                    Ok(_) => {
                        println!("\nID de tarea no válido.");
                    }
                    Err(_) => {
                        println!("\nID inválido. Debe ser un número.");
                    }
                }
            }
            _ => {
                println!("\nComando no reconocido. Intenta de nuevo.");
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