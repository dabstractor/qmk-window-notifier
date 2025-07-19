# QMK Window Notifier

A cross-platform Rust application that detects active window changes and sends this information to QMK keyboards, enabling dynamic layer switching and command execution based on the active application.

## Overview

QMK Window Notifier bridges the gap between your operating system and QMK-powered keyboards. It monitors window focus changes and transmits application class and title information to your keyboard, enabling context-aware layer switching and feature toggling.

This tool is part of a broader ecosystem:
- **[qmk-notifier](https://github.com/dabstractor/qmk-notifier)**: QMK module that receives commands and handles layer/feature toggling on your keyboard
- **[qmk_notifier](https://github.com/dabstractor/qmk_notifier)**: Desktop application that sends commands to your keyboard via Raw HID
- **qmk-window-notifier** (this tool): Application that detects window changes across platforms

## Features

- **Cross-Platform Support**:
  - Windows: Silent background application with system tray icon
  - Linux: Seamless integration with Hyprland's event system
  - macOS: Native window focus detection

- **Core Functionality**:
  - Real-time window focus change detection
  - Automatic transmission of application class and window title to QMK keyboards
  - Minimal resource footprint
  - Verbose logging option for debugging

- **Windows-Specific Features**:
  - Silent background operation (no console window)
  - System tray icon for easy access and control
  - Automatic startup with Windows
  - Singleton application (prevents multiple instances)
  - Professional MSI installer

- **Configuration**:
  - User-configurable settings
  - Automatic configuration reload

## Installation

### Windows

1. Download the MSI installer from the [releases page](https://github.com/dabstractor/qmk-window-notifier/releases)
2. Run the installer as Administrator
3. The application will start automatically and be added to Windows startup

### Arch Linux

```bash
git clone https://github.com/dabstractor/qmk-window-notifier.git
cd qmk-window-notifier/packaging/linux/arch
makepkg -si
```

### Other Linux Systems
Download the release binary from the [releases page](https://github.com/dabstractor/qmk-window-notifier/releases)

If you want it to start automatically, install the service file and start the service:
```
curl https://raw.githubusercontent.com/dabstractor/qmk-window-notifier/refs/heads/main/packaging/linux/systemd/qmk-window-notifier.service.template | sudo tee /usr/lib/systemd/user/qmk-window-notifier.service
systemctl --user enable --now qmk-window-notifier.service
```
If you want the service turned off when the keyboard isn't plugged in, copy the udev rules template from this project into /etc/udev/rules.d/
```
curl https://raw.githubusercontent.com/dabstractor/qmk-window-notifier/refs/heads/main/packaging/linux/udev/99-qmk-window-notifier.rules.template | sudo tee /etc/udev/rules.d/99-qmk-window-notifier.rules.template
```
Create the config file, write your config to the rules, then reload udev:
```bash
qmk-window-notifier -c
sudo qmk-window-notifier -r
sudo udevadm control --reload && sudo udevadm trigger
```

### MacOS
```bash
git clone https://github.com/dabstractor/qmk-window-notifier.git
cd qmk-window-notifier/packaging/macos
./build.sh
```
Then copy your QMK Window Notifier.app to your /Applications folder

### From Source

```bash
# Clone the repository
git clone https://github.com/dabstractor/qmk-window-notifier.git
cd qmk-window-notifier

# Build the project
cargo build --release

# The binary will be available at target/release/qmk-window-notifier
```

#### Platform-Specific Installation Steps

**Windows:**
```powershell
# Copy to Program Files
Copy-Item target\release\qmk-window-notifier.exe "$env:ProgramFiles\QMK Window Notifier\"

# Add to startup (optional)
$startupFolder = "$env:APPDATA\Microsoft\Windows\Start Menu\Programs\Startup"
$shortcutPath = "$startupFolder\QMK Window Notifier.lnk"
$shell = New-Object -ComObject WScript.Shell
$shortcut = $shell.CreateShortcut($shortcutPath)
$shortcut.TargetPath = "$env:ProgramFiles\QMK Window Notifier\qmk-window-notifier.exe"
$shortcut.Arguments = "--tray-app"
$shortcut.Save()
```

**Linux:**
```bash
# Copy to a location in your PATH
sudo cp target/release/qmk-window-notifier /usr/local/bin/

# Create udev rules file
sudo cp packaging/linux/udev/99-qmk-window-notifier.rules.template /etc/udev/rules.d/99-qmk-window-notifier.rules.template

# Create systemd service file
mkdir -p ~/.config/systemd/user/
cp packaging/linux/systemd/qmk-window-notifier.service.template ~/.config/systemd/user/qmk-window-notifier.service
```

## Configuration

### Creating a Configuration File

Create a configuration file with:

```bash
qmk-window-notifier -c
```

This will create a configuration file at:
- Windows: `%APPDATA%\qmk-notifier\config.toml`
- Linux/macOS: `~/.config/qmk-notifier/config.toml`

### Reloading Configuration

After changing the configuration file, reload it with:

```bash
qmk-window-notifier -r
```

On Linux, also reload udev rules:
```bash
sudo udevadm control --reload && sudo udevadm trigger
```

## Usage

### Windows

The application starts automatically with Windows and runs in the background with a system tray icon.

- **Start manually**: Run "QMK Window Notifier" from Start Menu
- **Exit**: Right-click the system tray icon and select "Quit"
- **Command-line options**:
  - `--tray-app`: Run as tray application (default)
  - `-v, --verbose`: Enable verbose logging
  - `-c, --config`: Create a configuration file
  - `-r, --reload`: Reload configuration

### MacOS

Run the application from within your QMK Window Notifier.app.

### Linux

The application should start automatically when your keyboard is plugged in.
If not, you can start it manually:
```bash
qmk-window-notifier & disown
```

## Technical Requirements

### Windows Service Implementation

- **Background Operation**: Runs silently without console windows
- **Automatic Startup**: Starts with Windows via startup folder
- **System Tray Integration**: Provides user interface through system tray icon
- **Singleton Pattern**: Prevents multiple instances from running simultaneously
- **Window Monitoring**: Properly detects window focus changes
- **Installer**: Professional MSI installer with automatic upgrade handling

### Core Functionality

- **Window Detection**: Monitors active window changes across platforms
- **QMK Integration**: Sends window information to QMK keyboards
- **Configuration Management**: User-configurable settings
- **Error Handling**: Graceful handling of errors and edge cases

## Integration with QMK

This tool works in conjunction with:
- The [qmk-notifier](https://github.com/dabstractor/qmk-notifier) QMK module running on your keyboard
- The [qmk_notifier](https://github.com/dabstractor/qmk_notifier) tool which handles the Raw HID communication

When a window focus change is detected, this application formats the data as:
`{application_class}{GS}{window_title}` where `{GS}` is the Group Separator character (0x1D).

## Default Configuration

```toml
# Your QMK keyboard's vendor ID (in hex)
vendor_id = 0xfeed

# Your QMK keyboard's product ID (in hex)
product_id = 0x0000

# Add any other configuration options here
```

## Example Use Cases

- Automatically switch to a coding layer when your IDE is focused
- Enable media controls when music or video applications are active
- Activate application-specific macros based on the current window
- Create context-aware keyboard layouts that adapt to your workflow

## Contributing

Contributions are welcome! Feel free to submit issues or pull requests on GitHub.

## License

[MIT License](LICENSE)