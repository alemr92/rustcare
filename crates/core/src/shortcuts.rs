//! Limpiador de Accesos Directos Rotos (.lnk)
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrokenShortcut {
    pub shortcut_path: PathBuf,
    pub target_path: String,
    pub file_name: String,
}

pub struct ShortcutCleaner;

impl ShortcutCleaner {
    /// Extrae la ruta de destino de un fichero .lnk de Windows (MS-SHLLINK Specification)
    pub fn parse_target_path<P: AsRef<Path>>(lnk_path: P) -> Option<String> {
        let mut file = File::open(lnk_path).ok()?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).ok()?;

        if buffer.len() < 0x4C {
            return None;
        }

        // HeaderSize debe ser 0x0000004C
        if buffer[0..4] != [0x4C, 0x00, 0x00, 0x00] {
            return None;
        }

        let flags = u32::from_le_bytes([buffer[0x14], buffer[0x15], buffer[0x16], buffer[0x17]]);
        let has_link_info = (flags & 0x02) != 0;
        let _has_relative_path = (flags & 0x08) != 0;

        // Intentar leer la sección LinkInfo si existe
        if has_link_info {
            let mut offset = 0x4C;

            // Si tiene LinkTargetIDList (bit 0), saltar su longitud
            if (flags & 0x01) != 0 {
                if buffer.len() >= offset + 2 {
                    let id_list_size = u16::from_le_bytes([buffer[offset], buffer[offset + 1]]) as usize;
                    offset += 2 + id_list_size;
                }
            }

            if buffer.len() >= offset + 0x1C {
                let link_info_size = u32::from_le_bytes([buffer[offset], buffer[offset + 1], buffer[offset + 2], buffer[offset + 3]]) as usize;
                let local_base_path_offset = u32::from_le_bytes([buffer[offset + 0x10], buffer[offset + 0x11], buffer[offset + 0x12], buffer[offset + 0x13]]) as usize;

                if local_base_path_offset > 0 && local_base_path_offset < link_info_size {
                    let path_start = offset + local_base_path_offset;
                    if path_start < buffer.len() {
                        let path_bytes = &buffer[path_start..];
                        if let Some(null_pos) = path_bytes.iter().position(|&b| b == 0) {
                            if let Ok(path_str) = std::str::from_utf8(&path_bytes[..null_pos]) {
                                if !path_str.trim().is_empty() {
                                    return Some(path_str.to_string());
                                }
                            }
                        }
                    }
                }
            }
        }

        // Alternativamente, buscar cadenas ASCII de rutas típicas "X:\" dentro del binario
        for i in 0..buffer.len().saturating_sub(4) {
            if buffer[i].is_ascii_alphabetic() && buffer[i + 1] == b':' && buffer[i + 2] == b'\\' {
                let slice = &buffer[i..];
                if let Some(end) = slice.iter().position(|&b| b == 0 || b < 32 || b > 126) {
                    if let Ok(path_str) = std::str::from_utf8(&slice[..end]) {
                        if path_str.len() >= 3 && (path_str.ends_with(".exe") || path_str.ends_with(".bat") || path_str.ends_with(".cmd") || path_str.contains('\\')) {
                            return Some(path_str.to_string());
                        }
                    }
                }
            }
        }

        None
    }

    /// Directorios habituales donde se acumulan accesos directos
    pub fn default_shortcut_directories() -> Vec<PathBuf> {
        let mut dirs = Vec::new();

        if let Ok(userprofile) = env::var("USERPROFILE") {
            dirs.push(PathBuf::from(&userprofile).join("Desktop"));
        }
        if let Ok(public) = env::var("PUBLIC") {
            dirs.push(PathBuf::from(&public).join("Desktop"));
        }
        if let Ok(appdata) = env::var("APPDATA") {
            dirs.push(PathBuf::from(&appdata).join("Microsoft").join("Windows").join("Start Menu").join("Programs"));
        }

        dirs.into_iter().filter(|d| d.exists()).collect()
    }

    /// Escanea las rutas en busca de accesos directos cuyo destino ya no existe en el disco
    pub fn scan_broken_shortcuts() -> Vec<BrokenShortcut> {
        let mut broken = Vec::new();

        for dir in Self::default_shortcut_directories() {
            for entry in WalkDir::new(dir)
                .follow_links(false)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                if entry.file_type().is_file() {
                    let path = entry.path();
                    if path.extension().map(|e| e.to_string_lossy().to_lowercase()) == Some("lnk".to_string()) {
                        if let Some(target) = Self::parse_target_path(path) {
                            let target_path = Path::new(&target);
                            // Si la ruta tiene formato de disco local ("C:\...") y no existe en el sistema
                            if target.len() >= 3 && target.chars().nth(1) == Some(':') {
                                if !target_path.exists() {
                                    broken.push(BrokenShortcut {
                                        shortcut_path: path.to_path_buf(),
                                        target_path: target,
                                        file_name: path.file_name().unwrap_or_default().to_string_lossy().to_string(),
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }

        broken
    }

    /// Elimina un acceso directo roto
    pub fn remove_broken_shortcut(item: &BrokenShortcut) -> anyhow::Result<()> {
        fs::remove_file(&item.shortcut_path)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shortcut_directories() {
        let dirs = ShortcutCleaner::default_shortcut_directories();
        assert!(!dirs.is_empty());
    }
}
