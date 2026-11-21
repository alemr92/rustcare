# ROADMAP: RustCare - Windows 10/11 Optimization & Security Suite (Pro Edition)

RustCare es una suite de optimización, limpieza, mantenimiento, privacidad y seguridad para Windows 10/11 desarrollada en Rust nativo, inspirada en las mejores funcionalidades de **Advanced SystemCare Pro / Ultimate**, **Glary Utilities Pro** y **WizTree**, con enfoque de **seguridad primero**, **cero privilegios de administrador obligatorios** y **100% código abierto**.

---

## Filosofía de Diseño y Requisitos Centrales

1. **Permisos de Administrador Opcionales (Zero-Admin Required):**
   - La suite funciona al 100% como usuario estándar (Medium Integrity).
   - Limpieza de temporales, cachés, historiales, arranque de usuario, duplicados, analizador de disco, destructor de archivos, buscador de archivos gigantes, carpetas vacías, optimizador de RAM y WinGet **no requieren elevación**.
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

## Comparativa Exhaustiva de Funcionalidades (Advanced SystemCare Pro vs RustCare)

| Módulo | Funcionalidad en Advanced SystemCare (Free & PRO) | Implementación en RustCare Pro | Estado en RustCare |
| :--- | :--- | :--- | :---: |
| **Care (1-Clic)** | Escaneo y reparación unificada de múltiples áreas con 1 botón. | Motor coordinado de limpieza, privacidad, basura, memoria y seguridad. | ✅ Implementado |
| **Limpieza de Basura** | Junk Files Cleaner (Temporales, Logs, Dumps, Reciclaje). | Limpiador declarativo multihilo (`walkdir` + `rayon`) basado en TOML. | ✅ Implementado |
| **Privacidad de Navegadores** | Privacy Sweep (Historiales, cookies, sesiones, compactar SQLite). | Limpiador granular por casillas para Chrome, Edge, Firefox, Brave. | ✅ Implementado |
| **Privacidad del Sistema** | Windows Privacy Protector (Telemetría, Ads, Cortana, Bing). | Modificador declarativo del Registro con respaldo `.reg` automático. | ✅ Implementado |
| **Accesos Directos** | Invalid Shortcuts Fixer. | Parser nativo de binarios `.lnk` (MS-SHLLINK) en Escritorio y Menú Inicio. | ✅ Implementado |
| **Buscador Archivos Grandes (PRO)**| Large Files Finder (Filtros por tamaño >50MB, >100MB, >1GB). | Escáner multihilo clasificado por categoría (Videos, ISOs, Comprimidos). | 🚀 En Implementación |
| **Carpetas Vacías (PRO)** | Empty Folders Finder (Elimina carpetas huérfanas). | Escáner post-order recursivo para eliminar directorios vacíos. | 🚀 En Implementación |
| **Destructor Seguro (PRO)** | File Shredder (Borrado irrecuperable de archivos). | Sobrescritura con ceros / patrones aleatorios y `sync_all` antes de borrar. | 🚀 En Implementación |
| **Diagrama de Disco (WizTree)** | Disk Space Analyzer (Diagrama visual estilo WizTree / WinDirStat). | Gráfica de bloques ponderados y porcentajes de tamaño por carpeta. | 🚀 En Implementación |
| **Menú Contextual (PRO)** | Context Menu Manager (Gestor de clic derecho). | Auditoría y desactivación de handlers en `HKCU\Software\Classes\*\shellex`. | 🚀 En Implementación |
| **Centro de Rescate (PRO)** | Rescue Center (Puntos de restauración y backups de registro). | Gestor de respaldos `.reg` con fecha y restauración automática con 1 clic. | 🚀 En Implementación |
| **Aceleración (Turbo Boost)** | Turbo Boost (Modo Juego / Trabajo / Ahorro). | Perfiles configurables para suspender/detener servicios innecesarios. | ✅ Implementado |
| **Liberador de Memoria** | Smart RAM / Auto RAM Clean. | Vaciado de Working Sets vía `K32EmptyWorkingSet` de Win32 API. | ✅ Implementado |
| **Gestor de Arranque** | Startup Manager (Apps de inicio, servicios, tareas). | Gestor en `Run`, `RunOnce` y `Startup` con deshabilitación reversible. | ✅ Implementado |
| **Red y DNS** | Internet Booster / DNS Flush. | Vaciado de caché DNS (`ipconfig /flushdns` y ajustes de red). | ✅ Implementado |
| **Antivirus / Malware** | Spyware & Malware Scanner (IObit Anti-Malware). | **RustCare Shield**: Procesos en temp, impostores, IFEO y hashes `blake3`. | ✅ Implementado |
| **Software Updater** | Software Updater (Catálogo de terceros). | Integración nativa con **WinGet** (`winget upgrade`), oficial de Microsoft. | ✅ Implementado |
| **Desinstalador Profundo** | IObit Uninstaller (con rastreo de residuos). | Lanzamiento de desinstalador nativo + escaneo de carpetas residuales. | ✅ Implementado |
| **Buscador de Duplicados** | Cloned Files Finder. | Algoritmo en 3 fases: tamaño -> hash rápido 4KB -> hash `blake3`. | ✅ Implementado |
| **Monitor en Tiempo Real** | Performance Monitor (Desktop widget con CPU/RAM). | Monitor en vivo con gráficos de CPU, RAM, Discos y Top procesos. | ✅ Implementado |
| **Persistencia de Ajustes** | Guardado de preferencias de usuario. | Almacenamiento en `%LOCALAPPDATA%\RustCare\settings.json`. | ✅ Implementado |
