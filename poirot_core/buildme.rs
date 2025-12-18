use anyhow::Context;
use std::env;
use std::path::PathBuf;
use std::process::Command;

fn probe_libclang_via_env() -> Option<PathBuf> {
    env::var("LIBCLANG_PATH").ok().map(PathBuf::from)
}

fn probe_via_pkg_config() -> bool {
    pkg_config::Config::new().atleast_version("0").probe("libclang").is_ok()
}

fn probe_via_llvm_config() -> Option<PathBuf> {
    if let Ok(path) = which::which("llvm-config") {
        if let Ok(output) = Command::new(path).arg("--libdir").output() {
            if output.status.success() {
                if let Ok(s) = String::from_utf8(output.stdout) {
                    let p = PathBuf::from(s.trim());
                    if p.exists() {
                        return Some(p);
                    }
                }
            }
        }
    }
    None
}

fn probe_common_paths() -> Option<PathBuf> {
    let candidates = ["/usr/lib", "/usr/lib64", "/usr/local/lib", "/usr/local/opt/llvm/lib"];
    for c in &candidates {
        let p = PathBuf::from(c);
        if p.join("libclang.so").exists() || p.join("libclang.dylib").exists() || p.join("libclang.dll").exists() {
            return Some(p);
        }
    }
    None
}

fn try_run_installer(script: &str) -> anyhow::Result<()> {
    if cfg!(target_family = "unix") {
        let status = Command::new("sh").arg(script).status().context("failed to run shell installer")?;
        if status.success() { return Ok(()); }
        anyhow::bail!("installer script failed");
    } else if cfg!(target_os = "windows") {
        let status = Command::new("powershell")
            .arg("-NoProfile")
            .arg("-ExecutionPolicy")
            .arg("Bypass")
            .arg("-File")
            .arg(script)
            .status()
            .context("failed to run powershell installer")?;
        if status.success() { return Ok(()); }
        anyhow::bail!("installer script failed");
    }
    Ok(())
}

fn main() {
    // 1) honor LIBCLANG_PATH
    if let Some(dir) = probe_libclang_via_env() {
        println!("cargo:rustc-link-search=native={}", dir.display());
        println!("cargo:rustc-link-lib=dylib=clang");
        return;
    }

    // 2) try pkg-config
    if probe_via_pkg_config() {
        // pkg-config will handle link flags
        return;
    }

    // 3) try llvm-config
    if let Some(dir) = probe_via_llvm_config() {
        println!("cargo:rustc-link-search=native={}", dir.display());
        println!("cargo:rustc-link-lib=dylib=clang");
        return;
    }

    // 4) common paths
    if let Some(dir) = probe_common_paths() {
        println!("cargo:rustc-link-search=native={}", dir.display());
        println!("cargo:rustc-link-lib=dylib=clang");
        return;
    }

    // 5) Optionally attempt auto-install if explicitly allowed
    let auto = env::var("AUTO_INSTALL_LIBCLANG").map(|v| v == "1").unwrap_or(false);
    if auto {
        println!("cargo:warning=AUTO_INSTALL_LIBCLANG=1 set; attempting installer scripts (may require privileges)");
        if cfg!(target_family = "unix") {
            let script = "scripts/install-libclang.sh";
            if std::path::Path::new(script).exists() {
                if let Err(e) = try_run_installer(script) {
                    println!("cargo:warning=installer failed: {}", e);
                }
            } else {
                println!("cargo:warning=installer script {} not found", script);
            }
        } else if cfg!(target_os = "windows") {
            let script = "scripts/install-libclang.ps1";
            if std::path::Path::new(script).exists() {
                if let Err(e) = try_run_installer(script) {
                    println!("cargo:warning=installer failed: {}", e);
                }
            } else {
                println!("cargo:warning=installer script {} not found", script);
            }
        } else if cfg!(target_os = "macos") {
            println!("cargo:warning=Attempting `brew install llvm`. Requires Homebrew and user approval.");
            let _ = Command::new("brew").arg("install").arg("llvm").status();
        }

        // After attempting install, re-run probes
        if probe_via_pkg_config() || probe_via_llvm_config().is_some() || probe_common_paths().is_some() {
            return;
        }
    }

    // 6) fail with helpful message
    println!("\n*** libclang not found ***\n");
    println!("Please install LLVM/libclang or point to it with the LIBCLANG_PATH environment variable.");
    println!("Suggestions:");
    println!(" - Linux (Debian/Ubuntu): sudo apt-get install clang libclang-dev llvm-dev pkg-config build-essential");
    println!(" - Fedora: sudo dnf install clang llvm-devel pkgconf-pkg-config");
    println!(" - Arch: sudo pacman -S clang llvm pkgconf");
    println!(" - macOS: brew install llvm");
    println!(" - Windows: install LLVM (https://llvm.org/) or use vcpkg/choco/winget and set LIBCLANG_PATH if necessary");
    println!("To attempt an automatic install during build set AUTO_INSTALL_LIBCLANG=1 and rebuild, but note this may require admin privileges.");

    panic!("libclang not found; see messages above for install instructions");
}
