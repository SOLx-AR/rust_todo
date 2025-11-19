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
            let tags_str = task.tags.join(";");
            file_content.push_str(&format!("{},{},{},{}\n", status, timestamp_str, task.descripcion, tags_str));
        }
        Ok(fs::write(file_path, file_content)?)
    }
    
    pub fn load(&self, file_path: &str) -> Result<Vec<Tarea>, Box<dyn Error>> {
        if !fs::metadata(file_path).is_ok() { return Ok(Vec::new()); }
        let file_content = fs::read_to_string(file_path)?;
        let mut tasks = Vec::new();
        for line in file_content.lines() {
            let mut parts = line.splitn(4, ',');
            if let (Some(status_str), Some(timestamp_str), Some(description), Some(tags_str)) = (parts.next(), parts.next(), parts.next(), parts.next()) {
                let timestamp_opt = if timestamp_str.is_empty() { None } else { DateTime::parse_from_rfc3339(timestamp_str).ok().map(|dt| dt.with_timezone(&Utc)) };
                let tags = if tags_str.is_empty() { Vec::new() } else { tags_str.split(';').map(String::from).collect() };
                tasks.push(Tarea {
                    descripcion: description.to_string(),
                    completada: status_str == "completada",
                    ultimo_cambio: timestamp_opt,
                    tags,
                });
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
        let _ = std::fs::remove_file(file_path);
        let tareas_originales = vec![
            Tarea { descripcion: "Tarea TXT 1".to_string(), completada: false, ultimo_cambio: None, tags: Vec::new() },
            Tarea { descripcion: "Tarea TXT 2".to_string(), completada: true, ultimo_cambio: Some(chrono::Utc::now()), tags: vec!["tag1".to_string(), "tag2".to_string()] },
        ];
        assert!(storage.save(file_path, &tareas_originales).is_ok());
        let tareas_cargadas = storage.load(file_path).unwrap();
        assert_eq!(tareas_originales.len(), tareas_cargadas.len());
        assert_eq!(tareas_originales[0].descripcion, tareas_cargadas[0].descripcion);
        assert_eq!(tareas_originales[1].tags, tareas_cargadas[1].tags);
        let _ = std::fs::remove_file(file_path);
    }
}