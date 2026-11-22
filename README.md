# RustCare

A high-performance, zero-telemetry, zero-admin system maintenance and optimization suite built in Rust. Multiplatform support for Windows, Linux, and macOS.

## Highlights

- **Multiplatform Architecture**: Runs natively on Windows 10/11, Linux (systemd/XDG), and macOS (Apple Silicon & Intel).
- **Zero Mandatory Elevation**: Operates fully within standard user integrity without intrusive UAC, sudo, or privilege elevation requirements.
- **1-Click Care**: Comprehensive scan and repair covering temporary junk, browser privacy, working sets, and security persistence checks.
- **Clean Engine**: Fast parallel cleaning of system cache, crash dumps, developer caches (Cargo, npm, pip, Gradle, NuGet), and browser data.
- **Turbo Boost & Working Set Optimizer**: Safe Win32 `K32EmptyWorkingSet` memory compaction on Windows, kernel cache management on Linux, and native `purge` on macOS, alongside automated DNS cache flushing.
- **WizTree-Style Treemap**: Interactive, responsive disk space visualizer with extension color mapping.
- **Security Shield**: Heuristic scanner checking for processes running from temporary paths, IFEO debugger hijacks, and registry autostarts.
- **Toolbox PRO**: Includes driver hardware auditor with automated backup, duplicate finder (BLAKE3), large files filter, empty folders cleaner, broken shortcut fixer, secure shredder, context menu manager, package updater (WinGet / Flatpak / Homebrew), TCP/IP internet booster, SSD TRIM optimizer, browser SQLite VACUUM compactor, icon cache rebuilder, hardware information hub, power plan optimizer, WinSxS DISM component store cleanup, hiberfil.sys hibernation manager, PnP device conflict troubleshooter, and orphaned scheduled tasks auditor.
- **Rescue Center & Non-Destructive Rollback**: Automated registry snapshots, system restore points, TCP/IP state backups, and browser SQLite `.bak` preservation before any write action.
- **AutoCare & Smart Automation**: Background daemon for memory thresholds, automated junk detection, and desktop toast notifications.

## Architecture

The project is structured as a Cargo workspace:
- `crates/core`: Optimization engines, system inspection, rule parsers, and backup managers.
- `crates/gui`: Modern dark interface using `eframe` and `egui`.
- `rules/`: TOML configuration definitions for cleaning, booster, and privacy policies.

## Building from Source

Prerequisites:
- Rust toolchain (`stable` channel)

```bash
# Build the release binary
cargo build --release -p rustcare-gui

# Run unit tests across the workspace
cargo test --workspace
```

The resulting executable is located at `target/release/rustcare-gui.exe`.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
