//! Motor de Modo Reposo de Programas (estilo AVG TuneUp Sleep Mode)
//! Permite suspender elementos de inicio y procesos en segundo plano de forma reversible.
use crate::backend::SystemBackend;
use crate::monitor::SystemMetricsSnapshot;
use crate::startup::StartupEntry;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SleepImpact {
    High,
    Medium,
    Low,
}

impl SleepImpact {
    pub fn label(&self) -> &'static str {
        match self {
            SleepImpact::High => "Alto",
            SleepImpact::Medium => "Medio",
            SleepImpact::Low => "Bajo",
        }
    }

    pub fn bar_fraction(&self) -> f32 {
        match self {
            SleepImpact::High => 0.85,
            SleepImpact::Medium => 0.52,
            SleepImpact::Low => 0.22,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SleepProgramItem {
    pub id: String,
    pub name: String,
    pub startup_id: Option<String>,
    pub process_name: Option<String>,
    pub memory_mb: f32,
    pub impact: SleepImpact,
    pub is_sleeping: bool,
    pub is_ignored: bool,
}

pub struct SleepEngine;

impl SleepEngine {
    const PROTECTED_PROCESSES: &'static [&'static str] = &[
        "system",
        "idle",
        "svchost.exe",
        "explorer.exe",
        "csrss.exe",
        "wininit.exe",
        "winlogon.exe",
        "lsass.exe",
        "services.exe",
        "smss.exe",
        "dwm.exe",
        "registry",
        "memory compression",
        "rustcare",
        "rustcare-gui.exe",
    ];

    fn is_protected(name: &str) -> bool {
        let low = name.to_lowercase();
        Self::PROTECTED_PROCESSES.iter().any(|p| low.contains(p))
    }

    /// Combina entradas de inicio y procesos activos en segundo plano para detectar programas que ralentizan el PC
    pub fn scan_programs(
        startup_entries: &[StartupEntry],
        snapshot: &SystemMetricsSnapshot,
    ) -> Vec<SleepProgramItem> {
        let mut items = Vec::new();
        let mut seen = HashSet::new();

        for entry in startup_entries {
            if Self::is_protected(&entry.name) {
                continue;
            }
            let low_cmd = entry.command.to_lowercase();
            let impact = if low_cmd.contains("update")
                || low_cmd.contains("helper")
                || low_cmd.contains("cloud")
                || low_cmd.contains("onedrive")
                || low_cmd.contains("steam")
                || low_cmd.contains("discord")
            {
                SleepImpact::High
            } else if entry.is_enabled {
                SleepImpact::Medium
            } else {
                SleepImpact::Low
            };

            let matching_mem = snapshot
                .top_processes_memory
                .iter()
                .chain(snapshot.top_processes_cpu.iter())
                .find(|p| p.name.to_lowercase().contains(&entry.name.to_lowercase()))
                .map(|p| p.memory_bytes as f32 / (1024.0 * 1024.0))
                .unwrap_or(45.0);

            seen.insert(entry.name.to_lowercase());
            items.push(SleepProgramItem {
                id: format!("startup:{}", entry.id),
                name: entry.name.clone(),
                startup_id: Some(entry.id.clone()),
                process_name: None,
                memory_mb: matching_mem,
                impact,
                is_sleeping: !entry.is_enabled,
                is_ignored: false,
            });
        }

        // Añadir procesos pesados en segundo plano que no estén ya listados en inicio
        for proc_snap in snapshot
            .top_processes_memory
            .iter()
            .chain(snapshot.top_processes_cpu.iter())
        {
            if Self::is_protected(&proc_snap.name) {
                continue;
            }
            let key = proc_snap.name.to_lowercase();
            if seen.contains(&key) {
                continue;
            }
            seen.insert(key);
            let mem_mb = proc_snap.memory_bytes as f32 / (1024.0 * 1024.0);
            if mem_mb < 35.0 && proc_snap.cpu_usage < 1.0 {
                continue;
            }
            let impact = if mem_mb > 300.0 || proc_snap.cpu_usage > 5.0 {
                SleepImpact::High
            } else if mem_mb > 100.0 {
                SleepImpact::Medium
            } else {
                SleepImpact::Low
            };
            items.push(SleepProgramItem {
                id: format!("proc:{}", proc_snap.pid),
                name: proc_snap.name.clone(),
                startup_id: None,
                process_name: Some(proc_snap.name.clone()),
                memory_mb: mem_mb,
                impact,
                is_sleeping: false,
                is_ignored: false,
            });
        }

        items.sort_by(|a, b| {
            b.impact
                .bar_fraction()
                .partial_cmp(&a.impact.bar_fraction())
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        items
    }

    /// Pone en reposo un programa (desactiva su arranque automático de forma reversible)
    pub fn sleep_program(item: &mut SleepProgramItem, backend: &Arc<dyn SystemBackend>) -> Result<()> {
        if let Some(sid) = &item.startup_id {
            backend.toggle_startup_entry(sid, false)?;
        }
        item.is_sleeping = true;
        Ok(())
    }

    /// Despierta un programa (reactiva su arranque automático)
    pub fn wake_program(item: &mut SleepProgramItem, backend: &Arc<dyn SystemBackend>) -> Result<()> {
        if let Some(sid) = &item.startup_id {
            backend.toggle_startup_entry(sid, true)?;
        }
        item.is_sleeping = false;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::SandboxBackend;
    use crate::monitor::ProcessSnapshot;

    #[test]
    fn test_sleep_mode_scan_excludes_critical() {
        let mut snap = SystemMetricsSnapshot {
            cpu_global_percent: 15.0,
            cpu_count: 8,
            cpu_cores_usage: vec![15.0; 8],
            memory_total_bytes: 16 * 1024 * 1024 * 1024,
            memory_used_bytes: 6 * 1024 * 1024 * 1024,
            memory_free_bytes: 10 * 1024 * 1024 * 1024,
            memory_usage_percent: 37.5,
            swap_total_bytes: 0,
            swap_used_bytes: 0,
            disks: vec![],
            top_processes_cpu: vec![
                ProcessSnapshot {
                    pid: 4,
                    name: "svchost.exe".into(),
                    cpu_usage: 2.0,
                    memory_bytes: 200 * 1024 * 1024,
                },
                ProcessSnapshot {
                    pid: 555,
                    name: "OneDrive.exe".into(),
                    cpu_usage: 3.5,
                    memory_bytes: 350 * 1024 * 1024,
                },
            ],
            top_processes_memory: vec![],
            os_name: "Windows".into(),
            os_version: "11".into(),
            uptime_seconds: 1200,
        };
        let items = SleepEngine::scan_programs(&[], &snap);
        assert!(items.iter().all(|i| !i.name.eq_ignore_ascii_case("svchost.exe")));
        assert!(items.iter().any(|i| i.name == "OneDrive.exe"));
        snap.top_processes_cpu.clear();
    }

    #[test]
    fn test_sleep_and_wake_sandboxed() {
        let backend: Arc<dyn SystemBackend> = Arc::new(SandboxBackend::new());
        let entries = backend.list_startup_entries();
        let snap = backend.get_metrics();
        let mut programs = SleepEngine::scan_programs(&entries, &snap);
        assert!(!programs.is_empty());
        let first = &mut programs[0];
        SleepEngine::sleep_program(first, &backend).unwrap();
        assert!(first.is_sleeping);
        SleepEngine::wake_program(first, &backend).unwrap();
        assert!(!first.is_sleeping);
    }
}
