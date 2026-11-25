# RustCare Design System (`DESIGN.md`)

> Sistema de diseño visual y reglas de arquitectura UI/UX para **RustCare PRO** (inspirado en *Spatial Obsidian & Volumetric Glass*, *shadcn/ui*, *Tremor*, *Recharts*, *Sonner*, *Vaul*, *cmdk* y *pbakaus/impeccable*).

---

## 1. Tokens de Color (`crates/gui/src/theme.rs`)

| Token | Hex / RGB | Uso Semántico |
| :--- | :--- | :--- |
| `BG_VOID` | `#0B0F17` | Fondo base de ventana y lienzo central |
| `BG_SIDEBAR` | `#0D121B` | Panel de navegación lateral |
| `BG_SURFACE` | `#121825` | Superficie de tarjetas y contenedores Bento |
| `BG_ELEVATED` | `#1A2334` | Inputs, filas, botones secundarios y controles |
| `BG_HOVER` | `#222E44` | Estado hover interactivo |
| `BORDER_SUBTLE` | `#202B3E` | Bordes estándar `1.0px` |
| `ACCENT_TEAL` | `#0EA5E9` | Acento principal de acción |
| `ACCENT_CYAN` | `#38BDF8` | Acento secundario, telemetría y foco activo |
| `ACCENT_EMERALD` | `#10B981` | Estado óptimo, salud 100%, éxito |
| `ACCENT_INDIGO` | `#818CF8` | Memoria RAM / métricas de capacidad |
| `ACCENT_AMBER` | `#F59E0B` | Advertencias y Modo Sandbox |
| `ACCENT_CORAL` | `#F43F5E` | Peligro, Destructor DoD y Modo Real |
| `TEXT_PRIMARY` | `#F8FAFC` | Títulos y valores clave de alto contraste |
| `TEXT_MUTED` | `#94A3B8` | Subtítulos, etiquetas técnicas y descripciones |

---

## 2. Reglas de Superficie y Profundidad

1. **Tarjetas de Cristal Espacial (`draw_glass_card`)**:
   - Radio de esquina uniforme (`14px` en Bento, `10px` en Toasts/Badges).
   - Luz especular superior (`1.0px` blanco translúcido) + cápsula de proyección neón inferior (*underglow*) centrada en el borde inferior.
2. **Atmósfera Ambiental (`draw_ambient_mesh_glow`)**:
   - Halos difusos de baja opacidad (`alpha 0.04–0.05`) detrás del contenido central para evitar fondos planos.
3. **Capas de Interacción**:
   - **Toasts (`toast.rs`, estilo *Sonner*)**: esquina inferior derecha, máx. 4 simultáneos, barra de expiración y cierre manual.
   - **Command Palette (`cmdk.rs`, estilo *cmdk*)**: `Ctrl+K` global, búsqueda difusa sobre vistas, herramientas y acciones rápidas.
   - **Slide-over Drawer (`widgets::slide_over_drawer`, estilo *Vaul*)**: inspección de detalles sin abandonar el Bento Grid.

---

## 3. Reglas Anti-Slop (Calidad Determinista)

- **Prohibido usar widgets por defecto de egui** (`ui.button`, `ui.checkbox`, `ui.radio_value`) en las vistas; usar siempre las primitivas de `crates/gui/src/widgets.rs` (`neon_button`, `ghost_button`, `danger_button`, `pill_toggle`, `checkbox_item`).
- **Sin telemetría simulada**: Toda métrica mostrada en tarjetas (`CPU`, `RAM`, `Disco`) debe provenir de `SystemBackend::get_metrics()` con suavizado EMA y throttle >= 900 ms.
- **Idioma y Tipografía**: 100% español en textos visibles; prohibido usar emojis Unicode del sistema (usar `crates/gui/src/icons.rs`).
- **Validación Visual**: Usar `rustcare-gui --gallery` para revisar todos los componentes en todos sus estados antes de publicar cambios visuales.
