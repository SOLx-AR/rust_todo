use crate::app::task::Tarea;
use std::error::Error;
use std::fs;
use chrono::{DateTime, Utc};

pub struct CsvStorage;
impl CsvStorage {
    pub fn save(&self, file_path: &str, tasks: &Vec<Tarea>) -> Result<(), Box<dyn Error>> {
        let mut writer = csv::Writer::from_path(file_path)?;
        writer.write_record(&["completada", "ultimo_cambio", "descripcion", "tags"])?;
        for task in tasks {
            let timestamp_str = task.ultimo_cambio.map_or(String::new(), |ts| ts.to_rfc3339());
            let tags_str = task.tags.join(";");
            writer.write_record(&[
                task.completada.to_string(),
                timestamp_str,
                task.descripcion.clone(),
                tags_str,
            ])?;
        }
        Ok(writer.flush()?)
    }
    
    pub fn load(&self, file_path: &str) -> Result<Vec<Tarea>, Box<dyn Error>> {
        if !fs::metadata(file_path).is_ok() { return Ok(Vec::new()); }
        let file = fs::File::open(file_path)?;
        let mut reader = csv::ReaderBuilder::new().has_headers(true).from_reader(file);
        let mut tasks = Vec::new();
        for result in reader.records() {
            let record = result?;
            let tags = record.get(3).map_or(Vec::new(), |s| {
                if s.is_empty() { Vec::new() } else { s.split(';').map(String::from).collect() }
            });
            tasks.push(Tarea {
                completada: record.get(0).unwrap_or("").parse().unwrap_or(false),
                ultimo_cambio: record.get(1).and_then(|s| DateTime::parse_from_rfc3339(s).ok()).map(|dt| dt.with_timezone(&Utc)),
                descripcion: record.get(2).unwrap_or("").to_string(),
                tags,
            });
        }
        Ok(tasks)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::task::Tarea;

    #[test]
    fn test_guardar_y_cargar_csv() {
        let storage = CsvStorage;
        let file_path = "test_temporal_tareas.csv";
        let _ = std::fs::remove_file(file_path);
        let tareas_originales = vec![
            Tarea { descripcion: "Tarea CSV 1".to_string(), completada: false, ultimo_cambio: None, tags: Vec::new() },
            Tarea { descripcion: "Tarea CSV 2".to_string(), completada: true, ultimo_cambio: Some(chrono::Utc::now()), tags: vec!["tag1".to_string(), "tag2".to_string()] },
        ];
        assert!(storage.save(file_path, &tareas_originales).is_ok());
        let tareas_cargadas = storage.load(file_path).unwrap();
        assert_eq!(tareas_originales.len(), tareas_cargadas.len());
        assert_eq!(tareas_originales[0].descripcion, tareas_cargadas[0].descripcion);
        assert_eq!(tareas_originales[1].tags, tareas_cargadas[1].tags);
        let _ = std::fs::remove_file(file_path);
    }
}