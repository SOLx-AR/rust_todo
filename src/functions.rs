//! ## Funciones lógicas principales.
//!
//! Incluye funciones para añadir, editar, eliminar, listar y completar tareas, así como para mostrar las estadísticas de uso del usuario.

use std::io::{self};
use crate::data::*;

/// ### Agrega una nueva tarea.
///
/// Permite al usuario elegir una prioridad entre las predeterminadas. Permite al usuario agregar una categoría nueva o elegir una entre las preexistenes. También permite que la tarea se agregue sin categoría.
/// 
/// Actualiza las estadísticas de uso y persiste los cambios en el archivo JSON.
pub fn add_task(tasks: &mut Vec<Task>, description: &str, all_tags: &mut Vec<String>, stats: &mut Statistics) {
    if description.is_empty() {
        println!("La descripción no puede estar vacía.");
        return;
    }

    let priority = loop {
        println!("Ingrese la prioridad (1: Alta, 2: Media, 3: Baja): ");
        let mut priority_input = String::new();
        io::stdin().read_line(&mut priority_input).expect("Inténtelo nuevamente.");
        let priority_input = priority_input.trim();

        match priority_input.parse::<u8>() {
            Ok(1) => break Priority::Alta,
            Ok(2) => break Priority::Media,
            Ok(3) => break Priority::Baja,
            _ => {
                println!("Prioridad no válida. Por favor, intente de nuevo.");
            },
        }
    };
    
    println!("\nCategorías disponibles:");
    if all_tags.is_empty() {
        println!("\t(No hay categorías existentes)");
    } else {
        for (i, tag) in all_tags.iter().enumerate() {
            println!("\t{}: {}", i + 1, tag);
        }
    }
    
    let tag = loop {
        println!("Ingrese el número de la categoría, escriba una nueva, o presione Enter para 'Sin Categoría':");
        let mut tag_input = String::new();
        io::stdin().read_line(&mut tag_input).expect("Inténtelo nuevamente.");
        let tag_input = tag_input.trim();

        if tag_input.is_empty() {
            println!("Categoría asignada: 'Sin Categoría'.");
            break "Sin Categoría".to_string();
        }

        if let Ok(id) = tag_input.parse::<usize>() {
            if id > 0 && id <= all_tags.len() {
                break all_tags[id - 1].clone();
            } else {
                println!("ID de categoría no válido.");
            }

        } else {
            all_tags.push(tag_input.to_string());
            println!("Categoría '{tag_input}' creada.");
            break tag_input.to_string();
        }
    };

    tasks.push(Task {
        description: description.to_string(),
        done: false,
        priority,
        tag,
        subtasks: Vec::new(),
    });
    
    stats.added_tasks += 1;
    stats.pending_tasks += 1;

    println!("\nTarea agregada: {description} ({priority:?})");
}

/// ### Muestra la lista completa de tareas.
/// 
/// Mediante iteración, despliega en la consola todas las tareas con su estado, ID, descripción, prioridad, categoría y subtareas. 
pub fn list_tasks(list: &[Task] ) {
    println!("\n LISTA DE TAREAS:");
    for (i, item) in list.iter().enumerate() {
        item.format_task(i+1);
    }
}

/// ### Marca una tarea como completada.
///
/// Actualiza el estado de la tarea y de las subtareas que incluya. También actualiza las estadísticas.
pub fn complete_task(tasks: &mut [Task], id: usize, stats: &mut Statistics) {
    if id > 0 && id <= tasks.len() {
        let task = &mut tasks[id - 1];

        if !task.done {
            task.done = true;
            stats.done_tasks += 1;
            stats.pending_tasks -= 1;
        }
        
        for subtask in &mut task.subtasks {
            subtask.done = true;
        }

        println!("\nTarea {id} completada.");
    } else {
        println!("\nID no válido.");
    }
}

/// ### Elimina tareas. 
///
/// Realiza la eliminación de a una tarea, identificada por su ID. También actualiza las estadísticas correspondientes.
pub fn remove_task(tasks: &mut Vec<Task>, id: usize, stats: &mut Statistics) {
    if id > 0 && id <= tasks.len() {
        let removed_task = tasks.remove(id - 1);
        println!("\nTarea eliminada: {}", removed_task.description);
        stats.removed_tasks += 1;
        
        if removed_task.done {
            stats.done_tasks -= 1;
        } else {
            stats.pending_tasks -= 1;
        }
    } else {
        println!("\nID no válido.");
    }
}

/// ### Permite editar una tarea.
///
/// El usuario puede cambiar la descripción, prioridad, categoría o agregar/completar subtareas.
pub fn edit_task(tasks: &mut [Task], task_id: usize, all_tags: &mut Vec<String>) {
    if task_id == 0 || task_id > tasks.len() {
        println!("ID de tarea no válido.");
        return;
    }

    let task = &mut tasks[task_id - 1];

    loop {
        println!("\nEditando Tarea {}: {}", task_id, task.description);
        println!("Opciones: ");
        println!("\t1. Editar descripción");
        println!("\t2. Editar prioridad ({:?})", task.priority);
        println!("\t3. Editar categoría ({})", task.tag);
        println!("\t4. Agregar subtarea");
        println!("\t5. Completar subtarea");
        println!("\t6. Volver al menú principal");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Inténtelo nuevamente.");
        let input = input.trim();

        match input {
            "1" => {
                println!("Ingrese la nueva descripción:");
                let mut new_desc = String::new();
                io::stdin().read_line(&mut new_desc).expect("Inténtelo nuevamente.");
                task.description = new_desc.trim().to_string();
                println!("Descripción actualizada.");
            },
            "2" => {
                let priority = loop {
                    println!("Ingrese la nueva prioridad (1: Alta, 2: Media, 3: Baja): ");
                    let mut priority_input = String::new();
                    io::stdin().read_line(&mut priority_input).expect("Inténtelo nuevamente.");
                    let priority_input = priority_input.trim();
            
                    match priority_input.parse::<u8>() {
                        Ok(1) => break Priority::Alta,
                        Ok(2) => break Priority::Media,
                        Ok(3) => break Priority::Baja,
                        _ => {
                            println!("Prioridad no válida. Por favor, intente de nuevo.");
                        },
                    }
                };
                task.priority = priority;
                println!("Prioridad actualizada.");
            },
            "3" => {
                println!("\nCategorías disponibles:");
                if all_tags.is_empty() {
                    println!("\t(No hay categorías existentes)");
                } else {
                    for (i, tag) in all_tags.iter().enumerate() {
                        println!("\t{}: {}", i + 1, tag);
                    }
                }
                
                let new_tag = loop {
                    println!("Ingrese el número de la categoría, escriba una nueva, o presione Enter para 'Sin Categoría':");
                    let mut tag_input = String::new();
                    io::stdin().read_line(&mut tag_input).expect("Inténtelo nuevamente.");
                    let tag_input = tag_input.trim();
            
                    if tag_input.is_empty() {
                        break "Sin Categoría".to_string();
                    }
            
                    if let Ok(id) = tag_input.parse::<usize>() {
                        if id > 0 && id <= all_tags.len() {
                            break all_tags[id - 1].clone();
                        } else {
                            println!("ID de categoría no válido.");
                        }
                    } else if !tag_input.is_empty() {
                        all_tags.push(tag_input.to_string());
                        println!("Categoría '{tag_input}' creada.");
                        break tag_input.to_string();
                    } else {
                        println!("El tag no puede estar vacío.");
                    }
                };
                task.tag = new_tag;
                println!("Categoría actualizada.");
            },
            "4" => {
                println!("Ingrese la descripción de la nueva subtarea:");
                let mut subtask_desc = String::new();
                io::stdin().read_line(&mut subtask_desc).expect("Inténtelo nuevamente.");
                let subtask_desc = subtask_desc.trim();
                if !subtask_desc.is_empty() {
                    task.subtasks.push(Subtask { description: subtask_desc.to_string(), done: false });
                    println!("Subtarea agregada.");
                } else {
                    println!("La descripción no puede estar vacía.");
                }
            },
            "5" => {
                println!("Subtareas de Tarea {task_id}:");
                for (sub_id, subtask) in task.subtasks.iter().enumerate() {
                    let sub_status = if subtask.done{"[X]"} else {"[ ]"};
                    println!("\t{} {} {}", sub_status, sub_id + 1, subtask.description);
                }
                println!("Ingrese el ID de la subtarea a completar:");
                let mut subtask_input = String::new();
                io::stdin().read_line(&mut subtask_input).expect("Inténtelo nuevamente.");
                let subtask_id: usize = match subtask_input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("ID de subtarea no válido.");
                        continue;
                    }
                };
                if subtask_id > 0 && subtask_id <= task.subtasks.len() {
                    task.subtasks[subtask_id - 1].done = true;
                    println!("Subtarea completada.");
                } else {
                    println!("ID de subtarea no válido.");
                }
            },
            "6" => break,
            _ => {
                println!("Opción no válida.");
            },
        }
    }
}

/// ### Muestra un reporte de uso.
/// 
/// El reporte incluye la cantidad de tareas agregadas, completadas, pendientes y eliminadas, desde la primera ejecución del gestor de tareas.
pub fn show_stats(stats: &Statistics) {
    println!("\n--- REPORTE DE ESTADÍSTICAS ---");
    println!("Tareas Agregadas: {}", stats.added_tasks);
    println!("Tareas Completadas: {}", stats.done_tasks);
    println!("Tareas Pendientes: {}", stats.pending_tasks);
    println!("Tareas Eliminadas: {}", stats.removed_tasks);
    println!("-------------------------------");
}