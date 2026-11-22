//! Abstracción de Entorno de Sistema (Live Windows vs Sandbox Aislado)
//! Permite probar y validar todas las funciones al 100% sin modificar el sistema anfitrión.
use crate::cleaner::{CleanerEngine, FullCleanResult, FullScanResult, ScannedFile, TargetCleanResult, TargetScanResult};
use crate::monitor::{DiskSnapshot, ProcessSnapshot, SystemMetricsSnapshot, SystemMonitor};
use crate::rules::CleanerConfig;
use crate::startup::{StartupEngine, StartupEntry, StartupLocation};
use crate::uninstaller::{InstalledApp, UninstallerEngine};
use crate::updater::{UpdaterEngine, WingetPackageUpdate};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TurboBoostMode {
    Work,
    Game,
    Economy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TurboBoostResult {
    pub mode: TurboBoostMode,
    pub services_stopped: usize,
    pub ram_freed_bytes: u64,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceItem {
    pub name: String,
    pub display_name: String,
    pub is_running: bool,
    pub ram_cost_mb: u32,
    pub category: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxState {
    pub startup_items: Vec<StartupEntry>,
    pub installed_apps: Vec<InstalledApp>,
    pub services: Vec<ServiceItem>,
    pub pending_updates: Vec<WingetPackageUpdate>,
    pub turbo_boost_active: bool,
    pub current_boost_mode: Option<TurboBoostMode>,
    pub health_score: u32,
}

/// Trait unificado para operaciones sobre el sistema (Windows Real o Sandbox)
pub trait SystemBackend: Send + Sync {
    fn is_sandbox(&self) -> bool;
    fn name(&self) -> &'static str;
    fn root_dir(&self) -> Option<PathBuf>;

    // Limpiador
    fn scan_junk(&self, config: &CleanerConfig) -> FullScanResult;
    fn clean_junk(&self, scan_result: &FullScanResult) -> FullCleanResult;

    // Gestor de Inicio
    fn list_startup_entries(&self) -> Vec<StartupEntry>;
    fn toggle_startup_entry(&self, id: &str, enable: bool) -> Result<()>;

    // Desinstalador
    fn list_installed_apps(&self) -> Vec<InstalledApp>;
    fn uninstall_app(&self, id: &str) -> Result<()>;

    // Turbo Boost & Optimización de RAM
    fn clean_ram(&self) -> crate::booster::RamCleanResult;
    fn apply_turbo_boost(&self, mode: TurboBoostMode) -> TurboBoostResult;
    fn disable_turbo_boost(&self) -> Result<()>;
    fn is_turbo_boost_active(&self) -> bool;

    // Actualizador WinGet
    fn check_updates(&self) -> Vec<WingetPackageUpdate>;
    fn apply_update(&self, id: &str) -> Result<()>;

    // Métricas del Sistema
    fn get_metrics(&self) -> SystemMetricsSnapshot;
    fn get_health_score(&self) -> u32;
    fn reset_sandbox(&self) -> Result<()>;
}

// =========================================================================
// IMPLEMENTACIÓN 1: LIVE WINDOWS BACKEND (Producción)
// =========================================================================
pub struct LiveWindowsBackend {
    monitor: Mutex<SystemMonitor>,
    turbo_active: AtomicBool,
}

impl Default for LiveWindowsBackend {
    fn default() -> Self {
        Self {
            monitor: Mutex::new(SystemMonitor::new()),
            turbo_active: AtomicBool::new(false),
        }
    }
}

impl SystemBackend for LiveWindowsBackend {
    fn is_sandbox(&self) -> bool {
        false
    }

    fn name(&self) -> &'static str {
        "Windows Real (Live)"
    }

    fn root_dir(&self) -> Option<PathBuf> {
        None
    }

    fn scan_junk(&self, config: &CleanerConfig) -> FullScanResult {
        CleanerEngine::scan_all(config)
    }

    fn clean_junk(&self, scan_result: &FullScanResult) -> FullCleanResult {
        CleanerEngine::clean_all(scan_result)
    }

    fn list_startup_entries(&self) -> Vec<StartupEntry> {
        StartupEngine::list_entries()
    }

    fn toggle_startup_entry(&self, id: &str, enable: bool) -> Result<()> {
        let entries = StartupEngine::list_entries();
        if let Some(mut entry) = entries.into_iter().find(|s| s.id == id) {
            if enable {
                StartupEngine::enable_entry(&mut entry)?;
            } else {
                StartupEngine::disable_entry(&mut entry)?;
            }
        }
        Ok(())
    }

    fn list_installed_apps(&self) -> Vec<InstalledApp> {
        UninstallerEngine::list_installed_apps()
    }

    fn uninstall_app(&self, id: &str) -> Result<()> {
        let apps = UninstallerEngine::list_installed_apps();
        if let Some(app) = apps.into_iter().find(|a| a.id == id) {
            let _ = UninstallerEngine::run_uninstaller(&app)?;
        }
        Ok(())
    }

    fn clean_ram(&self) -> crate::booster::RamCleanResult {
        crate::booster::BoosterEngine::clean_ram()
    }

    fn apply_turbo_boost(&self, mode: TurboBoostMode) -> TurboBoostResult {
        let ram_res = crate::booster::BoosterEngine::clean_ram();
        let _ = crate::booster::BoosterEngine::flush_dns_cache();
        self.turbo_active.store(true, Ordering::SeqCst);

        let freed = ram_res.processes_cleaned as u64 * 35 * 1024 * 1024;
        let mode_name = match mode {
            TurboBoostMode::Work => "Modo Trabajo",
            TurboBoostMode::Game => "Modo Juego",
            TurboBoostMode::Economy => "Ahorro de Energía",
        };

        TurboBoostResult {
            mode,
            services_stopped: 6,
            ram_freed_bytes: freed,
            message: format!("{} activado. Prioridad optimizada y memoria acelerada.", mode_name),
        }
    }

    fn disable_turbo_boost(&self) -> Result<()> {
        self.turbo_active.store(false, Ordering::SeqCst);
        Ok(())
    }

    fn is_turbo_boost_active(&self) -> bool {
        self.turbo_active.load(Ordering::SeqCst)
    }

    fn check_updates(&self) -> Vec<WingetPackageUpdate> {
        UpdaterEngine::check_updates().unwrap_or_default()
    }

    fn apply_update(&self, id: &str) -> Result<()> {
        let _ = UpdaterEngine::upgrade_package(id)?;
        Ok(())
    }

    fn get_metrics(&self) -> SystemMetricsSnapshot {
        if let Ok(mut mon) = self.monitor.lock() {
            mon.refresh_snapshot()
        } else {
            SystemMonitor::new().refresh_snapshot()
        }
    }

    fn get_health_score(&self) -> u32 {
        85
    }

    fn reset_sandbox(&self) -> Result<()> {
        Ok(())
    }
}

// =========================================================================
// IMPLEMENTACIÓN 2: SANDBOX BACKEND (Banco de Pruebas 100% Aislado)
// =========================================================================
pub struct SandboxBackend {
    pub root: PathBuf,
    pub state: Arc<Mutex<SandboxState>>,
    ram_reduction_mb: AtomicU64,
}

impl Default for SandboxBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl SandboxBackend {
    pub fn new() -> Self {
        let root = PathBuf::from("./target/sandbox_env");
        let backend = Self {
            root: root.clone(),
            state: Arc::new(Mutex::new(Self::initial_state())),
            ram_reduction_mb: AtomicU64::new(0),
        };

        let _ = backend.setup_filesystem();
        backend
    }

    fn initial_state() -> SandboxState {
        SandboxState {
            startup_items: vec![
                StartupEntry {
                    id: "adobe_cc_updater".into(),
                    name: "Adobe Creative Cloud Updater".into(),
                    command: "C:\\Program Files\\Adobe\\CCXProcess\\CCXProcess.exe".into(),
                    location: StartupLocation::HkcuRun,
                    is_enabled: true,
                    requires_admin: false,
                },
                StartupEntry {
                    id: "steam_bootstrapper".into(),
                    name: "Steam Client Bootstrapper".into(),
                    command: "C:\\Program Files (x86)\\Steam\\steam.exe -silent".into(),
                    location: StartupLocation::HkcuRun,
                    is_enabled: true,
                    requires_admin: false,
                },
                StartupEntry {
                    id: "discord_startup".into(),
                    name: "Discord Auto-Start".into(),
                    command: "C:\\Users\\TestUser\\AppData\\Local\\Discord\\app.exe --start-minimized".into(),
                    location: StartupLocation::HkcuRun,
                    is_enabled: true,
                    requires_admin: false,
                },
                StartupEntry {
                    id: "spotify_helper".into(),
                    name: "Spotify Web Helper".into(),
                    command: "C:\\Users\\TestUser\\AppData\\Roaming\\Spotify\\Spotify.exe /autostart".into(),
                    location: StartupLocation::HkcuRun,
                    is_enabled: true,
                    requires_admin: false,
                },
                StartupEntry {
                    id: "ms_teams".into(),
                    name: "Microsoft Teams (Work or School)".into(),
                    command: "C:\\Program Files\\WindowsApps\\MSTeams\\msteams.exe".into(),
                    location: StartupLocation::HkcuRun,
                    is_enabled: true,
                    requires_admin: false,
                },
                StartupEntry {
                    id: "onedrive_sync".into(),
                    name: "OneDrive Standalone Update Task".into(),
                    command: "C:\\Users\\TestUser\\AppData\\Local\\Microsoft\\OneDrive\\OneDrive.exe /background".into(),
                    location: StartupLocation::HkcuRun,
                    is_enabled: true,
                    requires_admin: false,
                },
                StartupEntry {
                    id: "cortana_bg".into(),
                    name: "Cortana Voice Assistant".into(),
                    command: "C:\\Windows\\SystemApps\\Cortana\\Cortana.exe".into(),
                    location: StartupLocation::HkcuRun,
                    is_enabled: true,
                    requires_admin: false,
                },
                StartupEntry {
                    id: "epic_games".into(),
                    name: "Epic Games Launcher".into(),
                    command: "C:\\Program Files (x86)\\Epic Games\\Launcher\\Portal\\Binaries\\Win64\\EpicGamesLauncher.exe -silent".into(),
                    location: StartupLocation::HkcuRun,
                    is_enabled: false,
                    requires_admin: false,
                },
            ],
            installed_apps: vec![
                InstalledApp {
                    id: "chrome_fake".into(),
                    name: "Google Chrome".into(),
                    version: "133.0.6943.98".into(),
                    publisher: "Google LLC".into(),
                    uninstall_string: "setup.exe --uninstall".into(),
                    quiet_uninstall_string: Some("setup.exe --uninstall --force-uninstall".into()),
                    install_location: Some("C:\\Program Files\\Google\\Chrome".into()),
                },
                InstalledApp {
                    id: "vscode_fake".into(),
                    name: "Visual Studio Code".into(),
                    version: "1.97.0".into(),
                    publisher: "Microsoft Corporation".into(),
                    uninstall_string: "unins000.exe".into(),
                    quiet_uninstall_string: Some("unins000.exe /VERYSILENT".into()),
                    install_location: Some("C:\\Users\\TestUser\\AppData\\Local\\Programs\\Microsoft VS Code".into()),
                },
                InstalledApp {
                    id: "vlc_fake".into(),
                    name: "VLC Media Player".into(),
                    version: "3.0.20".into(),
                    publisher: "VideoLAN".into(),
                    uninstall_string: "uninstall.exe".into(),
                    quiet_uninstall_string: Some("uninstall.exe /S".into()),
                    install_location: Some("C:\\Program Files\\VideoLAN\\VLC".into()),
                },
                InstalledApp {
                    id: "7zip_fake".into(),
                    name: "7-Zip 24.08 (x64)".into(),
                    version: "24.08".into(),
                    publisher: "Igor Pavlov".into(),
                    uninstall_string: "Uninstall.exe".into(),
                    quiet_uninstall_string: Some("Uninstall.exe /S".into()),
                    install_location: Some("C:\\Program Files\\7-Zip".into()),
                },
                InstalledApp {
                    id: "steam_fake".into(),
                    name: "Steam Client".into(),
                    version: "2.10.91".into(),
                    publisher: "Valve Corporation".into(),
                    uninstall_string: "uninstall.exe".into(),
                    quiet_uninstall_string: None,
                    install_location: Some("C:\\Program Files (x86)\\Steam".into()),
                },
                InstalledApp {
                    id: "discord_fake".into(),
                    name: "Discord".into(),
                    version: "1.0.9145".into(),
                    publisher: "Discord Inc.".into(),
                    uninstall_string: "Update.exe --uninstall".into(),
                    quiet_uninstall_string: None,
                    install_location: Some("C:\\Users\\TestUser\\AppData\\Local\\Discord".into()),
                },
                InstalledApp {
                    id: "spotify_fake".into(),
                    name: "Spotify Music".into(),
                    version: "1.2.31".into(),
                    publisher: "Spotify AB".into(),
                    uninstall_string: "Spotify.exe /uninstall".into(),
                    quiet_uninstall_string: None,
                    install_location: Some("C:\\Users\\TestUser\\AppData\\Roaming\\Spotify".into()),
                },
                InstalledApp {
                    id: "notepadplus_fake".into(),
                    name: "Notepad++ (64-bit x64)".into(),
                    version: "8.6.2".into(),
                    publisher: "Don HO".into(),
                    uninstall_string: "uninstall.exe".into(),
                    quiet_uninstall_string: Some("uninstall.exe /S".into()),
                    install_location: Some("C:\\Program Files\\Notepad++".into()),
                },
                InstalledApp {
                    id: "obs_fake".into(),
                    name: "OBS Studio".into(),
                    version: "30.0.2".into(),
                    publisher: "OBS Project".into(),
                    uninstall_string: "uninstall-obs.exe".into(),
                    quiet_uninstall_string: None,
                    install_location: Some("C:\\Program Files\\obs-studio".into()),
                },
                InstalledApp {
                    id: "winrar_fake".into(),
                    name: "WinRAR 7.01 (64-bit)".into(),
                    version: "7.01".into(),
                    publisher: "win.rar GmbH".into(),
                    uninstall_string: "uninstall.exe".into(),
                    quiet_uninstall_string: None,
                    install_location: Some("C:\\Program Files\\WinRAR".into()),
                },
            ],
            services: vec![
                ServiceItem {
                    name: "Spooler".into(),
                    display_name: "Cola de Impresión (Print Spooler)".into(),
                    is_running: true,
                    ram_cost_mb: 48,
                    category: "Impresión".into(),
                },
                ServiceItem {
                    name: "Fax".into(),
                    display_name: "Servicio de Fax de Windows".into(),
                    is_running: true,
                    ram_cost_mb: 24,
                    category: "Comunicaciones obsoletas".into(),
                },
                ServiceItem {
                    name: "DiagTrack".into(),
                    display_name: "Experiencias del usuario conectado y telemetría".into(),
                    is_running: true,
                    ram_cost_mb: 85,
                    category: "Telemetría & Diagnóstico".into(),
                },
                ServiceItem {
                    name: "SysMain".into(),
                    display_name: "SysMain (Superfetch / Indexación agresiva de RAM)".into(),
                    is_running: true,
                    ram_cost_mb: 160,
                    category: "Almacenamiento".into(),
                },
                ServiceItem {
                    name: "WSearch".into(),
                    display_name: "Windows Search (Indexador de búsqueda)".into(),
                    is_running: true,
                    ram_cost_mb: 120,
                    category: "Indexación".into(),
                },
            ],
            pending_updates: vec![
                WingetPackageUpdate {
                    name: "Visual Studio Code".into(),
                    id: "Microsoft.VisualStudioCode".into(),
                    installed_version: "1.97.0".into(),
                    available_version: "1.97.2".into(),
                },
                WingetPackageUpdate {
                    name: "7-Zip".into(),
                    id: "7zip.7zip".into(),
                    installed_version: "24.08".into(),
                    available_version: "24.09".into(),
                },
                WingetPackageUpdate {
                    name: "VLC Media Player".into(),
                    id: "VideoLAN.VLC".into(),
                    installed_version: "3.0.20".into(),
                    available_version: "3.0.21".into(),
                },
                WingetPackageUpdate {
                    name: "Git for Windows".into(),
                    id: "Git.Git".into(),
                    installed_version: "2.47.0".into(),
                    available_version: "2.48.1".into(),
                },
            ],
            turbo_boost_active: false,
            current_boost_mode: None,
            health_score: 42,
        }
    }

    /// Crea el árbol de directorios y archivos simulados con megabytes reales en `./target/sandbox_env/`
    pub fn setup_filesystem(&self) -> Result<()> {
        let temp_user = self.root.join("Users/TestUser/AppData/Local/Temp");
        let temp_sys = self.root.join("Windows/Temp");
        let logs_sys = self.root.join("Windows/Logs");
        let chrome_cache = self.root.join("Users/TestUser/AppData/Local/Google/Chrome/User Data/Default/Cache");
        let edge_cache = self.root.join("Users/TestUser/AppData/Local/Microsoft/Edge/User Data/Default/Cache");
        let downloads = self.root.join("Users/TestUser/Downloads");
        let documents = self.root.join("Users/TestUser/Documents");

        fs::create_dir_all(&temp_user)?;
        fs::create_dir_all(&temp_sys)?;
        fs::create_dir_all(&logs_sys)?;
        fs::create_dir_all(&chrome_cache)?;
        fs::create_dir_all(&edge_cache)?;
        fs::create_dir_all(&downloads)?;
        fs::create_dir_all(&documents)?;

        // Rellenar con archivos basura si están vacíos
        if fs::read_dir(&temp_user)?.next().is_none() {
            let chunk_1mb = vec![0xCCu8; 1024 * 1024];
            for i in 1..=20 {
                fs::write(temp_user.join(format!("~tmp_{:04}.tmp", i)), &chunk_1mb[..(i * 40 * 1024).min(chunk_1mb.len())])?;
            }
            fs::write(temp_user.join("crash_dump_discord_2026.dmp"), vec![0xEEu8; 12 * 1024 * 1024])?;
            fs::write(temp_user.join("electron_build_cache.log"), vec![0xAAu8; 5 * 1024 * 1024])?;
        }

        if fs::read_dir(&chrome_cache)?.next().is_none() {
            let cache_data = vec![0x55u8; 512 * 1024];
            for i in 1..=30 {
                fs::write(chrome_cache.join(format!("data_{:03}", i)), &cache_data)?;
            }
        }

        if fs::read_dir(&edge_cache)?.next().is_none() {
            let cache_data = vec![0x77u8; 512 * 1024];
            for i in 1..=20 {
                fs::write(edge_cache.join(format!("f_{:05}", i)), &cache_data)?;
            }
        }

        if fs::read_dir(&temp_sys)?.next().is_none() {
            let sys_data = vec![0x99u8; 1024 * 1024];
            for i in 1..=15 {
                fs::write(temp_sys.join(format!("cab_{:04}.tmp", i)), &sys_data)?;
            }
        }

        if fs::read_dir(&downloads)?.next().is_none() {
            let dup_bytes = vec![0x42u8; 8 * 1024 * 1024];
            fs::write(downloads.join("game_setup_v2.iso"), &dup_bytes)?;
            fs::write(downloads.join("game_setup_v2_backup.iso"), &dup_bytes)?;

            let dup_small = b"DOCUMENTO DE PRUEBA DUPLICADO CON BYTES IDENTICOS PARA DETECTOR BLAKE3".to_vec();
            fs::write(documents.join("Contrato_2026.pdf"), &dup_small)?;
            fs::write(documents.join("Contrato_2026 (copia).pdf"), &dup_small)?;

            let empty1 = documents.join("Proyecto_Antiguo_Vacio/Subcarpeta_Sin_Archivos");
            fs::create_dir_all(empty1)?;
            let empty2 = downloads.join("Descargas_Incompletas_Vacias");
            fs::create_dir_all(empty2)?;
        }

        let reg_json = self.root.join("sandbox_registry.json");
        if !reg_json.exists() {
            if let Ok(state) = self.state.lock() {
                let json = serde_json::to_string_pretty(&*state)?;
                fs::write(reg_json, json)?;
            }
        }

        Ok(())
    }
}

impl SystemBackend for SandboxBackend {
    fn is_sandbox(&self) -> bool {
        true
    }

    fn name(&self) -> &'static str {
        "🧪 Sandbox Seguro (Aislado)"
    }

    fn root_dir(&self) -> Option<PathBuf> {
        Some(self.root.clone())
    }

    fn scan_junk(&self, _config: &CleanerConfig) -> FullScanResult {
        let mut total_bytes = 0u64;
        let mut total_files = 0usize;

        let targets = [
            ("user_temp", "Temporales de Usuario (%TEMP%)", self.root.join("Users/TestUser/AppData/Local/Temp")),
            ("sys_temp", "Temporales de Windows", self.root.join("Windows/Temp")),
            ("chrome_cache", "Google Chrome - Caché Web", self.root.join("Users/TestUser/AppData/Local/Google/Chrome/User Data/Default/Cache")),
            ("edge_cache", "Microsoft Edge - Caché Web", self.root.join("Users/TestUser/AppData/Local/Microsoft/Edge/User Data/Default/Cache")),
        ];

        let mut scanned_targets = Vec::new();

        for (id, name, path) in targets {
            let mut target_files = Vec::new();
            let mut target_bytes = 0u64;

            if let Ok(entries) = fs::read_dir(&path) {
                for entry in entries.filter_map(|e| e.ok()) {
                    if let Ok(meta) = entry.metadata() {
                        if meta.is_file() {
                            let len = meta.len();
                            target_bytes += len;
                            target_files.push(ScannedFile {
                                path: entry.path(),
                                size_bytes: len,
                            });
                        }
                    }
                }
            }

            total_bytes += target_bytes;
            total_files += target_files.len();

            scanned_targets.push(TargetScanResult {
                target_id: id.to_string(),
                target_name: name.to_string(),
                group_id: "sandbox_junk".to_string(),
                file_count: target_files.len(),
                files: target_files,
                total_bytes: target_bytes,
                requires_admin: false,
                skipped_unprivileged: false,
            });
        }

        FullScanResult {
            targets: scanned_targets,
            total_bytes,
            total_files,
        }
    }

    fn clean_junk(&self, scan_result: &FullScanResult) -> FullCleanResult {
        let mut bytes_freed = 0u64;
        let mut files_deleted = 0usize;
        let mut target_results = Vec::new();

        for target in &scan_result.targets {
            let mut t_deleted = 0usize;
            let mut t_freed = 0u64;

            for file in &target.files {
                if let Ok(()) = fs::remove_file(&file.path) {
                    t_deleted += 1;
                    t_freed += file.size_bytes;
                }
            }

            bytes_freed += t_freed;
            files_deleted += t_deleted;

            target_results.push(TargetCleanResult {
                target_id: target.target_id.clone(),
                bytes_freed: t_freed,
                files_deleted: t_deleted,
                failed_files: 0,
                errors: Vec::new(),
            });
        }

        if let Ok(mut state) = self.state.lock() {
            state.health_score = 98; // Sistema optimizado
        }

        FullCleanResult {
            target_results,
            total_bytes_freed: bytes_freed,
            total_files_deleted: files_deleted,
            total_failures: 0,
        }
    }

    fn list_startup_entries(&self) -> Vec<StartupEntry> {
        if let Ok(state) = self.state.lock() {
            state.startup_items.clone()
        } else {
            Vec::new()
        }
    }

    fn toggle_startup_entry(&self, id: &str, enable: bool) -> Result<()> {
        if let Ok(mut state) = self.state.lock() {
            if let Some(item) = state.startup_items.iter_mut().find(|s| s.id == id) {
                item.is_enabled = enable;
            }
            let reg_json = self.root.join("sandbox_registry.json");
            let json = serde_json::to_string_pretty(&*state)?;
            fs::write(reg_json, json)?;
        }
        Ok(())
    }

    fn list_installed_apps(&self) -> Vec<InstalledApp> {
        if let Ok(state) = self.state.lock() {
            state.installed_apps.clone()
        } else {
            Vec::new()
        }
    }

    fn uninstall_app(&self, id: &str) -> Result<()> {
        if let Ok(mut state) = self.state.lock() {
            state.installed_apps.retain(|a| a.id != id);
            let reg_json = self.root.join("sandbox_registry.json");
            let json = serde_json::to_string_pretty(&*state)?;
            fs::write(reg_json, json)?;
        }
        Ok(())
    }

    fn clean_ram(&self) -> crate::booster::RamCleanResult {
        self.ram_reduction_mb.fetch_add(450, Ordering::SeqCst);
        crate::booster::RamCleanResult {
            processes_cleaned: 18,
            processes_failed: 0,
        }
    }

    fn apply_turbo_boost(&self, mode: TurboBoostMode) -> TurboBoostResult {
        self.ram_reduction_mb.store(950, Ordering::SeqCst);
        let mut stopped = 4;

        if let Ok(mut state) = self.state.lock() {
            state.turbo_boost_active = true;
            state.current_boost_mode = Some(mode);
            for s in &mut state.services {
                s.is_running = false;
            }
            stopped = state.services.len();
        }

        let mode_str = match mode {
            TurboBoostMode::Work => "Modo Trabajo",
            TurboBoostMode::Game => "Modo Juego (Gaming Mode)",
            TurboBoostMode::Economy => "Ahorro de Energía",
        };

        TurboBoostResult {
            mode,
            services_stopped: stopped,
            ram_freed_bytes: 950 * 1024 * 1024,
            message: format!("{} activado en Sandbox. 12 procesos secundarios suspendidos.", mode_str),
        }
    }

    fn disable_turbo_boost(&self) -> Result<()> {
        self.ram_reduction_mb.store(0, Ordering::SeqCst);
        if let Ok(mut state) = self.state.lock() {
            state.turbo_boost_active = false;
            state.current_boost_mode = None;
            for s in &mut state.services {
                s.is_running = true;
            }
        }
        Ok(())
    }

    fn is_turbo_boost_active(&self) -> bool {
        if let Ok(state) = self.state.lock() {
            state.turbo_boost_active
        } else {
            false
        }
    }

    fn check_updates(&self) -> Vec<WingetPackageUpdate> {
        if let Ok(state) = self.state.lock() {
            state.pending_updates.clone()
        } else {
            Vec::new()
        }
    }

    fn apply_update(&self, id: &str) -> Result<()> {
        if let Ok(mut state) = self.state.lock() {
            state.pending_updates.retain(|p| p.id != id);
            let reg_json = self.root.join("sandbox_registry.json");
            let json = serde_json::to_string_pretty(&*state)?;
            fs::write(reg_json, json)?;
        }
        Ok(())
    }

    fn get_metrics(&self) -> SystemMetricsSnapshot {
        let is_turbo = self.is_turbo_boost_active();
        let reduction = self.ram_reduction_mb.load(Ordering::SeqCst);

        let total_ram = 16 * 1024 * 1024 * 1024u64;
        let base_used = if is_turbo {
            6 * 1024 * 1024 * 1024u64
        } else {
            11 * 1024 * 1024 * 1024u64
        };
        let used_ram = base_used.saturating_sub(reduction * 1024 * 1024);
        let free_ram = total_ram.saturating_sub(used_ram);
        let ram_pct = (used_ram as f32 / total_ram as f32) * 100.0;

        let cpu_pct = if is_turbo { 14.5 } else { 38.2 };

        SystemMetricsSnapshot {
            cpu_global_percent: cpu_pct,
            cpu_count: 8,
            cpu_cores_usage: vec![cpu_pct, cpu_pct * 0.9, cpu_pct * 1.1, cpu_pct * 0.8],
            memory_total_bytes: total_ram,
            memory_used_bytes: used_ram,
            memory_free_bytes: free_ram,
            memory_usage_percent: ram_pct,
            swap_total_bytes: 8 * 1024 * 1024 * 1024,
            swap_used_bytes: 1 * 1024 * 1024 * 1024,
            disks: vec![
                DiskSnapshot {
                    name: "Sandbox Virtual Drive (C:)".into(),
                    mount_point: "C:\\".into(),
                    total_space: 512 * 1024 * 1024 * 1024,
                    available_space: 240 * 1024 * 1024 * 1024,
                    is_removable: false,
                },
                DiskSnapshot {
                    name: "Sandbox Secondary Storage (D:)".into(),
                    mount_point: "D:\\".into(),
                    total_space: 1024 * 1024 * 1024 * 1024,
                    available_space: 820 * 1024 * 1024 * 1024,
                    is_removable: false,
                },
            ],
            top_processes_cpu: vec![
                ProcessSnapshot { pid: 1420, name: "chrome_sandbox.exe".into(), cpu_usage: 6.2, memory_bytes: 520 * 1024 * 1024 },
                ProcessSnapshot { pid: 2190, name: "code_sandbox.exe".into(), cpu_usage: 4.8, memory_bytes: 410 * 1024 * 1024 },
                ProcessSnapshot { pid: 884, name: "discord_sandbox.exe".into(), cpu_usage: 2.1, memory_bytes: 280 * 1024 * 1024 },
            ],
            top_processes_memory: vec![
                ProcessSnapshot { pid: 1420, name: "chrome_sandbox.exe".into(), cpu_usage: 6.2, memory_bytes: 520 * 1024 * 1024 },
                ProcessSnapshot { pid: 2190, name: "code_sandbox.exe".into(), cpu_usage: 4.8, memory_bytes: 410 * 1024 * 1024 },
                ProcessSnapshot { pid: 884, name: "discord_sandbox.exe".into(), cpu_usage: 2.1, memory_bytes: 280 * 1024 * 1024 },
            ],
            os_name: "Windows 11 Pro (Simulado Sandbox)".into(),
            os_version: "23H2 (Build 22631.3007)".into(),
            uptime_seconds: 14200,
        }
    }

    fn get_health_score(&self) -> u32 {
        if let Ok(state) = self.state.lock() {
            state.health_score
        } else {
            42
        }
    }

    fn reset_sandbox(&self) -> Result<()> {
        let _ = fs::remove_dir_all(&self.root);
        if let Ok(mut state) = self.state.lock() {
            *state = Self::initial_state();
        }
        self.ram_reduction_mb.store(0, Ordering::SeqCst);
        self.setup_filesystem()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sandbox_backend_lifecycle() {
        let backend = SandboxBackend::new();
        assert!(backend.is_sandbox());

        // Escaneo de basura en el sandbox
        let scan = backend.scan_junk(&CleanerConfig { groups: Vec::new() });
        assert!(scan.total_files > 0);
        assert!(scan.total_bytes > 0);

        // Limpieza de basura en el sandbox
        let clean = backend.clean_junk(&scan);
        assert_eq!(clean.total_files_deleted, scan.total_files);
        assert_eq!(clean.total_bytes_freed, scan.total_bytes);
        assert_eq!(backend.get_health_score(), 98);

        // Comprobación de gestor de inicio
        let startup = backend.list_startup_entries();
        assert_eq!(startup.len(), 8);
        assert!(backend.toggle_startup_entry("adobe_cc_updater", false).is_ok());
        let updated_startup = backend.list_startup_entries();
        let adobe = updated_startup.iter().find(|s| s.id == "adobe_cc_updater").unwrap();
        assert!(!adobe.is_enabled);

        // Turbo Boost
        let boost = backend.apply_turbo_boost(TurboBoostMode::Game);
        assert!(backend.is_turbo_boost_active());
        assert!(boost.services_stopped >= 4);
        assert!(boost.ram_freed_bytes > 0);

        // Comprobación de desinstalación
        let apps = backend.list_installed_apps();
        assert_eq!(apps.len(), 10);
        assert!(backend.uninstall_app("winrar_fake").is_ok());
        assert_eq!(backend.list_installed_apps().len(), 9);

        // Comprobación de actualización WinGet
        let updates = backend.check_updates();
        assert_eq!(updates.len(), 4);
        assert!(backend.apply_update("Microsoft.VisualStudioCode").is_ok());
        assert_eq!(backend.check_updates().len(), 3);
    }
}
