#!/bin/bash
set -e

cargo build --release

# Create app bundle with human-readable name
rm -rf "QMK Window Notifier.app"

mkdir -p "QMK Window Notifier.app/Contents/MacOS"
cp ../../target/release/qmk-window-notifier "QMK Window Notifier.app/Contents/MacOS/"

mkdir -p "QMK Window Notifier.app/Contents/Resources"
cp Icon.icns "QMK Window Notifier.app/Contents/Resources/"
cp ../Icon.png "QMK Window Notifier.app/Contents/Resources/"

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

# Create a DMG file containing the app bundle
DMG_NAME="QMK Window Notifier.dmg"
VOLNAME="QMK Window Notifier Installer"
TEMP_DIR=$(mktemp -d)

# Create a symbolic link to /Applications
ln -s "/Applications" "$TEMP_DIR/Applications"

# Copy the app bundle to the temporary directory
cp -R "QMK Window Notifier.app" "$TEMP_DIR/"

# Create the DMG file with a compressed, read-only format (UDZO)
hdiutil create -volname "$VOLNAME" -srcfolder "$TEMP_DIR" -ov -format UDZO "$DMG_NAME"

# Clean up the temporary directory
rm -rf "$TEMP_DIR"

echo "✅ DMG built: $DMG_NAME"
