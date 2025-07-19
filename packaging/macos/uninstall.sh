#!/bin/bash
# Full path uninstaller - handles app name with spaces

# Stop and remove LaunchAgent
launchctl remove io.mulletware.qmkonnect
rm -f ~/Library/LaunchAgents/io.mulletware.qmkonnect.plist

# Remove app bundle
sudo rm -rf "/Applications/QMKonnect.app"

# Remove logs
rm -f /tmp/qmkonnect.{out,err}.log

echo "✅ QMKonnect completely uninstalled"
