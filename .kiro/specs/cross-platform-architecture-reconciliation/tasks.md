# Implementation Plan

- [x] 1. Extract platform-specific execution logic into separate runner modules
  - Create dedicated runner modules for each platform to isolate execution logic
  - Move Windows service/tray logic to windows_runner.rs
  - Move macOS threading/observer logic to macos_runner.rs
  - Create simplified linux_runner.rs with direct console execution
  - _Requirements: 1.1, 1.2, 1.3, 1.4, 6.1, 6.2_

- [x] 1.1 Create Windows runner module
  - Extract all Windows-specific execution logic from main.rs into src/runners/windows.rs
  - Include service mode, tray mode, console mode, and singleton management
  - Preserve all existing Windows functionality without changes
  - _Requirements: 1.2, 4.1, 4.2, 10.1_

- [x] 1.2 Create macOS runner module  
  - Extract all macOS-specific execution logic from main.rs into src/runners/macos.rs
  - Include permission handling, threading, and Core Foundation event loops
  - Preserve all existing macOS functionality without changes
  - _Requirements: 1.4, 3.6, 10.2_

- [x] 1.3 Create Linux runner module
  - Create new src/runners/linux.rs with simplified console execution model
  - Implement direct monitor creation and startup without Windows complexity
  - Add simple signal handling for Ctrl+C exit
  - Remove all Windows service/tray logic from Linux execution path
  - _Requirements: 1.3, 5.1, 5.3, 5.4, 10.3_

- [x] 2. Implement unified configuration management with platform-specific paths
  - Create consistent configuration interface across all platforms
  - Implement macOS configuration paths (currently placeholder)
  - Ensure Linux configuration paths work correctly with XDG compliance
  - Test configuration loading and creation on all platforms
  - _Requirements: 2.1, 2.2, 2.3, 2.4, 2.5, 2.6, 2.7_

- [x] 2.1 Implement macOS configuration paths
  - Add get_config_paths() implementation for macOS in platforms/macos.rs
  - Use ~/Library/Application Support/QMKonnect as primary location
  - Add create_config_dir() implementation for macOS
  - Test configuration directory creation and file access
  - _Requirements: 2.3, 2.4_

- [x] 2.2 Verify Linux configuration management
  - Test existing Linux configuration paths with XDG_CONFIG_HOME
  - Verify ~/.config/qmk-notifier fallback works correctly
  - Test configuration file creation and loading on Linux
  - Ensure udev rules integration still functions
  - _Requirements: 2.2, 2.4, 5.6_

- [x] 2.3 Create unified configuration interface
  - Define consistent ConfigManager trait for all platforms
  - Implement platform-specific configuration path resolution
  - Add default configuration creation for each platform
  - Test TOML format consistency across platforms
  - _Requirements: 2.4, 2.5, 2.6_

- [x] 3. Refactor main.rs to use platform-specific routing
  - Implement platform detection and routing logic in main.rs
  - Route to appropriate platform runner based on target_os
  - Remove Windows-specific complexity from non-Windows execution paths
  - Preserve all existing command-line argument handling per platform
  - _Requirements: 1.1, 1.5, 1.6, 6.1, 6.2_

- [x] 3.1 Implement platform routing logic
  - Create platform detection function in main.rs
  - Add conditional compilation for platform-specific runners
  - Route execution to windows_runner, macos_runner, or linux_runner
  - Ensure platform-specific arguments only processed on correct platform
  - _Requirements: 1.1, 1.5, 1.6_

- [x] 3.2 Clean up main.rs execution paths
  - Remove inline Windows service/tray logic from main.rs
  - Remove inline macOS threading logic from main.rs  
  - Remove Windows complexity from Linux execution path
  - Simplify main.rs to focus on routing and shared functionality
  - _Requirements: 6.1, 6.2, 5.2_

- [x] 3.3 Update command-line argument handling
  - Ensure Windows-specific arguments only shown on Windows
  - Ensure macOS-specific arguments only shown on macOS
  - Ensure Linux-specific arguments only shown on Linux
  - Test help output on each platform shows appropriate options
  - _Requirements: 9.1, 9.2, 9.3, 9.4_

- [x] 4. Fix Linux window monitoring and execution model
  - Restore simple Linux execution model without Windows complexity
  - Fix Linux window monitoring to work independently
  - Remove Windows threading conflicts from Linux implementation
  - Test Linux functionality works identically to pre-Windows state
  - _Requirements: 5.1, 5.3, 5.4, 5.6, 10.3_

- [x] 4.1 Restore Linux execution simplicity
  - Implement direct console execution in linux_runner.rs
  - Remove Windows service/tray complexity from Linux path
  - Add immediate window monitoring startup for Linux
  - Implement simple Ctrl+C signal handling for immediate exit
  - _Requirements: 5.1, 5.3, 5.4_

- [x] 4.2 Fix Linux window monitoring
  - Ensure Linux WindowMonitor implementation works independently
  - Remove Windows threading conflicts from Linux monitor
  - Test X11 and Wayland window detection functionality
  - Verify QMK notifications work correctly on Linux
  - _Requirements: 3.1, 3.2, 3.4, 5.6_

- [x] 4.3 Test Linux threading model
  - Verify Linux uses appropriate threading without Windows conflicts
  - Test monitor thread creation and management on Linux
  - Ensure clean shutdown and resource cleanup on Linux
  - Test signal handling works correctly for Linux
  - _Requirements: 7.2, 7.4, 5.4_

- [x] 5. Update conditional compilation and feature gates
  - Ensure platform-specific code properly isolated with cfg attributes
  - Update Cargo.toml dependencies for clean platform separation
  - Test compilation on each platform includes only relevant dependencies
  - Verify feature flags work correctly for platform-specific functionality
  - _Requirements: 6.1, 6.2, 6.3, 11.1, 11.2, 11.3, 11.4, 11.5_

- [x] 5.1 Clean up conditional compilation attributes
  - Add proper #[cfg(target_os = "...")] attributes to all platform-specific code
  - Ensure Windows-specific code only compiles on Windows
  - Ensure macOS-specific code only compiles on macOS
  - Ensure Linux-specific code only compiles on Linux
  - _Requirements: 6.1, 6.2_

- [x] 5.2 Update Cargo.toml dependencies
  - Verify Windows dependencies only included for Windows targets
  - Verify macOS dependencies only included for macOS targets
  - Verify Linux dependencies only included for Linux targets
  - Test shared dependencies work across all platforms
  - _Requirements: 11.1, 11.2, 11.3, 11.4_

- [x] 5.3 Test feature flag functionality
  - Test hyprland feature flag works correctly on Linux
  - Test macos feature flag works correctly on macOS
  - Verify feature flags don't affect other platforms
  - Test default feature configuration works on all platforms
  - _Requirements: 11.5, 6.4_

- [x] 6. Implement unified error handling and logging
  - Ensure consistent error handling across platforms using appropriate methods
  - Implement platform-specific logging (Windows Event Log, console, etc.)
  - Test verbose mode works correctly on each platform
  - Verify error messages are helpful and platform-appropriate
  - _Requirements: 8.1, 8.2, 8.3, 8.4, 8.5_

- [x] 6.1 Implement platform-specific logging
  - Preserve Windows Event Log functionality for Windows
  - Ensure macOS uses console logging appropriately
  - Restore Linux console logging functionality
  - Test verbose mode provides detailed output on each platform
  - _Requirements: 8.1, 8.2, 8.3_

- [x] 6.2 Standardize error handling patterns
  - Ensure consistent error handling across all platform runners
  - Implement graceful degradation for each platform
  - Test fatal error handling exits cleanly on all platforms
  - Verify error logging doesn't cause cross-platform issues
  - _Requirements: 8.4, 8.5_

- [ ] 7. Create comprehensive test suite for platform isolation
  - Write tests for each platform runner independently
  - Create integration tests for shared functionality
  - Add regression tests for Windows and macOS functionality
  - Add restoration tests for Linux functionality
  - _Requirements: 10.1, 10.2, 10.3, 10.4, 10.5, 10.6_

- [ ] 7.1 Write platform runner tests
  - Create unit tests for windows_runner.rs functionality
  - Create unit tests for macos_runner.rs functionality
  - Create unit tests for linux_runner.rs functionality
  - Test each runner works independently without cross-platform interference
  - _Requirements: 6.3, 10.1, 10.2, 10.3_

- [ ] 7.2 Create integration tests
  - Test configuration management works on all platforms
  - Test window monitoring consistency across platforms
  - Test QMK notification formatting is consistent
  - Test command-line argument handling per platform
  - _Requirements: 2.4, 3.3, 9.5_

- [ ] 7.3 Add regression prevention tests
  - Test all existing Windows functionality (service, tray, settings)
  - Test all existing macOS functionality (permissions, observers, tray)
  - Test restored Linux functionality matches pre-Windows behavior
  - Create automated tests to prevent future architectural conflicts
  - _Requirements: 10.1, 10.2, 10.6_

- [ ] 8. Final integration and validation testing
  - Test complete application functionality on each platform independently
  - Verify no regressions in Windows service, tray, and settings functionality
  - Verify no regressions in macOS permission, observer, and tray functionality
  - Verify Linux restoration provides working console application
  - _Requirements: 10.1, 10.2, 10.3, 10.4, 10.5, 10.6_

- [ ] 8.1 Windows functionality validation
  - Test Windows service installation, start, stop, uninstall
  - Test Windows tray application with settings dialog
  - Test Windows console mode for debugging
  - Test Windows singleton instance management
  - Verify all Windows features work identically to current implementation
  - _Requirements: 10.1, 10.4_

- [ ] 8.2 macOS functionality validation
  - Test macOS permission handling and screen recording access
  - Test macOS Core Foundation event loops and observers
  - Test macOS tray integration and icon loading
  - Test macOS threading model and monitor execution
  - Verify all macOS features work identically to current implementation
  - _Requirements: 10.2, 10.5_

- [ ] 8.3 Linux functionality validation
  - Test Linux console application starts and runs correctly
  - Test Linux window monitoring detects focus changes
  - Test Linux QMK notifications work correctly
  - Test Linux configuration management and udev rules
  - Test Linux signal handling and clean exit
  - Verify Linux works identically to pre-Windows-feature state
  - _Requirements: 10.3, 10.6, 5.6_

- [ ] 8.4 Cross-platform consistency validation
  - Test configuration file format consistency across platforms
  - Test QMK notification message format consistency
  - Test window information extraction consistency
  - Test error handling and logging appropriateness per platform
  - _Requirements: 2.5, 3.3, 8.1, 8.2, 8.3_