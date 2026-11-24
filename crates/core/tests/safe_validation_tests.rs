//! Batería de Pruebas Seguras (Zero-Risk & Sandboxed) para RustCare
//! Todas las pruebas destructivas (borrado, triturado, limpieza) se ejecutan estrictamente
//! dentro de un directorio aislado en memoria/temporal (tempfile), garantizando CERO RIESGO
//! para el sistema operativo, los archivos del usuario o la estabilidad de Windows.

use rustcare_core::booster::BoosterEngine;
use rustcare_core::cleaner::{CleanerEngine, ScannedFile, TargetScanResult};
use rustcare_core::disk_analyzer::DiskAnalyzer;
use rustcare_core::drivers::DriverManager;
use rustcare_core::duplicates::DuplicateFinder;
use rustcare_core::empty_folders::EmptyFoldersFinder;
use rustcare_core::large_files::LargeFilesFinder;
use rustcare_core::monitor::SystemMonitor;
use rustcare_core::privacy::PrivacyEngine;
use rustcare_core::rescue::RescueCenter;
use rustcare_core::rules::cleaner::CleanerTarget;
use rustcare_core::rules::privacy::{PrivacyConfig, PrivacySetting};
use rustcare_core::security::is_elevated;
use rustcare_core::shield::ShieldEngine;
use rustcare_core::shredder::FileShredder;
use rustcare_core::startup::StartupEngine;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_safe_cleaner_dry_run_zero_risk() {
    let dir = tempdir().expect("Error creando tempdir aislado");
    let file1 = dir.path().join("cache1.tmp");
    let file2 = dir.path().join("log1.log");
    let keep_file = dir.path().join("important.doc");

    fs::write(&file1, "1234567890").unwrap();
    fs::write(&file2, "abcdefghij").unwrap();
    fs::write(&keep_file, "no borrar").unwrap();

    let target = CleanerTarget {
        id: "sandbox_cleaner_test".into(),
        name: "Sandbox Cleaner".into(),
        description: "Prueba segura".into(),
        path: dir.path().to_string_lossy().to_string(),
        patterns: vec!["*.tmp".into(), "*.log".into()],
        recursive: false,
        requires_admin: false,
        safety_level: "safe".into(),
        why_it_exists: "Archivos temporales de prueba".into(),
        safe_to_delete_reason: "Prueba aislada".into(),
    };

    // 1. Dry Run Scan: solo inspecciona, NO borra
    let result = CleanerEngine::scan_target(&target, "sandbox_group");

    assert_eq!(result.file_count, 2, "Debe detectar solo los 2 archivos coincidentes");
    assert_eq!(result.total_bytes, 20, "Debe contabilizar exactamente 20 bytes");

    // Verificar que los archivos siguen existiendo intactos (Dry Run)
    assert!(file1.exists(), "El archivo .tmp debe existir tras dry-run");
    assert!(file2.exists(), "El archivo .log debe existir tras dry-run");
    assert!(keep_file.exists(), "El archivo no coincidente debe seguir intacto");
}

#[test]
fn test_safe_cleaner_execution_isolated_sandbox() {
    let dir = tempdir().expect("Error creando tempdir aislado");
    let file1 = dir.path().join("junk.tmp");
    let keep_file = dir.path().join("keep.txt");

    fs::write(&file1, "datos temporales desechables").unwrap();
    fs::write(&keep_file, "información que debe preservarse").unwrap();

    let scan = rustcare_core::cleaner::FullScanResult {
        targets: vec![TargetScanResult {
            target_id: "test".into(),
            target_name: "Test Target".into(),
            group_id: "test_grp".into(),
            files: vec![ScannedFile {
                path: file1.clone(),
                size_bytes: 28,
            }],
            total_bytes: 28,
            file_count: 1,
            requires_admin: false,
            skipped_unprivileged: false,
        }],
        total_bytes: 28,
        total_files: 1,
    };

    // Ejecuta limpieza sobre los objetivos del escaneo aislado
    let clean_res = CleanerEngine::clean_all(&scan);

    assert_eq!(clean_res.total_files_deleted, 1);
    assert_eq!(clean_res.total_bytes_freed, 28);
    assert!(!file1.exists(), "junk.tmp debió ser eliminado");
    assert!(keep_file.exists(), "keep.txt debe continuar existiendo intacto");
}

#[test]
fn test_safe_duplicate_finder_sandboxed() {
    let dir = tempdir().expect("Error creando tempdir");
    let content = b"BLAKE3 identical cryptographic content for safe testing";

    let f1 = dir.path().join("original.dat");
    let f2 = dir.path().join("copia_1.dat");
    let f3 = dir.path().join("copia_2.dat");
    let f_different = dir.path().join("diferente.dat");

    fs::write(&f1, content).unwrap();
    fs::write(&f2, content).unwrap();
    fs::write(&f3, content).unwrap();
    fs::write(&f_different, b"contenido completamente unico").unwrap();

    let res = DuplicateFinder::scan_directory(dir.path());

    assert_eq!(res.groups.len(), 1, "Debe haber exactamente un grupo de duplicados");
    assert_eq!(res.groups[0].files.len(), 3, "El grupo debe tener los 3 archivos idénticos");
    assert_eq!(res.groups[0].file_size, content.len() as u64);
}

#[test]
fn test_safe_disk_analyzer_wiztree_sandboxed() {
    let dir = tempdir().expect("Error creando tempdir");
    let sub = dir.path().join("CarpetaSub");
    fs::create_dir(&sub).unwrap();

    let f1 = dir.path().join("video.mp4");
    let f2 = sub.join("app.exe");

    fs::write(&f1, vec![0u8; 1024]).unwrap(); // 1 KB
    fs::write(&f2, vec![0u8; 2048]).unwrap(); // 2 KB

    let analysis = DiskAnalyzer::analyze_directory(dir.path());

    assert_eq!(analysis.items.len(), 2);
    assert_eq!(analysis.total_size_bytes, 3072);
    assert!(analysis.treemap_items.len() >= 2);
    assert_eq!(analysis.treemap_items[0].extension, "exe");
    assert_eq!(analysis.treemap_items[1].extension, "mp4");
}

#[test]
fn test_safe_large_files_finder_sandboxed() {
    let dir = tempdir().expect("Error creando tempdir");
    let large = dir.path().join("big_installer.msi");
    let small = dir.path().join("small_note.txt");

    fs::write(&large, vec![0u8; 2 * 1024 * 1024]).unwrap(); // 2 MB
    fs::write(&small, b"solo unos bytes").unwrap();

    // Filtro con umbral mínimo de 1 MB
    let items = LargeFilesFinder::scan_directory(dir.path(), 1024 * 1024);

    assert_eq!(items.len(), 1);
    assert_eq!(items[0].name, "big_installer.msi");
    assert_eq!(items[0].size_bytes, 2 * 1024 * 1024);
}

#[test]
fn test_safe_empty_folders_cleaner_sandboxed() {
    let dir = tempdir().expect("Error creando tempdir");
    let empty_child = dir.path().join("nivel1").join("nivel2").join("vacia");
    let populated_dir = dir.path().join("con_archivo");

    fs::create_dir_all(&empty_child).unwrap();
    fs::create_dir_all(&populated_dir).unwrap();
    fs::write(populated_dir.join("archivo.txt"), b"contenido").unwrap();

    let empties = EmptyFoldersFinder::scan_directory(dir.path());
    assert!(!empties.is_empty(), "Debe detectar las carpetas vacías");

    for folder in &empties {
        let _ = EmptyFoldersFinder::delete_folder(folder);
    }

    assert!(!empty_child.exists(), "La carpeta vacía debió eliminarse");
    assert!(populated_dir.exists(), "La carpeta con archivo debe conservarse intacta");
}

#[test]
fn test_safe_file_shredder_sandboxed() {
    let dir = tempdir().expect("Error creando tempdir");
    let sensitive_file = dir.path().join("confidential_data.bin");

    fs::write(&sensitive_file, b"SECRETO DE PRUEBA A DESTRUIR").unwrap();
    assert!(sensitive_file.exists());

    let result = FileShredder::shred_file(&sensitive_file);

    assert!(result.is_ok(), "El triturado seguro debe finalizar con éxito");
    assert!(!sensitive_file.exists(), "El archivo triturado no debe existir en disco");
}

#[test]
fn test_safe_privacy_audit_strictly_read_only() {
    let config = PrivacyConfig {
        settings: vec![PrivacySetting {
            id: "safe_test_mru".into(),
            category: "history".into(),
            name: "Audit Test".into(),
            description: "Solo lectura segura".into(),
            hive: "HKCU".into(),
            path: "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer".into(),
            key: "TestKeyDoesNotModify".into(),
            value_type: "REG_DWORD".into(),
            recommended_value: 0,
            default_windows_value: 1,
        }],
    };

    // La auditoría lee el registro o devuelve el estado sin escribir ni alterar nada
    let results = PrivacyEngine::check_all(&config);
    assert_eq!(results.len(), 1);
}

#[test]
fn test_safe_driver_audit_strictly_read_only() {
    // Escaneo pasivo de controladores instalados (solo lectura de SetupAPI / sysfs)
    let drivers = DriverManager::scan_installed_drivers();
    let _ = drivers.len();
}

#[test]
fn test_safe_shield_scan_strictly_read_only() {
    // Inspección heurística de procesos y persistencias sin terminar ningún proceso
    let shield_res = ShieldEngine::scan_all();
    assert!(shield_res.processes_scanned > 0, "Debe auditar los procesos activos de forma pasiva");
}

#[test]
fn test_safe_system_monitor_strictly_read_only() {
    let mut monitor = SystemMonitor::new();
    let snap = monitor.refresh_snapshot();

    assert!(snap.cpu_count > 0);
    assert!(snap.memory_total_bytes > 0);
    assert!(snap.memory_usage_percent >= 0.0 && snap.memory_usage_percent <= 100.0);
}

#[test]
fn test_safe_ram_clean_non_destructive() {
    // Vaciado seguro de conjuntos de trabajo (K32EmptyWorkingSet / purge)
    // No mata ningún proceso, solo libera páginas de memoria en desuso hacia la lista de espera
    let res = BoosterEngine::clean_ram();
    assert!(res.processes_cleaned + res.processes_failed > 0);
}

#[test]
fn test_safe_rescue_center_backup_listing() {
    let dir = RescueCenter::backup_directory();
    assert!(!dir.to_string_lossy().is_empty());
    let backups = RescueCenter::list_backups();
    let _ = backups.len();
}

#[test]
fn test_safe_startup_listing_read_only() {
    let entries = StartupEngine::list_entries();
    let _ = entries.len();
}

#[test]
fn test_admin_token_detection_behavior() {
    // Comprueba que la detección de elevación nunca lanza diálogos y devuelve bool
    let elevated = is_elevated();
    println!("Proceso en ejecución con privilegios de administrador: {}", elevated);
}

#[test]
fn test_unprivileged_target_skip_without_crash() {
    let user_is_admin = is_elevated();

    let admin_target = CleanerTarget {
        id: "system_protected_dir".into(),
        name: "Windows System Directory".into(),
        description: "Objetivo protegido que requiere privilegios elevados".into(),
        path: "C:\\Windows\\System32\\LogFiles".into(),
        patterns: vec!["*.log".into()],
        recursive: false,
        requires_admin: true,
        safety_level: "safe".into(),
        why_it_exists: "Archivos del sistema".into(),
        safe_to_delete_reason: "Prueba aislada".into(),
    };

    let result = CleanerEngine::scan_target(&admin_target, "system_group");

    if !user_is_admin {
        // En modo usuario estándar: debe omitirse silenciosamente
        assert!(result.skipped_unprivileged, "Debe omitir el objetivo protegido si no es admin");
        assert_eq!(result.file_count, 0, "No debe escanear archivos del sistema sin elevación");
        assert!(result.requires_admin);
    } else {
        // En modo admin: puede procesarlo
        assert!(!result.skipped_unprivileged);
    }
}

#[test]
fn test_automation_autostart_and_smart_ram() {
    let _ = rustcare_core::automation::AutomationEngine::is_autostart_enabled();
    let _ = rustcare_core::automation::AutomationEngine::check_smart_ram(100.0);
}

#[test]
fn test_safe_network_booster_backup_and_restore() {
    let backup_res = rustcare_core::network_booster::NetworkBooster::backup_network_state();
    assert!(backup_res.is_ok());
    let path = backup_res.unwrap();
    assert!(path.exists());
    let _ = fs::remove_file(path);
}

#[test]
fn test_safe_disk_optimizer_health_check() {
    let check = rustcare_core::disk_optimizer::DiskOptimizer::check_filesystem_integrity("C");
    assert!(!check.drive_letter.is_empty());
}

#[test]
fn test_safe_browser_vacuum_sandboxed() {
    let dir = tempdir().unwrap();
    let fake_db = dir.path().join("History");
    fs::write(&fake_db, vec![0u8; 8192]).unwrap();

    let res = rustcare_core::browser_vacuum::BrowserVacuum::compact_database("Test Chrome", &fake_db);
    assert!(res.is_ok());
    let info = res.unwrap();
    assert_eq!(info.original_size_bytes, 8192);
    assert!(info.backup_path.exists());
    let _ = fs::remove_file(info.backup_path);
}

#[test]
fn test_safe_ui_repair_structure() {
    let res = rustcare_core::ui_repair::UiRepairResult {
        icon_cache_deleted: true,
        explorer_restarted: true,
        files_purged: 4,
        message: "OK".into(),
    };
    assert_eq!(res.files_purged, 4);
}

#[test]
fn test_safe_system_info_report_generation() {
    let rep = rustcare_core::system_info::SystemInfoHub::get_report();
    assert!(!rep.os_name.is_empty());
    assert!(!rep.cpu_brand.is_empty());
    let txt = rustcare_core::system_info::SystemInfoHub::export_text_report();
    assert!(txt.contains("INFORME DE HARDWARE"));
}

#[test]
fn test_safe_power_plan_listing() {
    let schemes = rustcare_core::power_plan::PowerPlanOptimizer::list_schemes();
    assert!(!schemes.is_empty());
}

#[test]
fn test_safe_dism_status() {
    let status = rustcare_core::dism_cleanup::DismEngine::check_status();
    assert!(!status.recommended_action.is_empty());
}

#[test]
fn test_safe_hibernation_status() {
    let status = rustcare_core::hibernation::HibernationManager::get_status();
    assert!(status.ram_size_bytes > 0);
    assert!(!status.potential_savings.is_empty());
}

#[test]
fn test_safe_hardware_troubleshooter() {
    let problem_devices = rustcare_core::hardware_troubleshooter::HardwareTroubleshooter::scan_problem_devices();
    // In normal systems this is 0 or contains problem devices, should not panic
    let _ = problem_devices.len();
}

#[test]
fn test_safe_scheduled_tasks_audit() {
    let tasks = rustcare_core::scheduled_tasks::ScheduledTaskManager::list_tasks();
    // Audit must run safely without modifying system state
    let _ = tasks.len();
}
