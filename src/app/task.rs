// src/app/task.rs

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tarea {
    pub descripcion: String,
    pub completada: bool,
    pub ultimo_cambio: Option<DateTime<Utc>>,
}

impl Tarea {
    pub fn mostrar(&self, id: usize) {
        let estado = if self.completada { "[X]" } else { "[ ]" };
        let timestamp_str = if self.completada {
            self.ultimo_cambio.map_or("".to_string(), |ts| format!("({}) ", ts.format("%d/%m/%Y %H:%M:%S")))
        } else {
            "".to_string()
        };
        println!("{} {}{}: {}", estado, timestamp_str, id, self.descripcion);
    }
}