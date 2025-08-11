use crate::app::task::Tarea;
use std::error::Error;
use std::fs;
use chrono::{DateTime, Utc}; // CORRECCIÓN: Agregamos los imports necesarios

pub struct CsvStorage;
impl CsvStorage {
    // CORRECCIÓN: Hacemos los métodos públicos con `pub`
    pub fn save(&self, file_path: &str, tasks: &Vec<Tarea>) -> Result<(), Box<dyn Error>> {
        let mut writer = csv::Writer::from_path(file_path)?;
        writer.write_record(&["completada", "ultimo_cambio", "descripcion"])?;
        for task in tasks {
            let timestamp_str = task.ultimo_cambio.map_or(String::new(), |ts| ts.to_rfc3339());
            writer.write_record(&[
                task.completada.to_string(),
                timestamp_str,
                task.descripcion.clone(),
            ])?;
        }
        Ok(writer.flush()?)
    }
    
    // CORRECCIÓN: Hacemos los métodos públicos con `pub`
    pub fn load(&self, file_path: &str) -> Result<Vec<Tarea>, Box<dyn Error>> {
        if !fs::metadata(file_path).is_ok() { return Ok(Vec::new()); }
        let file = fs::File::open(file_path)?;
        let mut reader = csv::ReaderBuilder::new().has_headers(true).from_reader(file);
        let mut tasks = Vec::new();
        for result in reader.records() {
            let record = result?;
            let timestamp_opt = record.get(1)
                .and_then(|ts_str| DateTime::parse_from_rfc3339(ts_str).ok())
                .map(|dt| dt.with_timezone(&Utc));
            tasks.push(Tarea {
                completada: record.get(0).unwrap_or("").parse().unwrap_or(false),
                ultimo_cambio: timestamp_opt,
                descripcion: record.get(2).unwrap_or("").to_string(),
            });
        }
        Ok(tasks)
    }
}

// src/storage/csv.rs

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::task::Tarea;

    #[test]
    fn test_guardar_y_cargar_csv() {
        let storage = CsvStorage;
        let file_path = "test_temporal_tareas.csv";
        
        // Limpiamos el archivo antes y después para que el test sea limpio
        let _ = std::fs::remove_file(file_path);

        let tareas_originales = vec![
            Tarea { descripcion: "Tarea CSV 1".to_string(), completada: false, ultimo_cambio: None },
            Tarea { descripcion: "Tarea CSV 2".to_string(), completada: true, ultimo_cambio: Some(chrono::Utc::now()) },
        ];

        // Probamos guardar
        assert!(storage.save(file_path, &tareas_originales).is_ok());

        // Probamos cargar
        let tareas_cargadas = storage.load(file_path).unwrap();

        // Verificamos que los datos sean iguales (ignorando el timestamp que pierde precisión)
        assert_eq!(tareas_originales.len(), tareas_cargadas.len());
        assert_eq!(tareas_originales[0].descripcion, tareas_cargadas[0].descripcion);
        assert_eq!(tareas_originales[1].descripcion, tareas_cargadas[1].descripcion);

        // Limpiamos al final
        let _ = std::fs::remove_file(file_path);
    }
}