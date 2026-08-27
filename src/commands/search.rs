use anyhow::{Result, bail};
use std::path::Path;

pub fn search(filename: String) -> Result<()> {
    let path = Path::new(&filename);

    if !path.exists() {
        bail!("El archivo o ruta '{}' no existe.", filename);
    }

    if !path.is_file() {
        bail!("La ruta '{}' es un directorio, no un archivo.", filename);
    }

    println!("Archivo encontrado: {}", path.display());

    Ok(())
}
