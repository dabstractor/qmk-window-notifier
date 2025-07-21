# Cross-Platform Architecture Reconciliation Design

## Overview

This design addresses the architectural conflicts between Windows, Linux, and macOS implementations in QMKonnect. The primary goal is to fix the broken Linux implementation while preserving the fully functional Windows and macOS implementations. The design establishes clear architectural boundaries and execution models for each platform.

## Architecture

### Current State Analysis

**Windows (Working):**
- Service-based architecture with Windows Service integration
- System tray with settings dialog
- Multiple execution modes (service, tray, console)
- Singleton instance management
- Windows Event Log integration
- AppData configuration storage

**macOS (Working):**
- Core Foundation event loop architecture
- NSWorkspace notification observers
- Screen recording permission handling
- Application bundle integration
- Threaded execution with tray integration
- Library/Application Support configuration storage

**Linux (Broken):**
- Originally simple console application
- Broken by Windows architectural complexity
- Conflicting execution paths and threading models
- XDG configuration management still functional

### Target Architecture

The design implements a **Platform-Specific Execution Strategy** pattern where each platform has its own execution model while sharing common core functionality.

```
┌─────────────────────────────────────────────────────────────┐
│                        main.rs                              │
│  ┌─────────────────────────────────────────────────────────┐ │
│  │            Platform Detection & Routing                 │ │
│  └─────────────────────────────────────────────────────────┘ │
│           │                    │                    │        │
│           ▼                    ▼                    ▼        │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐ │
│  │ Windows Runner  │  │  macOS Runner   │  │  Linux Runner   │ │
│  │   (Complex)     │  │   (Threaded)    │  │   (Simple)      │ │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘ │
└─────────────────────────────────────────────────────────────┘
           │                    │                    │
           ▼                    ▼                    ▼
┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐
│ Shared Core     │  │ Shared Core     │  │ Shared Core     │
│ - WindowMonitor │  │ - WindowMonitor │  │ - WindowMonitor │
│ - Notifier      │  │ - Notifier      │  │ - Notifier      │
│ - Config        │  │ - Config        │  │ - Config        │
└─────────────────┘  └─────────────────┘  └─────────────────┘
```

## Components and Interfaces

### 1. Platform Execution Runners

Each platform has its own execution runner that handles platform-specific startup, lifecycle, and shutdown logic.

#### Windows Runner (Preserve Existing)
```rust
// Existing complex logic preserved
fn run_windows() -> Result<(), Box<dyn Error>> {
    // Service mode detection
    // Tray mode execution  
    // Console mode for debugging
    // Singleton instance management
    // All existing Windows functionality preserved
}
```

#### macOS Runner (Preserve Existing)
```rust  
// Existing threaded logic preserved
fn run_macos() -> Result<(), Box<dyn Error>> {
    // Permission handling
    // Threaded monitor execution
    // Tray integration
    // Core Foundation event loops
    // All existing macOS functionality preserved
}
```

#### Linux Runner (New Simplified)
```rust
// Restored simple console model
fn run_linux() -> Result<(), Box<dyn Error>> {
    // Direct monitor creation and start
    // Simple signal handling
    // No tray, no service complexity
    // Original Linux behavior restored
}
```

### 2. Unified Configuration Management

Platform-specific configuration paths with consistent TOML format:

```rust
pub trait ConfigManager {
    fn get_config_paths() -> Vec<PathBuf>;
    fn create_config_dir() -> Result<PathBuf, Box<dyn Error>>;
    fn get_default_config_location() -> PathBuf;
}

// Windows: AppData\Roaming\QMKonnect\config.toml
// macOS: ~/Library/Application Support/QMKonnect/config.toml  
// Linux: ~/.config/qmk-notifier/config.toml
```

### 3. Window Monitor Abstraction

Unified WindowMonitor trait with platform-specific implementations:

```rust
pub trait WindowMonitor {
    fn platform_name(&self) -> &str;
    fn start(&mut self) -> Result<(), Box<dyn Error>>;
    fn stop(&mut self) -> Result<(), Box<dyn Error>>;
}

// Implementations:
// - WindowsMonitor (hooks + polling)
// - MacOSMonitor (Core Foundation observers)  
// - LinuxMonitor (X11/Wayland detection)
```

### 4. Platform Feature Gates

Clear conditional compilation boundaries:

```rust
// Platform-specific modules
#[cfg(target_os = "windows")]
mod windows_runner;

#[cfg(target_os = "macos")] 
mod macos_runner;

#[cfg(target_os = "linux")]
mod linux_runner;

// Platform-specific dependencies in Cargo.toml
[target.'cfg(target_os = "windows")'.dependencies]
windows-service = "0.7.0"
single-instance = "0.3"

[target.'cfg(target_os = "macos")'.dependencies]
core-foundation = "0.9"
objc = "0.2.7"

[target.'cfg(target_os = "linux")'.dependencies]
# Minimal Linux-specific deps only
```

## Data Models

### Configuration Structure
```toml
# Consistent across all platforms
vendor_id = 0xfeed
product_id = 0x0000

# Platform-specific sections can be added as needed
[windows]
# Windows-specific settings

[macos]  
# macOS-specific settings

[linux]
# Linux-specific settings
```

### Window Information
```rust
// Shared across all platforms
pub struct WindowInfo {
    pub app_class: String,
    pub title: String,
}
```

## Error Handling

### Platform-Specific Error Logging

**Windows:** Windows Event Log + console fallback (existing)
**macOS:** Console logging (existing)  
**Linux:** Console logging (restored)

```rust
fn log_error(error: &str) {
    #[cfg(target_os = "windows")]
    {
        // Use existing Windows Event Log
        log::error!("{}", error);
    }
    
    #[cfg(any(target_os = "macos", target_os = "linux"))]
    {
        eprintln!("Error: {}", error);
    }
}
```

### Graceful Degradation
- Windows: Service failures fall back to tray mode
- macOS: Permission failures provide clear user guidance
- Linux: Monitor failures exit cleanly with error message

## Testing Strategy

### Platform Isolation Testing
- Each platform runner tested independently
- Mock implementations for cross-platform shared code
- Platform-specific integration tests

### Regression Prevention
- Windows functionality tests (service, tray, settings)
- macOS functionality tests (permissions, observers, tray)
- Linux functionality tests (simple console execution)

### Cross-Platform Consistency Tests
- Configuration format compatibility
- Window information formatting
- QMK notification consistency

## Implementation Approach

### Phase 1: Platform Runner Extraction
1. Extract existing Windows logic into `windows_runner.rs`
2. Extract existing macOS logic into `macos_runner.rs`  
3. Create new simplified `linux_runner.rs`
4. Update main.rs to route to appropriate runner

### Phase 2: Configuration Unification
1. Implement macOS configuration paths (currently placeholder)
2. Ensure Linux configuration paths work correctly
3. Test configuration consistency across platforms

### Phase 3: Linux Restoration
1. Implement simple Linux execution model
2. Remove Windows complexity from Linux path
3. Restore original Linux threading model
4. Test Linux functionality independently

### Phase 4: Integration Testing
1. Test all platforms independently
2. Verify no regressions in Windows/macOS
3. Verify Linux restoration
4. Cross-platform configuration testing

## Migration Strategy

### Backwards Compatibility
- All existing Windows functionality preserved
- All existing macOS functionality preserved
- Linux restored to pre-Windows-feature state
- Configuration files remain compatible

### Deployment Considerations
- Windows: No changes to service/installer behavior
- macOS: No changes to bundle/permission behavior
- Linux: Restored simple execution model

## Security Considerations

### Platform-Specific Permissions
- Windows: Existing service account permissions preserved
- macOS: Existing screen recording permissions preserved
- Linux: Minimal permissions required (restored)

### Configuration Security
- Platform-appropriate file permissions
- No sensitive data in configuration files
- Secure default configurations

## Performance Implications

### Resource Usage
- Windows: No change (existing optimizations preserved)
- macOS: No change (existing optimizations preserved)  
- Linux: Reduced complexity = improved performance

### Startup Time
- Windows: No change
- macOS: No change
- Linux: Faster startup (reduced complexity)

### Memory Footprint
- Conditional compilation reduces binary size per platform
- Platform-specific dependencies only loaded when needed
- Shared core functionality optimized for all platforms

## Monitoring and Observability

### Platform-Specific Logging
- Windows: Windows Event Log (existing)
- macOS: Console/system log (existing)
- Linux: Console output (restored)

### Debug Modes
- Windows: Console mode for debugging (existing)
- macOS: Verbose logging (existing)
- Linux: Verbose flag support (restored)

## Future Extensibility

### Adding New Platforms
- Clear pattern for new platform runners
- Shared core functionality ready for reuse
- Configuration system extensible

### Platform-Specific Features
- Clear boundaries for platform-specific enhancements
- No cross-platform contamination
- Independent development and testing