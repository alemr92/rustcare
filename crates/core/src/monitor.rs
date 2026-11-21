//! Monitor del Sistema en Tiempo Real para Windows 10/11
use serde::{Deserialize, Serialize};
use sysinfo::{CpuRefreshKind, Disks, MemoryRefreshKind, RefreshKind, System};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessSnapshot {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub memory_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskSnapshot {
    pub name: String,
    pub mount_point: String,
    pub total_space: u64,
    pub available_space: u64,
    pub is_removable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetricsSnapshot {
    pub cpu_global_percent: f32,
    pub cpu_count: usize,
    pub cpu_cores_usage: Vec<f32>,
    pub memory_total_bytes: u64,
    pub memory_used_bytes: u64,
    pub memory_free_bytes: u64,
    pub memory_usage_percent: f32,
    pub swap_total_bytes: u64,
    pub swap_used_bytes: u64,
    pub disks: Vec<DiskSnapshot>,
    pub top_processes_cpu: Vec<ProcessSnapshot>,
    pub top_processes_memory: Vec<ProcessSnapshot>,
    pub os_name: String,
    pub os_version: String,
    pub uptime_seconds: u64,
}

pub struct SystemMonitor {
    system: System,
    disks: Disks,
}

impl Default for SystemMonitor {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemMonitor {
    pub fn new() -> Self {
        let refresh_kind = RefreshKind::nothing()
            .with_cpu(CpuRefreshKind::everything())
            .with_memory(MemoryRefreshKind::everything());
        let mut system = System::new_with_specifics(refresh_kind);
        system.refresh_all();
        let disks = Disks::new_with_refreshed_list();

        Self { system, disks }
    }

    /// Actualiza y toma una captura instantánea de las métricas del sistema
    pub fn refresh_snapshot(&mut self) -> SystemMetricsSnapshot {
        self.system.refresh_cpu_all();
        self.system.refresh_memory();
        self.system.refresh_processes(sysinfo::ProcessesToUpdate::All, true);
        self.disks.refresh(true);

        let cpu_global_percent = self.system.global_cpu_usage();
        let cpu_cores_usage = self.system.cpus().iter().map(|c| c.cpu_usage()).collect();

        let memory_total_bytes = self.system.total_memory();
        let memory_used_bytes = self.system.used_memory();
        let memory_free_bytes = self.system.free_memory();
        let memory_usage_percent = if memory_total_bytes > 0 {
            (memory_used_bytes as f32 / memory_total_bytes as f32) * 100.0
        } else {
            0.0
        };

        let swap_total_bytes = self.system.total_swap();
        let swap_used_bytes = self.system.used_swap();

        let disk_snapshots: Vec<DiskSnapshot> = self
            .disks
            .iter()
            .map(|d| DiskSnapshot {
                name: d.name().to_string_lossy().to_string(),
                mount_point: d.mount_point().to_string_lossy().to_string(),
                total_space: d.total_space(),
                available_space: d.available_space(),
                is_removable: d.is_removable(),
            })
            .collect();

        // Procesos ordenados por CPU
        let mut processes: Vec<ProcessSnapshot> = self
            .system
            .processes()
            .iter()
            .map(|(pid, proc)| ProcessSnapshot {
                pid: pid.as_u32(),
                name: proc.name().to_string_lossy().to_string(),
                cpu_usage: proc.cpu_usage(),
                memory_bytes: proc.memory(),
            })
            .collect();

        processes.sort_by(|a, b| b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap_or(std::cmp::Ordering::Equal));
        let top_processes_cpu: Vec<ProcessSnapshot> = processes.iter().take(5).cloned().collect();

        processes.sort_by(|a, b| b.memory_bytes.cmp(&a.memory_bytes));
        let top_processes_memory: Vec<ProcessSnapshot> = processes.iter().take(5).cloned().collect();

        SystemMetricsSnapshot {
            cpu_global_percent,
            cpu_count: self.system.cpus().len(),
            cpu_cores_usage,
            memory_total_bytes,
            memory_used_bytes,
            memory_free_bytes,
            memory_usage_percent,
            swap_total_bytes,
            swap_used_bytes,
            disks: disk_snapshots,
            top_processes_cpu,
            top_processes_memory,
            os_name: System::name().unwrap_or_else(|| "Windows".to_string()),
            os_version: System::os_version().unwrap_or_else(|| "10/11".to_string()),
            uptime_seconds: System::uptime(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monitor_snapshot() {
        let mut monitor = SystemMonitor::new();
        let snap = monitor.refresh_snapshot();
        assert!(snap.cpu_count > 0);
        assert!(snap.memory_total_bytes > 0);
    }
}
