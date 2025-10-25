use crate::tareas::Tareas;
use std::fs;

pub fn guardar(tareas: &Tareas, archivo: &str) -> Result<(), String> {
    let datos = serde_json::to_string_pretty(tareas).map_err(|e| e.to_string())?;
    fs::write(archivo, datos).map_err(|e| e.to_string())
}

pub fn cargar(archivo: &str) -> Tareas {
    match fs::read_to_string(archivo) {
        Ok(contenido) => serde_json::from_str(&contenido).unwrap_or_default(),
        Err(_) => Tareas::default(),
    }
}