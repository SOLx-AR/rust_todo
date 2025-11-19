mod app;
mod settings;
mod storage;

fn main() {
    if let Err(e) = app::run() {
        eprintln!("Error en la aplicación: {}", e);
        std::process::exit(1);
    }
}