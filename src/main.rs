use std::io;
use std::fs::File;
use std::io::Write;

struct Tarea 
{
    descripcion: String,
    completada: bool,
}

impl Tarea 
{
    fn mostrar(&self, id: usize) 
    {
        let estado = if self.completada { "[X]" } else { "[ ]" };
        println!("{} {}: {}",estado, id, self.descripcion);
    }
}


fn main() 
{
    println!("BIENVENIDO GESTOR DE TAREAS!!!"); //imprimo saludo

    let mut tareas: Vec<Tarea> = Vec::new(); //creo un vector vacio "Vec::new()" en donde adentro hay variavles del struct Tarea

    loop //creo un loop infinito
    {
        println!("\nIngresa un comando('agregar <descripcion>', 'listar', 'salir', 'completar <numero de la tarea>')"); //imprimo indicaciones

        let mut entrada = String::new(); //creo un string vacio modificable para ingresar los datos

        io::stdin()
            .read_line(&mut entrada)
            .expect("Error al leer la entrada");
        let entrada = entrada.trim();

        match entrada //uso el match (es como un switch en C)
        {
            "salir" => 
            {
                println!("\nSaliendo del gestor de tareas");
                break; //sale del loop
            }
            "listar" =>
            {
                listar_tareas(&tareas);
            }
            _ if entrada.starts_with("agregar ") =>
            {
                let descripcion = entrada[8..].trim();
                if !descripcion.is_empty() 
                {
                    tareas.push(Tarea {descripcion: descripcion.to_string(), completada: false,});

                    println!("\nTarea agregada: {}", descripcion);
                    guardar_tareas_a_archivo(&tareas);
                } 
                else 
                {
                    println!("\nLa descripción de la tarea no puede estar vacía.");
                }
            }
            _ if entrada.starts_with("completar ") =>
            {
                let id: usize = match entrada[10..].trim().parse() 
                {
                    Ok(num) => num,
                    Err(_) => 
                    {
                        println!("\nID inválido. Debe ser un número y que exista en la lista.");
                        continue;
                    }
                };

                if id > 0 && id <= tareas.len() {
                    tareas[id - 1].completada = true;
                    println!("\nTarea {} marcada como completada.", id);
                    guardar_tareas_a_archivo(&tareas);
                } 
                else { println!("\nID de tarea no válido."); }
            }
            _ =>
            {
                println!("\nComando no reconocido. Intenta de nuevo.");
            }
        }
    }
}


fn listar_tareas(lista_de_tareas: &Vec<Tarea>) //funcion para imprimir toda la lista de tareas hasta el momento
{
    println!("\nLista de Tareas:");
    
    for (i, tarea) in lista_de_tareas.iter().enumerate() 
    {
        tarea.mostrar(i + 1);
    }
}

fn guardar_tareas_a_archivo(tareas: &Vec<Tarea>) //funcion guarda el vector de tareas en el archivo
{
    let resultado_archivo = File::create("tareas.txt"); //crea el archivo 

    let mut archivo = match resultado_archivo //chequea si se creo
    {
        Ok(archivo_abierto) => archivo_abierto,
        Err(error_creacion) => 
        {
            eprintln!("Error creando archivo: {}", error_creacion);
            return;
        }
    };

    for (i, tarea) in tareas.iter().enumerate()  //por enumerate i guarda la posicion y tarea el contenido
    {
        let estado = if tarea.completada { "X" } else { " " };
        if let Err(error_escritura) = writeln!(archivo, "[{}] {}: {}", estado, i + 1, tarea.descripcion) 
        {
            eprintln!("Error escribiendo en archivo: {}", error_escritura);
            return;
        }
    }
}
