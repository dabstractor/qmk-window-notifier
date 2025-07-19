# Windows Service Implementation for QMK Window Notifier

## Overview
Convert the QMK Window Notifier from a console application to a proper Windows service that runs silently in the background without showing terminal windows.

## Current State
- Application currently runs as a console application
- Shows terminal window when launched
- Has system tray integration
- Window detection and QMK notification working correctly
- MSI installer successfully created

## Requirements

### Core Service Functionality
1. **Windows Service Integration**
   - Convert application to run as a Windows service
   - No console window or terminal popup
   - Service can be started/stopped via Windows Services manager
   - Service starts automatically on system boot
   - Proper service lifecycle management (start, stop, pause, resume)

2. **Silent Operation**
   - No visible windows or console output during normal operation
   - All logging should go to Windows Event Log or log files
   - System tray icon remains the only visible component
   - Service runs under appropriate system account

3. **Service Management**
   - Service installation/uninstallation via installer
   - Service configuration through Windows Services panel
   - Proper service description and display name
   - Error handling and recovery options

### Technical Implementation
1. **Service Architecture**
   - Use Windows Service API or appropriate Rust crate (e.g., `windows-service`)
   - Separate service logic from UI components
   - Service hosts the window monitoring functionality
   - System tray runs as separate component or integrated properly

2. **Logging and Diagnostics**
   - Replace console output with Windows Event Log entries
   - Configurable log levels (Error, Warning, Info, Debug)
   - Log rotation and management
   - Service status reporting

3. **Configuration Management**
   - Service reads configuration from appropriate location
   - Support for service-specific configuration options
   - Configuration changes without service restart where possible

### Installer Integration
1. **Service Installation**
   - MSI installer registers and installs the Windows service
   - Service configured to start automatically
   - Proper permissions and security settings
   - Clean service removal on uninstall

2. **User Experience**
   - Service starts immediately after installation
   - System tray icon appears without user intervention
   - No manual service management required for typical users
   - Advanced users can manage via Services panel

### Compatibility and Testing
1. **Windows Compatibility**
   - Support Windows 10 and Windows 11
   - Proper handling of different user account types
   - UAC compatibility for installation
   - Service isolation and security

2. **Existing Functionality Preservation**
   - All current window detection features maintained
   - QMK notification functionality unchanged
   - System tray functionality preserved
   - Configuration file support maintained

## Success Criteria
- [ ] Application runs as Windows service without console window
- [ ] Service starts automatically on system boot
- [ ] System tray icon appears and functions correctly
- [ ] Window detection and QMK notifications work identically to current implementation
- [ ] MSI installer properly installs and configures the service
- [ ] Service can be managed through Windows Services panel
- [ ] Clean uninstallation removes service completely
- [ ] No visible terminal windows or console popups during operation

## Technical Notes
- Consider using `windows-service` crate for Rust service implementation
- May need to separate service logic from UI thread management
- System tray might need special handling in service context
- Event logging should replace console output for debugging
- Service account permissions need careful consideration for window monitoring APIs

## Out of Scope
- GUI configuration interface (current file-based config sufficient)
- Service clustering or high availability features
- Remote service management
- Service performance monitoring beyond basic Windows service metrics