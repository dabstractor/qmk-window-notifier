# QMK Window Notifier - Technical Requirements

This document outlines the technical requirements and specifications for the QMK Window Notifier application.

## Core Requirements

### Cross-Platform Support

- **Windows**: Full support with system tray integration
- **Linux**: Support for X11 and Wayland (Hyprland)
- **macOS**: Native window focus detection

### Window Monitoring

- Real-time detection of window focus changes
- Capture of application class and window title information
- Filtering of internal system windows
- Debouncing of rapid window changes

### QMK Integration

- Format window information as `{application_class}{GS}{window_title}`
- Send formatted data to QMK keyboards via Raw HID
- Support for configurable vendor and product IDs

### Configuration Management

- User-configurable settings via TOML configuration file
- Support for keyboard-specific settings (vendor ID, product ID)
- Configuration reload without application restart

## Windows-Specific Requirements

### Background Operation

- **Silent Operation**: No visible console windows
- **Windows Subsystem**: Use `windows_subsystem = "windows"` attribute
- **Error Logging**: Log errors to Windows Event Log instead of console

### Singleton Pattern

- **Single Instance**: Only one instance can run at a time
- **Detection Method**: Use named mutex via `single-instance` crate
- **Graceful Exit**: Second instance detects first and exits cleanly

### System Tray Integration

- **Tray Icon**: Visible icon in system tray
- **Context Menu**: Right-click menu with "Quit" option
- **Icon Loading**: Proper icon loading from installation directory

### Automatic Startup

- **Startup Method**: Windows startup folder entry
- **Silent Startup**: Start automatically with Windows without user intervention
- **Default Behavior**: Run as tray application by default

### Installer

- **Installer Type**: Windows MSI installer using WiX Toolset
- **Installation Location**: Program Files directory
- **Startup Entry**: Create startup shortcut during installation
- **Upgrade Handling**: Detect and remove previous versions
- **Running Application**: Handle gracefully when upgrading while app is running

### Threading Model

- **Main Thread**: Window event hooks must run on main thread
- **Message Loop**: Maintain proper Windows message loop for event handling
- **Thread Safety**: Ensure thread-safe access to shared resources

## Linux-Specific Requirements

### Service Integration

- **Systemd Service**: Run as user service
- **Udev Rules**: Start/stop based on keyboard connection
- **X11/Wayland**: Support both display server protocols

### Configuration

- **XDG Compliance**: Follow XDG Base Directory Specification
- **Udev Rules**: Generate udev rules based on configuration

## macOS-Specific Requirements

### Application Bundle

- **App Bundle**: Package as proper macOS .app bundle
- **Resources**: Include necessary resources in bundle
- **Accessibility**: Request accessibility permissions for window monitoring

## Performance Requirements

- **Resource Usage**: Minimal CPU and memory footprint
- **Startup Time**: Fast startup and initialization
- **Responsiveness**: Immediate detection of window changes

## Error Handling

- **Graceful Degradation**: Handle errors without crashing
- **Logging**: Appropriate logging based on platform
- **User Feedback**: Provide feedback through appropriate channels (tray icon, logs)

## Security Requirements

- **Permissions**: Request only necessary permissions
- **Data Handling**: No collection or transmission of sensitive data
- **Installation**: Proper elevation for installation when required

## Compatibility

- **Windows**: Support Windows 10 and Windows 11
- **Linux**: Support major distributions with X11 or Wayland
- **macOS**: Support recent macOS versions

## Future Enhancements

- **Settings UI**: Graphical interface for configuration
- **Profile Support**: Application-specific profiles
- **Advanced Filtering**: More sophisticated window filtering options
- **Keyboard Selection**: Support for multiple QMK keyboards