#!/usr/bin/env pwsh
param(
    [switch]$SkipBuild,
    [switch]$Help
)

if ($Help) {
    Write-Host "QMK Window Notifier Installer Build Script"
    Write-Host ""
    Write-Host "Usage: .\build-installer.ps1 [OPTIONS]"
    Write-Host ""
    Write-Host "Options:"
    Write-Host "  -SkipBuild    Skip building the Rust application"
    Write-Host "  -Help         Show this help message"
    Write-Host ""
    exit 0
}

# Colors for output
$ErrorColor = "Red"
$SuccessColor = "Green"
$InfoColor = "Cyan"
$WarningColor = "Yellow"

function Write-Info {
    param([string]$Message)
    Write-Host $Message -ForegroundColor $InfoColor
}

function Write-Success {
    param([string]$Message)
    Write-Host $Message -ForegroundColor $SuccessColor
}

function Write-Error {
    param([string]$Message)
    Write-Host $Message -ForegroundColor $ErrorColor
}

function Write-Warning {
    param([string]$Message)
    Write-Host $Message -ForegroundColor $WarningColor
}

# Check if WiX is installed
function Test-WixInstalled {
    try {
        $null = Get-Command "candle.exe" -ErrorAction Stop
        $null = Get-Command "light.exe" -ErrorAction Stop
        return $true
    }
    catch {
        return $false
    }
}

# Main script
Write-Info "QMK Window Notifier Installer Build Script"
Write-Info "=========================================="

# Check WiX installation
if (-not (Test-WixInstalled)) {
    Write-Error "WiX Toolset not found!"
    Write-Error "Please install WiX Toolset v3.x from https://wixtoolset.org/releases/"
    Write-Error "Or install via package manager:"
    Write-Error "  Chocolatey: choco install wixtoolset"
    Write-Error "  Winget: winget install WiXToolset.WiX"
    exit 1
}

Write-Success "WiX Toolset found"

# Build Rust application if not skipped
if (-not $SkipBuild) {
    Write-Info "Building Rust application..."
    
    # Change to project root
    Push-Location "../.."
    
    try {
        # Clean previous build
        Write-Info "Cleaning previous build..."
        cargo clean
        
        # Build in release mode
        Write-Info "Building in release mode..."
        cargo build --release
        
        if ($LASTEXITCODE -ne 0) {
            throw "Cargo build failed"
        }
        
        # Verify executable exists
        $exePath = "target/release/qmk-window-notifier.exe"
        if (-not (Test-Path $exePath)) {
            throw "Executable not found at $exePath"
        }
        
        Write-Success "Rust application built successfully"
    }
    catch {
        Write-Error "Failed to build Rust application: $_"
        Pop-Location
        exit 1
    }
    finally {
        Pop-Location
    }
}
else {
    Write-Info "Skipping Rust build (as requested)"
    
    # Verify executable exists
    $exePath = "../../target/release/qmk-window-notifier.exe"
    if (-not (Test-Path $exePath)) {
        Write-Error "Executable not found at $exePath"
        Write-Error "Please build the application first or remove -SkipBuild flag"
        exit 1
    }
}

# Build installer
Write-Info "Building Windows installer..."

try {
    # Compile WiX source
    Write-Info "Compiling WiX source..."
    & candle.exe installer.wxs
    
    if ($LASTEXITCODE -ne 0) {
        throw "WiX compilation failed"
    }
    
    # Create MSI
    Write-Info "Creating MSI package..."
    & light.exe installer.wixobj -ext WixUIExtension -ext WixUtilExtension -out "QMK-Window-Notifier-Setup.msi"
    
    if ($LASTEXITCODE -ne 0) {
        throw "MSI creation failed"
    }
    
    # Clean up intermediate files
    Remove-Item "installer.wixobj" -ErrorAction SilentlyContinue
    Remove-Item "*.wixpdb" -ErrorAction SilentlyContinue
    
    Write-Success "Installer created successfully: QMK-Window-Notifier-Setup.msi"
    
    # Show file info
    $msiFile = Get-Item "QMK-Window-Notifier-Setup.msi"
    Write-Info "File size: $([math]::Round($msiFile.Length / 1MB, 2)) MB"
    Write-Info "Created: $($msiFile.CreationTime)"
    
}
catch {
    Write-Error "Failed to build installer: $_"
    
    # Clean up on failure
    Remove-Item "installer.wixobj" -ErrorAction SilentlyContinue
    Remove-Item "*.wixpdb" -ErrorAction SilentlyContinue
    
    exit 1
}

Write-Success "Build completed successfully!"
Write-Info ""
Write-Info "Installation Instructions:"
Write-Info "1. Run QMK-Window-Notifier-Setup.msi as Administrator"
Write-Info "2. The service will be installed and started automatically"
Write-Info "3. System tray icon should appear after installation"
Write-Info ""
Write-Info "Service Management:"
Write-Info "- View service: services.msc (look for 'QMK Window Notifier')"
Write-Info "- Manual control: sc start/stop QMKWindowNotifier"
Write-Info "- Logs: Windows Event Viewer > Applications and Services Logs"