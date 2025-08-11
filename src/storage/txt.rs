
use crate::app::task::Tarea;
use std::error::Error;
use std::fs;
use chrono::{DateTime, Utc};

pub struct TxtStorage;
impl TxtStorage {
    pub fn save(&self, file_path: &str, tasks: &Vec<Tarea>) -> Result<(), Box<dyn Error>> {
        let mut file_content = String::new();
        for task in tasks {
            let status = if task.completada { "completada" } else { "no completada" };
            let timestamp_str = task.ultimo_cambio.map_or(String::new(), |ts| ts.to_rfc3339());
            file_content.push_str(&format!("{},{},{}\n", status, timestamp_str, task.descripcion));
        }
        Ok(fs::write(file_path, file_content)?)
    }
    pub fn load(&self, file_path: &str) -> Result<Vec<Tarea>, Box<dyn Error>> {
        if !fs::metadata(file_path).is_ok() { return Ok(Vec::new()); }
        let file_content = fs::read_to_string(file_path)?;
        let mut tasks = Vec::new();
        for line in file_content.lines() {
            if let Some((status_str, rest)) = line.split_once(',') {
                if let Some((timestamp_str, description)) = rest.split_once(',') {
                    let timestamp_opt = if timestamp_str.is_empty() {
                        None
                    } else {
                        DateTime::parse_from_rfc3339(timestamp_str).ok().map(|dt| dt.with_timezone(&Utc))
                    };
                    tasks.push(Tarea {
                        descripcion: description.to_string(),
                        completada: status_str == "completada",
                        ultimo_cambio: timestamp_opt,
                    });
                }
            }
        }
        Ok(tasks)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::task::Tarea;

    #[test]
    fn test_guardar_y_cargar_txt() {
        let storage = TxtStorage;
        let file_path = "test_temporal_tareas.txt";
        
        // Limpiamos el archivo antes y después
        let _ = std::fs::remove_file(file_path);

        let tareas_originales = vec![
            Tarea { descripcion: "Tarea TXT 1".to_string(), completada: false, ultimo_cambio: None },
            Tarea { descripcion: "Tarea TXT 2".to_string(), completada: true, ultimo_cambio: Some(chrono::Utc::now()) },
        ];

        // Probamos guardar
        assert!(storage.save(file_path, &tareas_originales).is_ok());

        // Probamos cargar
        let tareas_cargadas = storage.load(file_path).unwrap();
        
        // Verificamos que los datos coincidan (ignorando la precisión de nanosegundos del timestamp)
        assert_eq!(tareas_originales.len(), tareas_cargadas.len());
        assert_eq!(tareas_originales[0].descripcion, tareas_cargadas[0].descripcion);
        assert_eq!(tareas_originales[1].descripcion, tareas_cargadas[1].descripcion);

        // Limpiamos al final
        let _ = std::fs::remove_file(file_path);
    }
}