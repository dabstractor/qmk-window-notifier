#!/bin/bash
set -e

cargo build --release

# Create app bundle with human-readable name
rm -rf "QMK Window Notifier.app"

mkdir -p "QMK Window Notifier.app/Contents/MacOS"
cp ../../target/release/qmk-window-notifier "QMK Window Notifier.app/Contents/MacOS/"

mkdir -p "QMK Window Notifier.app/Contents/Resources"
cp Icon.icns "QMK Window Notifier.app/Contents/Resources/"

# Generate Info.plist
cat << EOF > "QMK Window Notifier.app/Contents/Info.plist"
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>qmk-window-notifier</string>
    <key>CFBundleIdentifier</key>
    <string>io.mulletware.qmk-window-notifier</string>
    <key>CFBundleName</key>
    <string>QMK Window Notifier</string>
    <key>CFBundleDisplayName</key>
    <string>QMK Window Notifier</string>
    <key>LSBackgroundOnly</key>
    <true/>
    <key>CFBundleIconFile</key>
    <string>Icon.icns</string>
</dict>
</plist>
EOF

# Code sign (handles spaces in app name)
codesign --deep --force --sign - "QMK Window Notifier.app"

echo "✅ App built: packaging/macos/QMK Window Notifier.app"
