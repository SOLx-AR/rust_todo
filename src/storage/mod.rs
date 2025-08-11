use crate::app::task::Tarea;
use std::error::Error;

pub mod bincode;
pub mod csv;
pub mod json;
pub mod txt;

pub use bincode::BincodeStorage;
pub use csv::CsvStorage;
pub use json::JsonStorage;
pub use txt::TxtStorage;

pub enum Storage {
    Csv(CsvStorage),
    Json(JsonStorage),
    Txt(TxtStorage),
    Bincode(BincodeStorage),
}

impl Storage {
    pub fn save(&self, file_path: &str, tasks: &Vec<Tarea>) -> Result<(), Box<dyn Error>> {
        match self {
            Storage::Csv(s) => s.save(file_path, tasks),
            Storage::Json(j) => j.save(file_path, tasks),
            Storage::Txt(t) => t.save(file_path, tasks),
            Storage::Bincode(b) => b.save(file_path, tasks),
        }
    }
    pub fn load(&self, file_path: &str) -> Result<Vec<Tarea>, Box<dyn Error>> {
        match self {
            Storage::Csv(s) => s.load(file_path),
            Storage::Json(j) => j.load(file_path),
            Storage::Txt(t) => t.load(file_path),
            Storage::Bincode(b) => b.load(file_path),
        }
    }
}