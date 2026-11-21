//! RustCare Core Library
//! Arquitectura central de optimización, limpieza, monitorización y seguridad para Windows 10/11.

pub mod booster;
pub mod cleaner;
pub mod config;
pub mod context_menu;
pub mod disk_analyzer;
pub mod drivers;
pub mod duplicates;
pub mod empty_folders;
pub mod large_files;
pub mod monitor;
pub mod privacy;
pub mod rescue;
pub mod rules;
pub mod security;
pub mod shield;
pub mod shortcuts;
pub mod shredder;
pub mod startup;
pub mod uninstaller;
pub mod updater;

pub use booster::{BoosterEngine, BoosterStatus, RamCleanResult};
pub use cleaner::{CleanerEngine, FullCleanResult, FullScanResult, ScannedFile, TargetCleanResult, TargetScanResult};
pub use config::{ConfigManager, UserPreferences};
pub use context_menu::{ContextMenuItem, ContextMenuManager};
pub use disk_analyzer::{DiskAnalysisResult, DiskAnalyzer, DiskEntryInfo};
pub use drivers::{DeviceDriverInfo, DriverManager};
pub use duplicates::{DuplicateFinder, DuplicateGroup, DuplicateScanResult};
pub use empty_folders::{EmptyFolderItem, EmptyFoldersFinder};
pub use large_files::{FileCategory, LargeFileItem, LargeFilesFinder};
pub use monitor::{DiskSnapshot, ProcessSnapshot, SystemMetricsSnapshot, SystemMonitor};
pub use privacy::{PrivacyCheckResult, PrivacyEngine, SettingStatus};
pub use rescue::{BackupEntry, RescueCenter};
pub use rules::RulesEngine;
pub use security::is_elevated;
pub use shield::{ShieldEngine, ShieldScanResult, ThreatItem, ThreatLevel};
pub use shortcuts::{BrokenShortcut, ShortcutCleaner};
pub use shredder::FileShredder;
pub use startup::{StartupEngine, StartupEntry, StartupLocation};
pub use uninstaller::{InstalledApp, ResidualFolder, UninstallerEngine};
pub use updater::{UpdaterEngine, WingetPackageUpdate};
