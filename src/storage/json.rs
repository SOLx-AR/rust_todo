use crate::app::task::Tarea;
use std::error::Error;
use std::fs;

pub struct JsonStorage;
impl JsonStorage {
    pub fn save(&self, file_path: &str, tasks: &Vec<Tarea>) -> Result<(), Box<dyn Error>> {
        let json_str = serde_json::to_string_pretty(tasks)?;
        Ok(fs::write(file_path, json_str)?)
    }
    pub fn load(&self, file_path: &str) -> Result<Vec<Tarea>, Box<dyn Error>> {
        if !fs::metadata(file_path).is_ok() { return Ok(Vec::new()); }
        let json_str = fs::read_to_string(file_path)?;
        if json_str.trim().is_empty() { return Ok(Vec::new()); }
        Ok(serde_json::from_str(&json_str)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::task::Tarea;

    #[test]
    fn test_guardar_y_cargar_json() {
        let storage = JsonStorage;
        let file_path = "test_temporal_tareas.json";
        let _ = std::fs::remove_file(file_path);
        let tareas_originales = vec![
            Tarea { descripcion: "Tarea JSON 1".to_string(), completada: false, ultimo_cambio: None, tags: Vec::new() },
            Tarea { descripcion: "Tarea JSON 2".to_string(), completada: true, ultimo_cambio: Some(chrono::Utc::now()), tags: vec!["test".to_string()] },
        ];
        assert!(storage.save(file_path, &tareas_originales).is_ok());
        let tareas_cargadas = storage.load(file_path).unwrap();
        assert_eq!(tareas_originales, tareas_cargadas);
        let _ = std::fs::remove_file(file_path);
    }
}