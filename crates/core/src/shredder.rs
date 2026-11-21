//! Destructor Seguro de Archivos (File Shredder)
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;

pub struct FileShredder;

impl FileShredder {
    /// Destruye un archivo sobrescribiéndolo con ceros y forzando el volcado físico a disco
    /// antes de borrarlo del sistema de archivos, impidiendo su recuperación forense.
    pub fn shred_file<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
        let p = path.as_ref();
        let metadata = fs::metadata(p)?;
        let file_size = metadata.len();

        if file_size > 0 {
            let mut file = OpenOptions::new()
                .write(true)
                .open(p)?;

            // Sobrescribir en bloques de 64 KB con ceros
            let chunk_size = 65536usize;
            let zero_buffer = vec![0u8; chunk_size];
            let mut remaining = file_size;

            while remaining > 0 {
                let to_write = std::cmp::min(remaining as usize, chunk_size);
                file.write_all(&zero_buffer[..to_write])?;
                remaining -= to_write as u64;
            }

            // Sincronizar cambios en los platos del disco / celda NAND física
            file.sync_all()?;

            // Truncar longitud a 0
            file.set_len(0)?;
            file.sync_all()?;
        }

        fs::remove_file(p)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_file_shredder() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("secret.txt");
        fs::write(&file_path, b"Super confidential financial records").unwrap();
        assert!(file_path.exists());

        FileShredder::shred_file(&file_path).unwrap();
        assert!(!file_path.exists());
    }
}
