//! ## Estructuras de datos principales. 
//!
//! Las prioridades de las tareas se almacenan en un **enum**. y las tareas, subtareas y estadísticas de uso, se definen como **structs**.
//! 
//! Todas las estructuras son serializables y deserializables para su persistencia en .JSON.

use serde::{Serialize, Deserialize};

/// ### Tarea principal.
///
/// Contiene una descripción, estado de completitud, prioridad, una categoría (tag) y un vector de subtareas.
#[derive(Serialize, Deserialize, Clone)]
pub struct Task {
    pub description: String,
    pub done: bool,
    pub priority: Priority,
    pub tag: String,
    pub subtasks: Vec<Subtask>,
}

/// ### Subtarea.
///  
/// Es parte de una tarea principal.
#[derive(Serialize, Deserialize, Clone)]
pub struct Subtask {
    pub description: String,
    pub done: bool,
}

/// ### Estadísticas de uso.
///
/// Datos para generar reportes sobre el uso de la aplicación.
#[derive(Serialize, Deserialize, Default)]
pub struct Statistics {
    pub added_tasks: usize,
    pub done_tasks: usize,
    pub pending_tasks: usize,
    pub removed_tasks: usize,
}

/// ### Prioridad.
/// 
/// Define los niveles de prioridad estandarizados que el usuario puede elegir.
#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum Priority {
    Alta,
    Media,
    Baja,
}

/// **Formatea e imprime una tarea.**
///
/// Muestra el estado, ID, descripción, prioridad, categoría y subtareas identadas, si existen.
impl Task {
    pub fn format_task(&self, id:usize){
        let status = if self.done{"[X]"} else {"[ ]"};
        println!("{} {}: {} ({:?}) - {}", status, id, self.description, self.priority, self.tag);
        for (sub_id, subtask) in self.subtasks.iter().enumerate() {
            let sub_status = if subtask.done{"[X]"} else {"[ ]"};
            println!("\t{} {}.{} {}", sub_status, id, sub_id + 1, subtask.description);
        }
    }
}