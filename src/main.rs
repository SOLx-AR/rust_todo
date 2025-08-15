//! Este programa permite al usuario gestionar tareas diarias, mediante adición, edición, listado, completado y eliminación de tareas.
//! 
//! Las tareas pueden ser categorizadas mediante Tags dinámicas, definidas por el usuario, y asignarle tres grados de prioridad. 
//! 
//! También se permite definir subtareas y, finalmente, emitir un reporte histórico de tareas agregadas, completadas, pendientes y eliminadas.
//! 
//! Las categorías, tareas y estadísticas persisten en archivos .JSON.

mod data;
mod functions;
mod json;

use std::io::{self};
use data::*;
use functions::*;
use json::*;

/// Punto de entrada de la aplicación.
///
/// Inicia el gestor de tareas, y despliega el menú para el usuario en línea de comandos en un bucle. Realiza los correspondientes llamados a las funciones y estructuras de datos definidas en los otros módulos. 

fn main() {
    println!("---GESTOR DE TAREAS---");
    let mut tasks: Vec<Task> = load_tasks();
    let mut all_tags: Vec<String> = load_tags();
    let mut stats: Statistics = load_stats();

    loop { 
        println!("\nIngrese un comando: ");
        println!("\tagregar <descripción>\n\tcompletar <id>\n\teditar <id>\n\tlistar\n\teliminar <id>\n\treporte\n\tsalir");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Inténtelo nuevamente."); 
        let input = input.trim();

        match input { 
            "salir" => {
                println!("\n---FINALIZANDO GESTOR---");
                save_tasks(&tasks);
                save_tags(&all_tags);
                save_stats(&stats);
                break;
            },
            "listar" => list_tasks(&tasks),
            "reporte" => show_stats(&stats),
            input if input.starts_with("agregar ") => {
                let description = input[7..].trim();
                add_task(&mut tasks, description, &mut all_tags, &mut stats);
                save_tasks(&tasks);
                save_tags(&all_tags);
                save_stats(&stats);
            },
            input if input.starts_with("completar ") => {
                let id: usize = match input[9..].trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("\nID no válido.");
                        continue;
                    },
                };
                complete_task(&mut tasks, id, &mut stats);
                save_tasks(&tasks);
                save_stats(&stats);
            },
            input if input.starts_with("editar ") => {
                let id: usize = match input[7..].trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("\nID no válido.");
                        continue;
                    },
                };
                edit_task(&mut tasks, id, &mut all_tags);
                save_tasks(&tasks);
                save_tags(&all_tags);
                save_stats(&stats);
            },
            input if input.starts_with("eliminar ") => {
                let id: usize = match input[9..].trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("\nID no válido.");
                        continue;
                    },
                };
                remove_task(&mut tasks, id, &mut stats);
                save_tasks(&tasks);
                save_stats(&stats);
            },
            _ => {
                println!("\nComando no reconocido. Inténtelo otra vez.");
            },
        }
    }
}