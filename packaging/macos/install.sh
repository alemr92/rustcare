#!/usr/bin/env bash
# Script de instalación para macOS: ensambla RustCare.app en /Applications
set -e

VERSION="0.1.0"
APP_NAME="RustCare.app"
INSTALL_TARGET="/Applications/${APP_NAME}"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="${SCRIPT_DIR}/../.."
BIN_SRC="${ROOT_DIR}/target/release/rustcare-gui"

if [ ! -f "${BIN_SRC}" ]; then
    BIN_SRC="${SCRIPT_DIR}/rustcare-gui"
fi

if [ ! -f "${BIN_SRC}" ]; then
    echo "Error: No se encontró 'rustcare-gui'. Compila antes con 'cargo build --release -p rustcare-gui'."
    exit 1
fi

echo "=========================================="
echo "    Instalador de RustCare para macOS     "
echo "=========================================="

echo "[1/3] Creando bundle ${APP_NAME}..."
rm -rf "${INSTALL_TARGET}"
mkdir -p "${INSTALL_TARGET}/Contents/MacOS"
mkdir -p "${INSTALL_TARGET}/Contents/Resources/rules"

echo "[2/3] Copiando binarios y metadatos..."
cp "${BIN_SRC}" "${INSTALL_TARGET}/Contents/MacOS/rustcare-gui"
chmod +x "${INSTALL_TARGET}/Contents/MacOS/rustcare-gui"
cp "${SCRIPT_DIR}/Info.plist" "${INSTALL_TARGET}/Contents/Info.plist"

if [ -d "${ROOT_DIR}/rules" ]; then
    cp -r "${ROOT_DIR}/rules/"* "${INSTALL_TARGET}/Contents/Resources/rules/"
fi

echo "[3/3] Registrando aplicación..."
xattr -cr "${INSTALL_TARGET}" 2>/dev/null || true

echo "¡Instalación de RustCare completada con éxito en /Applications/RustCare.app!"
