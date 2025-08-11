use crate::app::task::Tarea;
use std::error::Error;
use std::fs;

pub struct BincodeStorage;
impl BincodeStorage {
    pub fn save(&self, file_path: &str, tasks: &Vec<Tarea>) -> Result<(), Box<dyn Error>> {
        let config = bincode::config::standard();
        let file = fs::File::create(file_path)?;
        bincode::serde::encode_into_std_write(tasks, &mut std::io::BufWriter::new(file), config)?;
        Ok(())
    }
    pub fn load(&self, file_path: &str) -> Result<Vec<Tarea>, Box<dyn Error>> {
        if !fs::metadata(file_path).is_ok() { return Ok(Vec::new()); }
        let config = bincode::config::standard();
        let file = fs::File::open(file_path)?;
        let tasks = bincode::serde::decode_from_std_read(&mut std::io::BufReader::new(file), config)?;
        Ok(tasks)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::task::Tarea;

    #[test]
    fn test_guardar_y_cargar_bincode() {
        let storage = BincodeStorage;
        let file_path = "test_temporal_tareas.bincode";

        // Limpiamos el archivo antes y después
        let _ = std::fs::remove_file(file_path);

        let tareas_originales = vec![
            Tarea { descripcion: "Tarea Bincode 1".to_string(), completada: false, ultimo_cambio: None },
            Tarea { descripcion: "Tarea Bincode 2".to_string(), completada: true, ultimo_cambio: Some(chrono::Utc::now()) },
        ];

        // Probamos guardar
        assert!(storage.save(file_path, &tareas_originales).is_ok());

        // Probamos cargar
        let tareas_cargadas = storage.load(file_path).unwrap();

        // Verificamos que los datos coincidan
        assert_eq!(tareas_originales, tareas_cargadas);

        // Limpiamos al final
        let _ = std::fs::remove_file(file_path);
    }
}