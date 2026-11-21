# ROADMAP: RustCare - Windows 10/11 Optimization & Security Suite (Pro Edition)

RustCare es una suite de optimización, limpieza, mantenimiento, privacidad y seguridad para Windows 10/11 desarrollada en Rust nativo, inspirada en las mejores funcionalidades de **Advanced SystemCare Pro / Ultimate**, **Glary Utilities Pro** y **WizTree**, con enfoque de **seguridad primero**, **cero privilegios de administrador obligatorios** y **100% código abierto**.

---

## Filosofía de Diseño y Requisitos Centrales

1. **Permisos de Administrador Opcionales (Zero-Admin Required):**
   - La suite funciona al 100% como usuario estándar (Medium Integrity).
   - Limpieza de temporales, cachés, historiales, arranque de usuario, duplicados, analizador de disco, destructor de archivos, buscador de archivos gigantes, carpetas vacías, optimizador de RAM, auditor de drivers y WinGet **no requieren elevación**.
   - Si el usuario ejecuta la aplicación con privilegios elevados (UAC), se desbloquean opcionalmente accesos globales (`%SystemRoot%\Temp`, servicios `HKLM`, TRIM de disco). Si no está elevado, se omiten limpiamente con un badge discreto `[Requiere Admin - Opcional]`, sin ventanas emergentes de UAC ni bloqueos.
2. **Selección Granular de Privacidad y Navegadores (Estilo ASC Pro):**
   - Selección casilla a casilla por navegador (Chrome, Edge, Firefox, Brave, Opera):
     - **Caché Web / HTTP Cache**, **Caché GPU / Code Cache V8**, **Historial de Navegación y Descargas**, **Cookies**, **Sesiones Restaurables**.
     - **Compactación y Optimización de Bases de Datos SQLite (Browser Database VACUUM)**.
     - **Protección por defecto**: Contraseñas guardadas siempre excluidas y protegidas.
3. **Seguridad y Respaldo Reversible:**
   - Modo **Dry-Run (Simulación)** en todos los escaneos con cálculo exacto de bytes.
   - Centro de Rescate (**Rescue Center**) con copias de seguridad `.reg` automáticas en `%LocalAppData%\RustCare\backups\` con restauración en 1 clic.
   - Deshabilitación no destructiva de inicio (`HKCU\Software\RustCare\DisabledStartup`).
4. **Diseño Visual Moderno, Fluido y Accesible:**
   - Interfaz en `eframe`/`egui` con modo oscuro refinado, esquinas redondeadas, navegación lateral por iconos y categorías claras.
   - Ejecución asíncrona mediante hilos de trabajo y canales (`crossbeam-channel`), garantizando 60 FPS estables y cero congelamiento.

---

## Comparativa Exhaustiva de Funcionalidades (Advanced SystemCare Pro vs RustCare Pro)

| Módulo | Funcionalidad en Advanced SystemCare (Free & PRO) | Implementación en RustCare Pro | Estado en RustCare |
| :--- | :--- | :--- | :---: |
| **Care (1-Clic)** | Escaneo y reparación unificada de múltiples áreas con 1 botón. | Motor coordinado de limpieza, privacidad, basura, memoria y seguridad. | ✅ **100% Implementado** |
| **Limpieza de Basura** | Junk Files Cleaner (Temporales, Logs, Dumps, Reciclaje). | Limpiador declarativo multihilo (`walkdir` + `rayon`) basado en TOML. | ✅ **100% Implementado** |
| **Privacidad de Navegadores** | Privacy Sweep (Historiales, cookies, sesiones, compactar SQLite). | Limpiador granular por casillas para Chrome, Edge, Firefox, Brave. | ✅ **100% Implementado** |
| **Privacidad del Sistema** | Windows Privacy Protector (Telemetría, Ads, Cortana, Bing). | Modificador declarativo del Registro con respaldo `.reg` automático. | ✅ **100% Implementado** |
| **Accesos Directos** | Invalid Shortcuts Fixer. | Parser nativo de binarios `.lnk` (MS-SHLLINK) en Escritorio y Menú Inicio. | ✅ **100% Implementado** |
| **Driver Booster (PRO)** | Driver Booster (Auditoría y actualización de controladores). | Auditor de drivers en `HKLM\SYSTEM\CurrentControlSet\Control\Class`. | ✅ **100% Implementado** |
| **Archivos Grandes (PRO)** | Large Files Finder (Filtros por tamaño >50MB, >100MB, >1GB). | Escáner multihilo clasificado por categoría (Videos, ISOs, Comprimidos). | ✅ **100% Implementado** |
| **Carpetas Vacías (PRO)** | Empty Folders Finder (Elimina carpetas huérfanas). | Escáner post-order recursivo para eliminar directorios vacíos. | ✅ **100% Implementado** |
| **Destructor Seguro (PRO)** | File Shredder (Borrado irrecuperable de archivos). | Sobrescritura con ceros y forzado `sync_all` antes de borrar. | ✅ **100% Implementado** |
| **Diagrama de Disco (WizTree)**| Disk Space Analyzer (Diagrama visual estilo WizTree / WinDirStat). | Gráfica de bloques ponderados y porcentajes de tamaño por carpeta. | ✅ **100% Implementado** |
| **Menú Contextual (PRO)** | Context Menu Manager (Gestor de clic derecho). | Auditoría y desactivación de handlers en `HKCU\Software\Classes\*\shellex`. | ✅ **100% Implementado** |
| **Centro de Rescate (PRO)** | Rescue Center (Puntos de restauración y backups de registro). | Gestor de respaldos `.reg` con fecha y restauración automática con 1 clic. | ✅ **100% Implementado** |
| **Aceleración (Turbo Boost)** | Turbo Boost (Modo Juego / Trabajo / Ahorro). | Perfiles configurables para suspender/detener servicios innecesarios. | ✅ **100% Implementado** |
| **Liberador de Memoria** | Smart RAM / Auto RAM Clean. | Vaciado de Working Sets vía `K32EmptyWorkingSet` de Win32 API. | ✅ **100% Implementado** |
| **Gestor de Arranque** | Startup Manager (Apps de inicio, servicios, tareas). | Gestor en `Run`, `RunOnce` y `Startup` con deshabilitación reversible. | ✅ **100% Implementado** |
| **Red y DNS** | Internet Booster / DNS Flush. | Vaciado de caché DNS (`ipconfig /flushdns` y ajustes de red). | ✅ **100% Implementado** |
| **Antivirus / Malware** | Spyware & Malware Scanner (IObit Anti-Malware). | **RustCare Shield**: Procesos en temp, impostores, IFEO y hashes `blake3`. | ✅ **100% Implementado** |
| **Software Updater** | Software Updater (Catálogo de terceros). | Integración nativa con **WinGet** (`winget upgrade`), oficial de Microsoft. | ✅ **100% Implementado** |
| **Desinstalador Profundo** | IObit Uninstaller (con rastreo de residuos). | Lanzamiento de desinstalador nativo + escaneo de carpetas residuales. | ✅ **100% Implementado** |
| **Buscador de Duplicados** | Cloned Files Finder. | Algoritmo en 3 fases: tamaño -> hash rápido 4KB -> hash `blake3`. | ✅ **100% Implementado** |
| **Monitor en Tiempo Real** | Performance Monitor (Desktop widget con CPU/RAM). | Monitor en vivo con gráficos de CPU, RAM, Discos y Top procesos. | ✅ **100% Implementado** |
| **Persistencia de Ajustes** | Guardado de preferencias de usuario. | Almacenamiento en `%LOCALAPPDATA%\RustCare\settings.json`. | ✅ **100% Implementado** |

---

## Fases del Roadmap (100% Completadas)

- [x] **Fase 1: Cimientos y Estructura Base:** Workspace, reglas declarativas TOML, permisos Zero-Admin.
- [x] **Fase 2: Motor de Limpieza, Privacidad y Monitor:** WalkDir sin reparse points, Dry-Run, backup .reg automático, SysInfo en vivo.
- [x] **Fase 3: Aceleración, Arranque, Duplicados, WinGet y Shield:** K32EmptyWorkingSet, persistencias reversibles, blake3 multihilo, auditoría de malware y procesos en temp.
- [x] **Fase 4: Herramientas Pro y Toolbox:** Driver Booster, Large Files, Empty Folders, File Shredder, WizTree analyzer, Context Menu, Rescue Center, Shortcuts fixer.
- [x] **Fase 5: Interfaz Gráfica Moderna y Fluida en egui:** 6 secciones, modo oscuro refinado, comunicación asíncrona multihilo, 23 tests unitarios pasando.
