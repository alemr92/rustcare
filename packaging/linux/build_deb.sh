#!/usr/bin/env bash
# Generador de paquete .deb para Debian/Ubuntu
set -e

VERSION="0.1.0"
ARCH="amd64"
PKG_DIR="rustcare_${VERSION}_${ARCH}"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="${SCRIPT_DIR}/../.."
OUTPUT_DIR="${ROOT_DIR}/dist"

echo "Creando estructura de paquete Debian..."
mkdir -p "${PKG_DIR}/DEBIAN"
mkdir -p "${PKG_DIR}/usr/bin"
mkdir -p "${PKG_DIR}/usr/share/rustcare/rules"
mkdir -p "${PKG_DIR}/usr/share/applications"

cat << EOF > "${PKG_DIR}/DEBIAN/control"
Package: rustcare
Version: ${VERSION}
Section: utils
Priority: optional
Architecture: ${ARCH}
Maintainer: RustCare Open Source Community <support@rustcare.org>
Description: High-performance system maintenance and optimization suite built in Rust.
 Multiplatform zero-telemetry and zero-admin PC optimization tool.
EOF

cp "${ROOT_DIR}/target/release/rustcare-gui" "${PKG_DIR}/usr/bin/"
cp -r "${ROOT_DIR}/rules/"* "${PKG_DIR}/usr/share/rustcare/rules/"
cp "${SCRIPT_DIR}/rustcare.desktop" "${PKG_DIR}/usr/share/applications/"

mkdir -p "${OUTPUT_DIR}"
dpkg-deb --build "${PKG_DIR}" "${OUTPUT_DIR}/RustCare-v${VERSION}-linux-amd64.deb"
rm -rf "${PKG_DIR}"

echo "Paquete Debian generado en: ${OUTPUT_DIR}/RustCare-v${VERSION}-linux-amd64.deb"
