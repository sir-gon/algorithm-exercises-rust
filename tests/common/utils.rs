use serde::de::DeserializeOwned;
use std::fs;

/// Carga un archivo JSON en un `Vec<T>`.
pub fn load_json<T: DeserializeOwned>(path: &str) -> Vec<T> {
    let contents = fs::read_to_string(path)
        .unwrap_or_else(|_| panic!("No se pudo leer el archivo: {}", path));
    serde_json::from_str(&contents)
        .unwrap_or_else(|_| panic!("JSON inválido en {}", path))
}
