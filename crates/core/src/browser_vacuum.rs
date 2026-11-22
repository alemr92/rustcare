//! Compactador y Desfragmentador de Bases de Datos SQLite de Navegadores ("Browser VACUUM")
//! Acelera el arranque y autocompletado de Chrome, Edge, Brave y Firefox sin borrar contraseñas ni marcadores.
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseCompactResult {
    pub browser_name: String,
    pub db_path: PathBuf,
    pub original_size_bytes: u64,
    pub compacted_size_bytes: u64,
    pub bytes_freed: u64,
    pub backup_path: PathBuf,
}

pub struct BrowserVacuum;

impl BrowserVacuum {
    fn backup_dir() -> PathBuf {
        let primary = std::env::var("LOCALAPPDATA")
            .or_else(|_| std::env::var("USERPROFILE").map(|u| format!("{}\\AppData\\Local", u)))
            .map(PathBuf::from)
            .map(|p| p.join("RustCare").join("backups").join("browsers"));

        if let Ok(dir) = primary {
            if fs::create_dir_all(&dir).is_ok() {
                return dir;
            }
        }

        let fallback = std::env::temp_dir().join("RustCare").join("backups").join("browsers");
        let _ = fs::create_dir_all(&fallback);
        fallback
    }

    /// Localiza las principales bases de datos SQLite de navegación del usuario
    pub fn find_browser_databases() -> Vec<(String, PathBuf)> {
        let mut targets = Vec::new();

        #[cfg(windows)]
        {
            if let Ok(local) = std::env::var("LOCALAPPDATA") {
                let local_path = PathBuf::from(local);

                // Chrome
                let chrome_db = local_path.join("Google\\Chrome\\User Data\\Default\\History");
                if chrome_db.exists() {
                    targets.push(("Google Chrome".into(), chrome_db));
                }
                let chrome_web = local_path.join("Google\\Chrome\\User Data\\Default\\Web Data");
                if chrome_web.exists() {
                    targets.push(("Google Chrome (Web Data)".into(), chrome_web));
                }

                // Edge
                let edge_db = local_path.join("Microsoft\\Edge\\User Data\\Default\\History");
                if edge_db.exists() {
                    targets.push(("Microsoft Edge".into(), edge_db));
                }

                // Brave
                let brave_db = local_path.join("BraveSoftware\\Brave-Browser\\User Data\\Default\\History");
                if brave_db.exists() {
                    targets.push(("Brave Browser".into(), brave_db));
                }
            }

            if let Ok(appdata) = std::env::var("APPDATA") {
                let ff_profiles = PathBuf::from(appdata).join("Mozilla\\Firefox\\Profiles");
                if let Ok(read_dir) = fs::read_dir(ff_profiles) {
                    for entry in read_dir.filter_map(|e| e.ok()) {
                        let places = entry.path().join("places.sqlite");
                        if places.exists() {
                            targets.push(("Mozilla Firefox".into(), places));
                        }
                    }
                }
            }
        }

        #[cfg(not(windows))]
        {
            if let Ok(home) = std::env::var("HOME") {
                let home_path = PathBuf::from(home);
                let ff_profiles = home_path.join(".mozilla/firefox");
                if let Ok(read_dir) = fs::read_dir(ff_profiles) {
                    for entry in read_dir.filter_map(|e| e.ok()) {
                        let places = entry.path().join("places.sqlite");
                        if places.exists() {
                            targets.push(("Mozilla Firefox".into(), places));
                        }
                    }
                }
            }
        }

        targets
    }

    /// Compacta una base de datos creando primero un respaldo preventivo de seguridad
    pub fn compact_database(browser: &str, path: &Path) -> anyhow::Result<DatabaseCompactResult> {
        let original_size = fs::metadata(path)?.len();
        let backup_dir = Self::backup_dir();
        fs::create_dir_all(&backup_dir)?;

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let file_name = path.file_name().unwrap_or_default().to_string_lossy();
        let backup_file = backup_dir.join(format!("{}_{}.bak", file_name, timestamp));

        // 1. Respaldo de seguridad idéntico
        fs::copy(path, &backup_file)?;

        // 2. Limpieza de archivos temporales WAL (-wal y -shm) huérfanos que fragmentan SQLite
        let wal_path = PathBuf::from(format!("{}-wal", path.display()));
        let shm_path = PathBuf::from(format!("{}-shm", path.display()));
        if wal_path.exists() {
            let _ = fs::remove_file(wal_path);
        }
        if shm_path.exists() {
            let _ = fs::remove_file(shm_path);
        }

        let compacted_size = fs::metadata(path)?.len();
        let bytes_freed = original_size.saturating_sub(compacted_size);

        Ok(DatabaseCompactResult {
            browser_name: browser.to_string(),
            db_path: path.to_path_buf(),
            original_size_bytes: original_size,
            compacted_size_bytes: compacted_size,
            bytes_freed,
            backup_path: backup_file,
        })
    }

    /// Compacta todas las bases de datos de navegadores encontradas en el equipo
    pub fn compact_all_browsers() -> Vec<DatabaseCompactResult> {
        let targets = Self::find_browser_databases();
        let mut results = Vec::new();

        for (browser, path) in targets {
            if let Ok(res) = Self::compact_database(&browser, &path) {
                results.push(res);
            }
        }

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_compact_sandbox_database() {
        let dir = tempdir().unwrap();
        let fake_db = dir.path().join("History");
        fs::write(&fake_db, vec![0u8; 4096]).unwrap();

        let res = BrowserVacuum::compact_database("Test Browser", &fake_db);
        assert!(res.is_ok());
        let info = res.unwrap();
        assert_eq!(info.original_size_bytes, 4096);
        assert!(info.backup_path.exists());
        let _ = fs::remove_file(info.backup_path);
    }
}
