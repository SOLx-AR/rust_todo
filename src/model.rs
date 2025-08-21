use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fs::File;
use std::io::Write;
use uuid::Uuid;

const ARCHIVO_TAREAS: &str = "tareas.json";

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct Tarea {
    pub id: String,
    pub parent_id: Option<String>,
    pub descripcion: String,
    pub completada: bool,
    pub etiquetas: Vec<String>,
    pub prioridad: u32,
}

impl Ord for Tarea {
    fn cmp(&self, other: &Self) -> Ordering {
        // Not completed tasks first
        self.completada
            .cmp(&other.completada)
            // Then by priority, lower first
            .then_with(|| self.prioridad.cmp(&other.prioridad))
    }
}

impl PartialOrd for Tarea {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Tarea {
    pub fn mostrar(&self, id: usize, indent: usize) {
        let prefix = "  ".repeat(indent);
        let estado = if self.completada { "[X]" } else { "[ ]" };
        print!("{}{} {}: {}", prefix, estado, id, self.descripcion);
        if self.prioridad != 0 {
            print!(" - prioridad: {} ", self.prioridad);
        }
        if !self.etiquetas.is_empty() {
            print!(" (etiquetas: {})", self.etiquetas.join(", "));
        }
        print!("\n");
    }

    pub fn etiquetar(&mut self, etiqueta: &str) {
        self.etiquetas.push(String::from(etiqueta));
    }

    pub fn new(descripcion: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            parent_id: None,
            descripcion: descripcion.to_string(),
            completada: false,
            etiquetas: Vec::from([]),
            prioridad: 0,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Tareas {
    pub tareas: Vec<Tarea>,
}

impl Tareas {
    pub fn new() -> Tareas {
        Tareas { tareas: Vec::new() }
    }

    pub fn agregar(&mut self, descripcion: &str) {
        if descripcion.is_empty() {
            println!("\nLa descripción de la tarea no puede estar vacía.");
        } else {
            self.tareas.push(Tarea::new(descripcion));
            println!("\nTarea agregada: {}", descripcion);
        }
    }

    pub fn agregar_subtarea(&mut self, parent_pos_id: usize, descripcion: &str) {
        if descripcion.is_empty() {
            println!("\nLa descripción de la tarea no puede estar vacía.");
            return;
        }
        if parent_pos_id > 0 && parent_pos_id <= self.tareas.len() {
            let parent_uuid = self.tareas[parent_pos_id - 1].id.clone();
            let mut new_task = Tarea::new(descripcion);
            new_task.parent_id = Some(parent_uuid);
            self.tareas.push(new_task);
            println!("\nSubtarea agregada: {}", descripcion);
        } else {
            println!("\nID de tarea padre no válido.");
        }
    }

    pub fn priorizar(&mut self, id: usize, prioridad: u32) {
        if id > 0 && id <= self.tareas.len() {
            self.tareas[id - 1].prioridad = prioridad;
            println!("\nTarea {} priorizada con {}.", id, prioridad);
        } else {
            println!("\nID de tarea no válido.");
        }
    }

    pub fn completar(&mut self, id: usize) {
        if id > 0 && id <= self.tareas.len() {
            self.tareas[id - 1].completada = true;
            println!("\nTarea {} marcada como completada.", id);
        } else {
            println!("\nID de tarea no válido.");
        }
    }

    pub fn etiquetar(&mut self, id: usize, etiqueta: &str) {
        if id > 0 && id <= self.tareas.len() {
            self.tareas[id - 1].etiquetar(etiqueta);
            println!("\nTarea {} etiquetada con {}.", id, etiqueta);
        } else {
            println!("\nID de tarea no válido.");
        }
    }

    pub fn listar(&self) {
        println!("\nLista de Tareas:");
        let mut top_level_tasks: Vec<_> = self
            .tareas
            .iter()
            .enumerate()
            .filter(|(_, t)| t.parent_id.is_none())
            .collect();

        top_level_tasks.sort_by(|(_, a), (_, b)| a.cmp(b));

        for (i, tarea) in top_level_tasks {
            tarea.mostrar(i + 1, 0);
            self.mostrar_subtareas(&tarea.id, 1);
        }
    }

    fn mostrar_subtareas(&self, parent_id: &str, indent: usize) {
        let mut sub_tasks: Vec<_> = self
            .tareas
            .iter()
            .enumerate()
            .filter(|(_, t)| t.parent_id.as_deref() == Some(parent_id))
            .collect();

        sub_tasks.sort_by(|(_, a), (_, b)| a.cmp(b));

        for (i, tarea) in sub_tasks {
            tarea.mostrar(i + 1, indent);
            self.mostrar_subtareas(&tarea.id, indent + 1);
        }
    }

    pub fn guardar(&self) {
        let json =
            serde_json::to_string_pretty(&self.tareas).expect("No se pudo serializar las tareas");
        let mut archivo = File::create(ARCHIVO_TAREAS).expect("No se pudo crear el archivo");
        archivo
            .write_all(json.as_bytes())
            .expect("No se pudo escribir en el archivo");
    }

    pub fn cargar() -> Tareas {
        match File::open(ARCHIVO_TAREAS) {
            Ok(archivo) => match serde_json::from_reader(archivo) {
                Ok(tareas) => Tareas { tareas },
                Err(_) => Tareas::new(),
            },
            Err(_) => Tareas::new(),
        }
    }
}
