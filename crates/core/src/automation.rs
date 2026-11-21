//! Motor de Automatización, Auto-Limpieza, Auto-RAM y Notificaciones ("AutoCare")
use crate::booster::{BoosterEngine, RamCleanResult};
use crate::cleaner::CleanerEngine;
use crate::monitor::SystemMonitor;
use crate::rules::RulesEngine;
use std::env;
#[cfg_attr(windows, allow(unused_imports))]
use std::path::PathBuf;
use std::process::Command;

#[cfg(windows)]
use winreg::enums::{HKEY_CURRENT_USER, KEY_ALL_ACCESS};
#[cfg(windows)]
use winreg::RegKey;

pub struct AutomationEngine;

impl AutomationEngine {
    #[cfg(windows)]
    const RUN_KEY: &'static str = "Software\\Microsoft\\Windows\\CurrentVersion\\Run";

    /// Comprueba si RustCare está configurado para iniciar automáticamente con el sistema
    pub fn is_autostart_enabled() -> bool {
        #[cfg(windows)]
        {
            let hkcu = RegKey::predef(HKEY_CURRENT_USER);
            if let Ok(key) = hkcu.open_subkey(Self::RUN_KEY) {
                key.get_value::<String, _>("RustCare").is_ok()
            } else {
                false
            }
        }

        #[cfg(target_os = "linux")]
        {
            if let Ok(home) = env::var("HOME") {
                PathBuf::from(home)
                    .join(".config/autostart/rustcare.desktop")
                    .exists()
            } else {
                false
            }
        }

        #[cfg(target_os = "macos")]
        {
            if let Ok(home) = env::var("HOME") {
                PathBuf::from(home)
                    .join("Library/LaunchAgents/com.rustcare.agent.plist")
                    .exists()
            } else {
                false
            }
        }

        #[cfg(not(any(windows, target_os = "linux", target_os = "macos")))]
        {
            false
        }
    }

    /// Habilita o deshabilita el inicio automático con el sistema (100% espacio de usuario)
    pub fn set_autostart(enabled: bool) -> anyhow::Result<()> {
        let exe_path = env::current_exe()?;
        let exe_str = exe_path.to_string_lossy().to_string();

        #[cfg(windows)]
        {
            let hkcu = RegKey::predef(HKEY_CURRENT_USER);
            let (key, _) = hkcu.create_subkey(Self::RUN_KEY)?;

            if enabled {
                let cmd = format!("\"{}\" --minimized", exe_str);
                key.set_value("RustCare", &cmd)?;
            } else if let Ok(key_write) = hkcu.open_subkey_with_flags(Self::RUN_KEY, KEY_ALL_ACCESS) {
                let _ = key_write.delete_value("RustCare");
            }
            Ok(())
        }

        #[cfg(target_os = "linux")]
        {
            if let Ok(home) = env::var("HOME") {
                let autostart_dir = PathBuf::from(home).join(".config/autostart");
                std::fs::create_dir_all(&autostart_dir)?;
                let file_path = autostart_dir.join("rustcare.desktop");

                if enabled {
                    let content = format!(
                        "[Desktop Entry]\nType=Application\nName=RustCare\nExec={} --minimized\nHidden=false\nNoDisplay=false\nX-GNOME-Autostart-enabled=true\n",
                        exe_str
                    );
                    std::fs::write(file_path, content)?;
                } else if file_path.exists() {
                    let _ = std::fs::remove_file(file_path);
                }
            }
            Ok(())
        }

        #[cfg(target_os = "macos")]
        {
            if let Ok(home) = env::var("HOME") {
                let agents_dir = PathBuf::from(home).join("Library/LaunchAgents");
                std::fs::create_dir_all(&agents_dir)?;
                let file_path = agents_dir.join("com.rustcare.agent.plist");

                if enabled {
                    let content = format!(
                        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">\n<plist version=\"1.0\">\n<dict>\n<key>Label</key><string>com.rustcare.agent</string>\n<key>ProgramArguments</key><array><string>{}</string><string>--minimized</string></array>\n<key>RunAtLoad</key><true/>\n</dict>\n</plist>",
                        exe_str
                    );
                    std::fs::write(file_path, content)?;
                } else if file_path.exists() {
                    let _ = std::fs::remove_file(file_path);
                }
            }
            Ok(())
        }

        #[cfg(not(any(windows, target_os = "linux", target_os = "macos")))]
        {
            let _ = enabled;
            Ok(())
        }
    }

    /// Comprueba la memoria RAM y ejecuta vaciado si supera el umbral configurado (ej: 80%)
    pub fn check_smart_ram(threshold_percent: f32) -> Option<RamCleanResult> {
        let mut monitor = SystemMonitor::new();
        let snap = monitor.refresh_snapshot();

        if snap.memory_usage_percent >= threshold_percent {
            Some(BoosterEngine::clean_ram())
        } else {
            None
        }
    }

    /// Comprueba el volumen de basura acumulada en el sistema
    pub fn check_high_junk(threshold_mb: u64) -> Option<u64> {
        if let Ok(cleaner_cfg) = RulesEngine::load_cleaner_rules() {
            let scan = CleanerEngine::scan_all(&cleaner_cfg);
            let threshold_bytes = threshold_mb * 1024 * 1024;
            if scan.total_bytes >= threshold_bytes {
                return Some(scan.total_bytes);
            }
        }
        None
    }

    /// Emite una notificación de escritorio nativa del sistema operativo
    pub fn send_notification(title: &str, message: &str) -> bool {
        #[cfg(windows)]
        {
            // En Windows se usa PowerShell con sintaxis segura para emitir el toast nativo
            let script = format!(
                "[Windows.UI.Notifications.ToastNotificationManager, Windows.UI.Notifications, ContentType = WindowsRuntime] > $null; \
                 $template = [Windows.UI.Notifications.ToastNotificationManager]::GetTemplateContent([Windows.UI.Notifications.ToastTemplateType]::ToastText02); \
                 $textNodes = $template.GetElementsByTagName('text'); \
                 $textNodes.Item(0).AppendChild($template.CreateTextNode('{}')) > $null; \
                 $textNodes.Item(1).AppendChild($template.CreateTextNode('{}')) > $null; \
                 $toast = [Windows.UI.Notifications.ToastNotification]::new($template); \
                 [Windows.UI.Notifications.ToastNotificationManager]::CreateToastNotifier('RustCare').Show($toast);",
                title.replace('\'', "''"),
                message.replace('\'', "''")
            );

            let res = Command::new("powershell")
                .args(["-NoProfile", "-NonInteractive", "-Command", &script])
                .output();

            res.map(|o| o.status.success()).unwrap_or(false)
        }

        #[cfg(target_os = "linux")]
        {
            let res = Command::new("notify-send")
                .arg(title)
                .arg(message)
                .output();
            res.map(|o| o.status.success()).unwrap_or(false)
        }

        #[cfg(target_os = "macos")]
        {
            let script = format!(
                "display notification \"{}\" with title \"{}\"",
                message.replace('"', "\\\""),
                title.replace('"', "\\\"")
            );
            let res = Command::new("osascript")
                .arg("-e")
                .arg(&script)
                .output();
            res.map(|o| o.status.success()).unwrap_or(false)
        }

        #[cfg(not(any(windows, target_os = "linux", target_os = "macos")))]
        {
            let _ = title;
            let _ = message;
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_autostart_query_runs() {
        let _ = AutomationEngine::is_autostart_enabled();
    }

    #[test]
    fn test_smart_ram_threshold() {
        // Con umbral de 100%, normalmente no se dispara en test salvo que la RAM esté llena
        let res = AutomationEngine::check_smart_ram(100.0);
        let _ = res.is_some();
    }
}
