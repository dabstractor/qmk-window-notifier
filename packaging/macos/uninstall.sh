#!/bin/bash
# Full path uninstaller - handles app name with spaces

# Stop and remove LaunchAgent
launchctl remove io.mulletware.qmk-window-notifier
rm -f ~/Library/LaunchAgents/io.mulletware.qmk-window-notifier.plist

# Remove app bundle
sudo rm -rf "/Applications/QMK Window Notifier.app"

# Remove logs
rm -f /tmp/qmk-window-notifier.{out,err}.log

echo "✅ QMK Window Notifier completely uninstalled"
