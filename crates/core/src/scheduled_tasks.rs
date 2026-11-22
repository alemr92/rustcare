//! Auditor de Tareas Programadas Huérfanas de Windows
//! Identifica tareas de Task Scheduler que apuntan a binarios o desinstaladores que ya no existen en disco.
use serde::{Deserialize, Serialize};
use std::path::Path;
#[cfg_attr(not(windows), allow(unused_imports))]
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledTaskItem {
    pub task_name: String,
    pub action_exec: String,
    pub is_orphaned: bool,
    pub status: String,
}

pub struct ScheduledTaskManager;

impl ScheduledTaskManager {
    /// Lista las tareas programadas y analiza si su binario ejecutable existe en disco
    pub fn list_tasks() -> Vec<ScheduledTaskItem> {
        let mut tasks = Vec::new();

        #[cfg(windows)]
        {
            let output = Command::new("schtasks")
                .args(["/query", "/fo", "CSV", "/v"])
                .output();

            if let Ok(out) = output {
                let stdout = String::from_utf8_lossy(&out.stdout);
                for line in stdout.lines().skip(1) {
                    // Los campos CSV contienen comillas: "NombreDeTarea","Hora","Estado",...
                    let fields: Vec<String> = line
                        .split("\",\"")
                        .map(|s| s.trim_matches('"').to_string())
                        .collect();

                    if fields.len() >= 9 {
                        let task_name = fields[0].clone();
                        let status = fields[3].clone();
                        // En formato detallado /v, el comando a ejecutar suele estar en campos posteriores
                        let action_raw = fields.get(8).cloned().unwrap_or_default();

                        if action_raw.is_empty() || action_raw.starts_with("N/D") || action_raw == "COM handler" {
                            continue;
                        }

                        // Extrae la ruta del ejecutable eliminando argumentos
                        let clean_path = Self::extract_executable_path(&action_raw);
                        let is_orphaned = if let Some(ref path_str) = clean_path {
                            let p = Path::new(path_str);
                            !p.exists() && (path_str.ends_with(".exe") || path_str.ends_with(".bat") || path_str.ends_with(".cmd") || path_str.ends_with(".vbs"))
                        } else {
                            false
                        };

                        tasks.push(ScheduledTaskItem {
                            task_name,
                            action_exec: action_raw,
                            is_orphaned,
                            status,
                        });
                    }
                }
            }
        }

        tasks
    }

    /// Extrae la ruta de un ejecutable quitando posibles argumentos o comillas
    fn extract_executable_path(raw: &str) -> Option<String> {
        let trimmed = raw.trim();
        if trimmed.starts_with('"') {
            if let Some(end) = trimmed[1..].find('"') {
                return Some(trimmed[1..=end].to_string());
            }
        }

        let first_token = trimmed.split_whitespace().next()?;
        Some(first_token.to_string())
    }

    /// Elimina una tarea programada huérfana de forma segura
    pub fn delete_task(task_name: &str) -> anyhow::Result<bool> {
        #[cfg(windows)]
        {
            let status = Command::new("schtasks")
                .args(["/delete", "/tn", task_name, "/f"])
                .status()?;
            Ok(status.success())
        }

        #[cfg(not(windows))]
        {
            let _ = task_name;
            Ok(true)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_executable_path() {
        let raw = "\"C:\\Program Files\\App\\updater.exe\" --silent";
        assert_eq!(
            ScheduledTaskManager::extract_executable_path(raw),
            Some("C:\\Program Files\\App\\updater.exe".to_string())
        );

        let raw2 = "C:\\Tools\\clean.bat /all";
        assert_eq!(
            ScheduledTaskManager::extract_executable_path(raw2),
            Some("C:\\Tools\\clean.bat".to_string())
        );
    }
}
