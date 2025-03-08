#!/bin/bash
set -e

cargo build --release

# Create app bundle with human-readable name
rm -rf "packaging/macos/QMK Window Notifier.app"
mkdir -p "packaging/macos/QMK Window Notifier.app/Contents/MacOS"
cp target/release/qmk-window-notifier "packaging/macos/QMK Window Notifier.app/Contents/MacOS/"

# Generate Info.plist
cat << EOF > "packaging/macos/QMK Window Notifier.app/Contents/Info.plist"
[PASTE THE Info.plist CONTENT ABOVE]
EOF

# Code sign (handles spaces in app name)
codesign --deep --force --sign - "packaging/macos/QMK Window Notifier.app"

echo "✅ App built: packaging/macos/QMK Window Notifier.app"
