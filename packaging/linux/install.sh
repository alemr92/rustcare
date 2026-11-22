#!/usr/bin/env bash
# Script de instalación nativo para Linux (Ubuntu, Debian, Fedora, Arch, etc.)
set -e

PREFIX="${PREFIX:-/usr/local}"
BIN_DIR="${PREFIX}/bin"
SHARE_DIR="${PREFIX}/share/rustcare"
APPS_DIR="${PREFIX}/share/applications"

echo "========================================"
echo "    Instalador de RustCare para Linux   "
echo "========================================"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BIN_SRC="${SCRIPT_DIR}/../../target/release/rustcare-gui"

if [ ! -f "${BIN_SRC}" ]; then
    BIN_SRC="${SCRIPT_DIR}/rustcare-gui"
fi

if [ ! -f "${BIN_SRC}" ]; then
    echo "Error: No se encontró 'rustcare-gui'. Compila el proyecto con 'cargo build --release -p rustcare-gui' primero."
    exit 1
fi

echo "[1/3] Instalando binario en ${BIN_DIR}..."
sudo install -d "${BIN_DIR}"
sudo install -m 755 "${BIN_SRC}" "${BIN_DIR}/rustcare-gui"

echo "[2/3] Instalando reglas de optimización en ${SHARE_DIR}..."
sudo install -d "${SHARE_DIR}/rules"
if [ -d "${SCRIPT_DIR}/../../rules" ]; then
    sudo cp -r "${SCRIPT_DIR}/../../rules/"* "${SHARE_DIR}/rules/"
fi

echo "[3/3] Registrando lanzador de escritorio..."
sudo install -d "${APPS_DIR}"
sudo install -m 644 "${SCRIPT_DIR}/rustcare.desktop" "${APPS_DIR}/rustcare.desktop"

echo "¡Instalación de RustCare completada con éxito!"
echo "Puedes ejecutarlo desde tu terminal con 'rustcare-gui' o desde tu menú de aplicaciones."
