# Requirements Document

## Introduction

The QMKonnect documentation currently contains command line instructions for Windows and macOS users, which creates confusion since these platforms provide GUI-only experiences. Based on the actual code implementation, Windows and macOS users interact with QMKonnect exclusively through GUI elements (system tray with settings dialog on Windows, menu bar on macOS), while Linux users use command line tools. The documentation needs to be updated to accurately reflect this reality.

## Requirements

### Requirement 1: Windows GUI-Only Documentation

**User Story:** As a Windows user, I want the documentation to clearly show that I only need to use the MSI installer and system tray GUI, so that I don't get confused by unnecessary command line instructions.

#### Acceptance Criteria

1. WHEN a Windows user reads the installation guide THEN the system SHALL only show MSI installer instructions with no command line alternatives
2. WHEN a Windows user reads the configuration guide THEN the system SHALL only show system tray right-click → Settings dialog instructions
3. WHEN a Windows user reads the usage guide THEN the system SHALL only show GUI-based operations (system tray icon, Start Menu, right-click menu with Quit)
4. WHEN Windows documentation mentions configuration THEN the system SHALL only reference the Settings dialog accessible from system tray

### Requirement 2: macOS GUI-Only Documentation

**User Story:** As a macOS user, I want the documentation to clearly show that I only need to use the application bundle and menu bar GUI, so that I can avoid unnecessary command line complexity.

#### Acceptance Criteria

1. WHEN a macOS user reads the installation guide THEN the system SHALL only show downloading QMKonnect.app and copying to Applications folder
2. WHEN a macOS user reads the configuration guide THEN the system SHALL only show menu bar right-click → Settings dialog instructions
3. WHEN a macOS user reads the usage guide THEN the system SHALL only show GUI-based operations (Applications folder launch, menu bar icon, right-click menu)
4. WHEN macOS documentation mentions configuration THEN the system SHALL only reference the Settings dialog accessible from menu bar

### Requirement 3: Linux Command Line Documentation

**User Story:** As a Linux user, I want clear command line instructions for installation and configuration, so that I can set up QMKonnect using familiar tools and manage the systemd service.

#### Acceptance Criteria

1. WHEN a Linux user reads the installation guide THEN the system SHALL provide makepkg -si command for Arch Linux installation
2. WHEN a Linux user reads the configuration guide THEN the system SHALL show qmkonnect -c to create config file and manual editing of ~/.config/qmk-notifier/config.toml
3. WHEN Linux documentation mentions systemd service THEN the system SHALL include the exact command `systemctl --user enable --now qmkonnect.service`
4. WHEN Linux documentation mentions configuration reload THEN the system SHALL show qmkonnect -r command and udev rules reload

### Requirement 4: Accurate Configuration Information

**User Story:** As a user on any platform, I want accurate information about how configuration actually works, so that I can properly set up my keyboard IDs.

#### Acceptance Criteria

1. WHEN Windows/macOS documentation shows configuration THEN the system SHALL describe the Settings dialog with Vendor ID and Product ID hex input fields
2. WHEN Linux documentation shows configuration THEN the system SHALL show the actual config.toml format with vendor_id = 0xfeed and product_id = 0x0000 examples
3. WHEN documentation mentions configuration file location THEN the system SHALL show correct paths: Windows %APPDATA%\qmk-notifier\config.toml, Linux/macOS ~/.config/qmk-notifier/config.toml
4. WHEN documentation shows configuration examples THEN the system SHALL use real hex values like 0xfeed and 0x0000, not made-up examples

### Requirement 5: README and Documentation Site Consistency

**User Story:** As a user reading documentation, I want consistent and accurate information across README and docs site, so that I understand the correct setup method for my system.

#### Acceptance Criteria

1. WHEN the README shows Windows installation THEN the system SHALL only mention MSI installer with automatic startup
2. WHEN the README shows macOS installation THEN the system SHALL only mention downloading and copying QMKonnect.app to Applications folder
3. WHEN the README shows Linux installation THEN the system SHALL provide makepkg -si for Arch and systemctl --user enable --now qmkonnect.service for service setup
4. WHEN documentation mentions usage THEN the system SHALL accurately reflect that Windows/macOS run automatically with GUI access, while Linux uses systemd service management