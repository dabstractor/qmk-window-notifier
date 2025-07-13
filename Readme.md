# QMK Window Notifier

A specialized Rust application that detects active window changes in supported environments (Hyprland and macOS) and sends this information to QMK keyboards, enabling dynamic layer switching and command execution based on the active application.

## Overview

QMK Window Notifier bridges the gap between supported environments (Hyprland and macOS) and QMK-powered keyboards. It monitors window focus changes and transmits application class and title information to your keyboard, enabling context-aware layer switching and feature toggling.

This tool is part of a broader ecosystem I've created:
- **[qmk-notifier](https://github.com/dabstractor/qmk-notifier)**: QMK module that receives commands and handles layer/feature toggling on your keyboard
- **[qmk_notifier](https://github.com/dabstractor/qmk_notifier)**: Desktop application that sends commands to your keyboard via Raw HID
- **qmk-window-notifier** (this tool): Application that detects window changes in supported environments

## Features

- Seamless integration with Hyprland's event system and macOS
- Real-time window focus change detection
- Automatic transmission of application class and window title to QMK keyboards
- Minimal resource footprint
- Verbose logging option for debugging

## Installation

### Dependencies

- [qmk-notifier](https://github.com/dabstractor/qmk-notifier) must be installed into your QMK keyboard's main directory alongside the `keymap` directory

### Arch Linux

```bash
git clone https://github.com/dabstractor/qmk-window-notifier.git
cd qmk-window-notifier/packaging/linux/arch
makepkg -si
```

### Other linux systems
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
# Copy it to a location in your PATH
sudo cp target/release/qmk-window-notifier /usr/local/bin/


# Create udev rules file (Linux only)
sudo cp packaging/linux/udev/99-qmk-window-notifier.rules.template /etc/udev/rules.d/99-qmk-window-notifier.rules.template

# Create systemd service file (Linux only)
mkdir -p ~/.config/systemd/user/
cp packaging/linux/systemd/qmk-window-notifier.service.template ~/.config/systemd/user/qmk-window-notifier.service

qmk-window-notifier -c
qmk-window-notifier -r
```

## Configuration

### Creating a Configuration File

Create a configuration file with:

```bash
qmk-window-notifier -c
```

This will create a configuration file at `~/.config/qmk-notifier/config.toml` with default values.

### Reloading Configuration

After changing the configuration file, reload it with:

```bash
qmk-window-notifier -r
sudo udevadm control --reload && sudo udevadm trigger
```

This will:
1. Read your configuration file
2. Update the udev rules with your keyboard's vendor and product IDs
3. Reload the udev rules

**Important:** You must reload the configuration whenever you change your keyboard's vendor ID or product ID in the config file.

## Usage

### MacOS

Run the application from within your QMK Window Notifier.app.

### Linux

The application should start automatically when your keyboard is plugged in.
If not, you can start it manually:
```bash
qmk-window-notifier & disown
```


The application automatically:
1. Verifies it's running within a supported environment
2. Sets up event listeners for window focus changes
3. Captures application class and title information
4. Formats and sends this data to your QMK keyboard using the qmk_notifier library

## Integration with QMK

This tool works in conjunction with:
- The [qmk-notifier](https://github.com/dabstractor/qmk-notifier) QMK module running on your keyboard
- The [qmk_notifier](https://github.com/dabstractor/qmk_notifier) tool which handles the Raw HID communication

When a window focus change is detected, this application formats the data as:
`{application_class}{GS}{window_title}` where `{GS}` is the Group Separator character (0x1D).

## Technical Details

- Written in Rust for performance and reliability
- Uses the Hyprland crate to interact with Hyprland's IPC (if running on Wayland)
- Utilizes Hyprland's event system to detect window changes (if running on Wayland)
- Handles error conditions gracefully (e.g., when a supported environment is not running)
- Minimal overhead and resource usage


Your QMK keyboard should be detected when plugged in and the service should start and stop automatically.

## Other Keyboards
If your QMK keyboard uses a different vendor or product ID, or has a different usage_page or usage value, you can configure them in the configuration file at $XDG_CONFIG_HOME/qmk-notifier/config.toml. After changing it, reload the udev rules (linux only):
```
qmk-window-notifier -r
```
The default configuration file looks like:
```
# Your QMK keyboard's vendor ID (in hex)
vendor_id = 0xfeed

# Your QMK keyboard's product ID (in hex)
product_id = 0x0000

# Usage Page
usage_page = 0xFF60

# Usage
usage = 0x61
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

