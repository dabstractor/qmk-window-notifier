#!/bin/bash
set -e

# Install app with human-readable name
sudo rm -rf "/Applications/QMKonnect.app"
sudo cp -r "packaging/macos/QMKonnect.app" "/Applications/"

# Install LaunchAgent (uses bundle ID)
cat << EOF > ~/Library/LaunchAgents/io.mulletware.qmkonnect.plist
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>io.mulletware.qmkonnect</string>
    <key>ProgramArguments</key>
    <array>
        <string>/Applications/QMKonnect.app/Contents/MacOS/qmkonnect</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <true/>
    <key>StandardOutPath</key>
    <string>/tmp/qmkonnect.out.log</string>
    <key>StandardErrorPath</key>
    <string>/tmp/qmkonnect.err.log</string>
</dict>
</plist>
EOF

launchctl load ~/Library/LaunchAgents/io.mulletware.qmkonnect.plist

echo "✅ Installed! First run:"
echo "1. Open Finder → Applications → 'QMKonnect'"
echo "2. Right-click → Open → 'Open' in security dialog"
echo "3. Grant Screen Recording permission"
