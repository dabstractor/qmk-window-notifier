#!/bin/bash
set -e

# Install app with human-readable name
sudo rm -rf "/Applications/QMK Window Notifier.app"
sudo cp -r "packaging/macos/QMK Window Notifier.app" "/Applications/"

# Install LaunchAgent (uses bundle ID)
cat << EOF > ~/Library/LaunchAgents/io.mulletware.qmk-window-notifier.plist
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>io.mulletware.qmk-window-notifier</string>
    <key>ProgramArguments</key>
    <array>
        <string>/Applications/QMK Window Notifier.app/Contents/MacOS/qmk-window-notifier</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <true/>
    <key>StandardOutPath</key>
    <string>/tmp/qmk-window-notifier.out.log</string>
    <key>StandardErrorPath</key>
    <string>/tmp/qmk-window-notifier.err.log</string>
</dict>
</plist>
EOF

launchctl load ~/Library/LaunchAgents/io.mulletware.qmk-window-notifier.plist

echo "✅ Installed! First run:"
echo "1. Open Finder → Applications → 'QMK Window Notifier'"
echo "2. Right-click → Open → 'Open' in security dialog"
echo "3. Grant Screen Recording permission"
