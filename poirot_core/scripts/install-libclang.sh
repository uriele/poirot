#!/usr/bin/env bash
set -euo pipefail

die() {
  echo "ERROR: $*" >&2
  exit 1
}

SUDO=""
if [ "${EUID:-$(id -u)}" -ne 0 ]; then
  if command -v sudo >/dev/null 2>&1; then
    SUDO=sudo
  else
    die "This script requires root privileges. Please run as root or install sudo."
  fi
fi

echo "Detecting package manager..."

if [ -f /etc/os-release ]; then
  . /etc/os-release
  os_id=${ID:-}
  os_like=${ID_LIKE:-}
else
  os_id=""
  os_like=""
fi

install_debian() {
  $SUDO apt-get update
  $SUDO apt-get install -y clang libclang-dev llvm-dev pkg-config build-essential || die "apt-get install failed"
}

install_fedora() {
  $SUDO dnf install -y clang llvm-devel pkgconf-pkg-config make gcc || die "dnf install failed"
}

install_arch() {
  $SUDO pacman -Syu --noconfirm clang llvm pkgconf base-devel || die "pacman install failed"
}

install_opensuse() {
  $SUDO zypper refresh
  $SUDO zypper install -y clang llvm-devel pkg-config gcc make || die "zypper install failed"
}

install_alpine() {
  $SUDO apk update
  $SUDO apk add clang llvm-dev build-base pkgconfig || die "apk install failed"
}

case "${os_id,,}" in
  ubuntu|debian|linuxmint)
    install_debian
    ;;
  fedora|rhel|centos)
    install_fedora
    ;;
  arch|manjaro)
    install_arch
    ;;
  opensuse*|sles)
    install_opensuse
    ;;
  alpine)
    install_alpine
    ;;
  *)
    # fallback to checking ID_LIKE
    if [[ " ${os_like} " =~ " debian " ]]; then
      install_debian
    elif [[ " ${os_like} " =~ " rhel " ]] || [[ " ${os_like} " =~ " fedora " ]]; then
      install_fedora
    else
      echo "Unsupported or unknown distro: $os_id. Please install clang/libclang manually." >&2
      exit 2
    fi
    ;;
esac

echo "Installation complete."

if command -v llvm-config >/dev/null 2>&1; then
  libdir=$(llvm-config --libdir 2>/dev/null || true)
  if [ -n "$libdir" ]; then
    echo "If cargo still can't find libclang, export LIBCLANG_PATH as follows (example):"
    echo "  export LIBCLANG_PATH=\"$libdir\""
  fi
fi

echo "Try: cargo build"
