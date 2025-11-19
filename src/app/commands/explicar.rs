use crate::settings::Settings;

pub fn ejecutar(args: &str, settings: &Settings) -> bool {
    if !args.is_empty() {
        if let Some(explicacion) = settings.explicaciones.get(args) {
            println!("\nAyuda para '{}':\n{}", args, explicacion);
        } else {
            println!("No encontré ayuda para el comando '{}'.", args);
        }
    } else {
        println!("\nComandos disponibles:");
        let mut comandos_ordenados: Vec<_> = settings.comandos.keys().collect();
        comandos_ordenados.sort();
        for alias in comandos_ordenados {
            println!("- {}", alias);
        }
        println!("\nEscribí 'ayuda <comando>' para más detalles.");
    }
    false
}