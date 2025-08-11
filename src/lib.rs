// src/lib.rs

// Declara los módulos que componen tu biblioteca
pub mod app;
pub mod settings;
pub mod storage;

// Exporta la función `run` para que main.rs pueda usarla
pub use app::run;