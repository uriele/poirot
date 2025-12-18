Param()
Write-Host "Attempting to install LLVM/libclang on Windows..."

function Run-Choco($pkg) {
    if (Get-Command choco -ErrorAction SilentlyContinue) {
        choco install $pkg -y
        return $true
    }
    return $false
}

function Run-Winget($id) {
    if (Get-Command winget -ErrorAction SilentlyContinue) {
        winget install --id $id --accept-package-agreements --accept-source-agreements
        return $true
    }
    return $false
}

if (Run-Choco 'llvm') { exit 0 }
if (Run-Winget 'LLVM.LLVM') { exit 0 }

Write-Error "No supported Windows package manager found (choco/winget). Please install LLVM from https://llvm.org/ and ensure libclang.dll is on PATH or set LIBCLANG_PATH."
exit 1
