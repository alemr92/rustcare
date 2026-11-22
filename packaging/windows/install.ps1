# Instaldor Automático de RustCare para Windows (PowerShell)
param (
    [string]$InstallDir = "$env:LOCALAPPDATA\Programs\RustCare"
)

$ErrorActionPreference = "Stop"
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "   Instalador Nativo de RustCare v0.1.0 " -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Cyan

$CurrentDir = Split-Path -Parent $MyInvocation.MyCommand.Definition
$SourceExe = Join-Path $CurrentDir "..\..\target\release\rustcare-gui.exe"
$SourceRules = Join-Path $CurrentDir "..\..\rules"

if (-not (Test-Path $SourceExe)) {
    # Si viene desde un paquete distribuido
    $SourceExe = Join-Path $CurrentDir "rustcare-gui.exe"
    $SourceRules = Join-Path $CurrentDir "rules"
}

if (-not (Test-Path $SourceExe)) {
    Write-Error "No se encontró el ejecutable 'rustcare-gui.exe'. Por favor compila o descarga el binario antes de ejecutar el instalador."
}

Write-Host "[1/3] Creando directorio de instalación en: $InstallDir..." -ForegroundColor Yellow
New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
New-Item -ItemType Directory -Force -Path (Join-Path $InstallDir "rules") | Out-Null

Write-Host "[2/3] Copiando binarios y reglas de optimización..." -ForegroundColor Yellow
Copy-Item -Path $SourceExe -Destination (Join-Path $InstallDir "rustcare-gui.exe") -Force
if (Test-Path $SourceRules) {
    Copy-Item -Path "$SourceRules\*" -Destination (Join-Path $InstallDir "rules") -Recurse -Force
}

Write-Host "[3/3] Creando accesos directos..." -ForegroundColor Yellow
$WshShell = New-Object -ComObject WScript.Shell

# Acceso directo en el Menú Inicio del usuario (Zero-Admin)
$ProgramsDir = [Environment]::GetFolderPath("Programs")
$StartShortcut = $WshShell.CreateShortcut((Join-Path $ProgramsDir "RustCare.lnk"))
$StartShortcut.TargetPath = Join-Path $InstallDir "rustcare-gui.exe"
$StartShortcut.WorkingDirectory = $InstallDir
$StartShortcut.Description = "RustCare - Windows Optimization & Security Suite"
$StartShortcut.Save()

# Acceso directo en el Escritorio
$DesktopDir = [Environment]::GetFolderPath("Desktop")
$DesktopShortcut = $WshShell.CreateShortcut((Join-Path $DesktopDir "RustCare.lnk"))
$DesktopShortcut.TargetPath = Join-Path $InstallDir "rustcare-gui.exe"
$DesktopShortcut.WorkingDirectory = $InstallDir
$DesktopShortcut.Description = "RustCare - Windows Optimization & Security Suite"
$DesktopShortcut.Save()

Write-Host "`n¡Instalación completada con éxito!" -ForegroundColor Green
Write-Host "Se ha creado el acceso directo en tu Escritorio y Menú Inicio."
Write-Host "Ubicación: $InstallDir\rustcare-gui.exe"
