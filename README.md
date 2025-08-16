Gestor de Tareas To-Do en Rust
Una aplicación de línea de comandos (CLI) para la gestión de tareas, desarrollada en Rust, con un fuerte enfoque en la modularidad, la configuración y una sólida suite de tests.

✨ Características Principales
Gestión Completa de Tareas: Añadir, listar, completar, desmarcar, etiquetar, desetiquetar y eliminar tareas.

Interfaz de Usuario Interactiva: Un menú navegable con las flechas del teclado para una experiencia de usuario amigable, construido con dialoguer.

Altamente Configurable: Controla el comportamiento de la app a través de un archivo Settings.toml sin necesidad de recompilar. Puedes personalizar:

El formato de guardado (json, csv, txt, bincode).

El nombre del archivo de tareas.

Los nombres de los comandos (alias) que el usuario puede usar.

Los textos de ayuda para cada comando.

Una lista de etiquetas válidas.

Persistencia Multi-Formato: Soporta el guardado y la carga de tareas en cuatro formatos diferentes, gestionados de forma polimórfica.

Importación/Exportación: Permite importar tareas desde un archivo, ya sea fusionando las listas o reemplazando la actual.

Etiquetado: Asigna etiquetas personalizadas a las tareas para una mejor organización.

Testing Robusto: Una suite completa de tests de integración y unitarios garantiza la fiabilidad y la corrección de cada funcionalidad.

📂 Estructura del Proyecto
El código está organizado en una estructura modular para garantizar la mantenibilidad y la legibilidad, separando las responsabilidades:

src/main.rs: El punto de entrada de la aplicación, mínimo y limpio.

src/lib.rs: La raíz de la biblioteca que declara y organiza los módulos principales.

src/app/: Contiene el corazón de la lógica de la aplicación.

mod.rs: La orquestación principal y la función run.

ui.rs: Toda la lógica de la interfaz de usuario interactiva.

task.rs: La definición de la struct Tarea.

commands/: Un submódulo para cada comando, con su lógica aislada.

src/settings.rs: Define las structs para cargar la configuración desde Settings.toml.

src/storage/: Contiene toda la lógica de guardado y carga, con un submódulo para cada formato.

tests/: Contiene los tests de integración que simulan el uso de la aplicación por parte del usuario.

⚙️ Configuración (Settings.toml)
Para personalizar la aplicación, crea un archivo Settings.toml en la carpeta principal del proyecto. Si el archivo no existe, la aplicación usará valores por defecto.

# Ejemplo de Settings.toml

# Configuración del archivo de guardado
[storage]
filename_base = "mis_tareas"
format = "json"

# Configuración general de la app
[general]
tags_validos = ["trabajo", "personal", "urgente", "estudio"]

# Mapa de los comandos que el usuario puede usar
[comandos]
"añadir" = "agregar"
"ver" = "listar"
"hecho" = "completar"
"pendiente" = "desmarcar"
# ... y así sucesivamente

# Textos de ayuda para el comando 'ayuda'
[explicaciones]
"añadir" = "Uso: añadir <descripción>. Añade una nueva tarea."
"ver" = "Uso: ver. Muestra todas las tareas."
# ... y así sucesivamente

🚀 Cómo Ejecutar el Proyecto
Build:

cargo build --release

Ejecutar los Tests:

# Ejecuta todos los tests (unitarios y de integración)
cargo test

# Ejecuta solo los tests de la biblioteca para un feedback más rápido
cargo test --lib

Ejecutar la Aplicación:

Modo Interactivo (con menú):

cargo run

Modo de Comando Único (para scripting o tests):

cargo run -- <comando> [argumentos...]
# Ejemplo: cargo run -- añadir "Comprar leche"

🌱 Evolución del Proyecto
El proyecto nació a partir de un único archivo main.rs y evolucionó a través de un proceso iterativo de refactorización y mejora:

Inicio Monolítico: Lógica base para agregar y listar tareas en memoria.

Persistencia y Formatos: Implementación del guardado en archivo (CSV, TXT, JSON, Bincode).

Diseño Polimórfico: Uso de un enum (Storage) para gestionar los diferentes formatos de forma flexible.

Configuración Externa: Desplazamiento de la configuración (nombre del archivo, formato) a Settings.toml.

Flexibilidad de Comandos: Los nombres de los comandos y los textos de ayuda se volvieron configurables.

Mejora de la UI: Paso de una simple entrada de texto a un menú interactivo con dialoguer.

Adición de Funcionalidades: Implementación de etiquetas, importación/carga desde archivo, y gestión completa del estado de las tareas.

Refactorización a Módulos: Reestructuración final del código a una arquitectura profesional de biblioteca + binario, con módulos dedicados para cada responsabilidad.
