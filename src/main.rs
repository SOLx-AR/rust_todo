// src/main.rs

// Declaramos los módulos que ahora forman nuestra biblioteca de lógica
mod app;
mod settings;
mod storage;

fn main() {
    // Llamamos a la función principal de nuestra app
    if let Err(e) = app::run() {
        eprintln!("Error en la aplicación: {}", e);
        std::process::exit(1);
    }
}