//! Comprobaciones de seguridad y privilegios multiplataforma (Windows, Linux, macOS)

#[cfg(windows)]
use windows::Win32::Foundation::{CloseHandle, HANDLE};
#[cfg(windows)]
use windows::Win32::Security::{GetTokenInformation, TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY};
#[cfg(windows)]
use windows::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};

/// Determina si el proceso actual tiene privilegios elevados (Root / Administrador).
/// Esta comprobación es 100% silenciosa y nunca lanza diálogos de UAC ni sudo.
pub fn is_elevated() -> bool {
    #[cfg(windows)]
    {
        unsafe {
            let mut token = HANDLE::default();
            if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token).is_err() {
                return false;
            }

            let mut elevation = TOKEN_ELEVATION::default();
            let mut return_length = 0u32;
            let success = GetTokenInformation(
                token,
                TokenElevation,
                Some(&mut elevation as *mut _ as *mut _),
                std::mem::size_of::<TOKEN_ELEVATION>() as u32,
                &mut return_length,
            );

            let _ = CloseHandle(token);

            if success.is_ok() {
                elevation.TokenIsElevated != 0
            } else {
                false
            }
        }
    }

    #[cfg(unix)]
    {
        std::process::Command::new("id")
            .arg("-u")
            .output()
            .ok()
            .and_then(|out| String::from_utf8(out.stdout).ok())
            .map(|s| s.trim() == "0")
            .unwrap_or(false)
    }

    #[cfg(not(any(windows, unix)))]
    {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_elevated_runs_without_panic() {
        let _elevated = is_elevated();
    }
}
