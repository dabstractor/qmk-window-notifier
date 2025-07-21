# Design Document

## Overview

The QMKonnect documentation needs to be restructured to accurately reflect the platform-specific user experiences. Based on the actual code implementation, Windows and macOS provide GUI-only experiences through system tray/menu bar interfaces, while Linux uses command line tools and systemd service management. This design addresses how to reorganize the documentation to eliminate confusion and provide clear, platform-appropriate instructions.

## Architecture

### Documentation Structure
```
README.md                    # Main project overview with accurate platform sections
docs/
├── installation.md         # Platform-specific installation methods
├── configuration.md        # Platform-specific configuration approaches  
├── usage.md               # Platform-specific usage instructions
├── qmk-integration.md     # QMK firmware integration (unchanged)
├── troubleshooting.md     # Platform-specific troubleshooting
├── advanced.md           # Advanced features (unchanged)
└── examples.md           # Usage examples (unchanged)
```

### Platform-Specific Content Organization

#### Windows Section Structure
- **Installation**: MSI installer only
- **Configuration**: System tray → Settings dialog only
- **Usage**: System tray operations, automatic startup
- **No command line references**

#### macOS Section Structure  
- **Installation**: Application bundle download and copy to Applications
- **Configuration**: Menu bar → Settings dialog only
- **Usage**: Applications folder launch, menu bar operations
- **No command line references**

#### Linux Section Structure
- **Installation**: makepkg -si, systemd service setup
- **Configuration**: Command line config file creation and editing
- **Usage**: systemctl commands, manual service management
- **Full command line workflow**

## Components and Interfaces

### Windows GUI Documentation
```markdown
## Windows

### Installation
1. Download MSI installer from releases
2. Run as Administrator
3. Application starts automatically

### Configuration
1. Right-click QMKonnect system tray icon
2. Select "Settings"
3. Enter Vendor ID (hex): feed
4. Enter Product ID (hex): 0000
5. Click OK

### Usage
- Starts automatically with Windows
- Right-click system tray icon to access menu
- Select "Quit" to exit
```

### macOS GUI Documentation
```markdown
## macOS

### Installation
1. Download QMKonnect.app from releases
2. Copy to Applications folder
3. Launch from Applications

### Configuration
1. Right-click QMKonnect menu bar icon
2. Select "Settings"
3. Enter Vendor ID (hex): feed
4. Enter Product ID (hex): 0000
5. Click OK

### Usage
- Launch from Applications folder
- Right-click menu bar icon to access menu
- Select "Quit" to exit
```

### Linux Command Line Documentation
```markdown
## Linux

### Arch Linux Installation
```bash
git clone https://github.com/dabstractor/qmkonnect.git
cd qmkonnect/packaging/linux/arch
makepkg -si
systemctl --user enable --now qmkonnect.service
```

### Configuration
```bash
# Create config file
qmkonnect -c

# Edit configuration
nano ~/.config/qmk-notifier/config.toml

# Reload configuration
qmkonnect -r
sudo udevadm control --reload && sudo udevadm trigger
```

### Usage
```bash
# Check service status
systemctl --user status qmkonnect

# View logs
journalctl --user -u qmkonnect -f
```
```

## Data Models

### Platform Configuration Model
```yaml
platforms:
  windows:
    installation_method: "msi_installer"
    configuration_method: "gui_dialog"
    user_interaction: "system_tray"
    command_line_required: false
    
  macos:
    installation_method: "app_bundle"
    configuration_method: "gui_dialog"
    user_interaction: "menu_bar"
    command_line_required: false
    
  linux:
    installation_method: "package_manager"
    configuration_method: "config_file"
    user_interaction: "command_line"
    command_line_required: true
```

### Configuration Data Model
```yaml
configuration:
  windows:
    method: "Settings Dialog"
    fields:
      - name: "Vendor ID"
        format: "hex"
        example: "feed"
      - name: "Product ID"
        format: "hex"
        example: "0000"
    
  macos:
    method: "Settings Dialog"
    fields:
      - name: "Vendor ID"
        format: "hex"
        example: "feed"
      - name: "Product ID"
        format: "hex"
        example: "0000"
    
  linux:
    method: "TOML Config File"
    location: "~/.config/qmk-notifier/config.toml"
    format: |
      vendor_id = 0xfeed
      product_id = 0x0000
```

## Error Handling

### Documentation Clarity Issues
- **Mixed Platform Instructions**: Clearly separate platform sections
- **Incorrect Command Line References**: Remove from Windows/macOS sections
- **Outdated Examples**: Use actual hex values from code (0xfeed, 0x0000)

### User Confusion Prevention
- **Platform Identification**: Clear headers and section separation
- **Method Emphasis**: Bold the recommended approach for each platform
- **Consistent Terminology**: Use "Settings dialog" not "configuration file" for GUI platforms

## Testing Strategy

### Content Validation
1. **Platform Separation**: Verify no command line instructions in Windows/macOS sections
2. **Accuracy Check**: Confirm all instructions match actual code behavior
3. **Example Verification**: Ensure all examples use real values from codebase
4. **Link Validation**: Check all internal and external links work correctly

### User Experience Testing
1. **Windows User Path**: MSI installer → automatic startup → system tray settings
2. **macOS User Path**: App download → Applications copy → menu bar settings  
3. **Linux User Path**: makepkg → systemctl enable → config file editing

## Implementation Notes

### Critical Changes Needed

#### README.md Updates
- Remove command line instructions from Windows/macOS sections
- Update macOS section to show app bundle installation only
- Ensure Linux section shows proper systemctl commands
- Remove incorrect configuration examples

#### docs/installation.md Updates
- Remove "Manual Installation" section for Windows
- Remove "Build from Source" instructions for macOS in main flow
- Emphasize MSI installer for Windows, app bundle for macOS
- Keep detailed Linux command line instructions

#### docs/configuration.md Updates
- Replace Windows/macOS command line instructions with GUI dialog steps
- Show actual Settings dialog fields (Vendor ID, Product ID)
- Keep Linux command line configuration with proper file paths
- Remove incorrect system tool examples

#### docs/usage.md Updates
- Remove command line options from Windows/macOS sections
- Focus on GUI operations (system tray, menu bar)
- Keep Linux systemctl and command line usage
- Remove made-up debugging examples

### Accurate Information Requirements
- Use real hex values: 0xfeed, 0x0000 (from code)
- Show actual file paths: %APPDATA%\qmk-notifier\config.toml (Windows), ~/.config/qmk-notifier/config.toml (Linux/macOS)
- Reference actual GUI elements: "Settings" menu item, Vendor ID/Product ID fields
- Use correct systemd commands: `systemctl --user enable --now qmkonnect.service`

### Content Organization
- Lead with recommended method for each platform
- Use clear platform headers (## Windows, ## macOS, ## Linux)
- Separate GUI instructions from command line instructions
- Provide platform-specific troubleshooting sections