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
        println!("\nIngresa un comando ('agregar <descripcion>', 'listar', 'completar', 'salir')");

        let mut entrada = String::new();
        io::stdin()
            .read_line(&mut entrada)
            .expect("Error al leer la entrada");
        let entrada = entrada.trim();

        // Dividimos la cadena en un iterador de palabras.
        let mut parts = entrada.split_whitespace();
        // `parts.next()` nos da la primera palabra (el comando) como `Option<&str>`.
        let comando: Option<&str> = parts.next();

        match comando {
            Some("salir") => {
                println!("\nSaliendo del gestor de tareas.");
                break;
            }
            Some("agregar") => {
                // Recogemos todas las palabras restantes del iterador y las unimos en una sola cadena.
                let descripcion: String = parts.collect::<Vec<&str>>().join(" ");
                if !descripcion.is_empty() {
                    tareas.push(Tarea {
                        descripcion: descripcion,
                        completada: false,
                    });
                    println!("✅ Tarea agregada.");
                } else {
                    println!("❌ La descripción de la tarea no puede estar vacía.");
                }
            }
            Some("listar") => {
                listar_tareas(&tareas);
            }
            Some("completar") => {
                // Tomamos el próximo elemento de `parts` que debería ser el ID.
                match parts.next() {
                    Some(id_str) => match id_str.parse::<usize>() {
                        Ok(id) if id > 0 && id <= tareas.len() => {
                            tareas[id - 1].completada = true;
                            println!("✅ Tarea {} marcada como completada.", id);
                        }
                        _ => println!("❌ ID de tarea no válido. Ingresa un número válido."),
                    },
                    None => {
                        println!("❌ El comando 'completar' requiere un ID de tarea.");
                    }
                }
            }
            Some(_) => {
                println!("❌ Comando no reconocido. Intenta de nuevo.");
            }
            None => {
                // Este caso maneja las líneas vacías.
                println!("⚠️ Ingresa un comando.");
            }
        }
    } // The main function ends here
}

// The listar_tareas function is now outside of main
fn listar_tareas(lista_de_tareas: &Vec<Tarea>) {
    println!("\nLista de Tareas:");

    for (i, tarea) in lista_de_tareas.iter().enumerate() {
        tarea.mostrar(i + 1);
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