#!/bin/bash
set -e

cargo build --release

# Create app bundle with human-readable name
rm -rf "QMKonnect.app"

mkdir -p "QMKonnect.app/Contents/MacOS"
cp ../../target/release/qmkonnect "QMKonnect.app/Contents/MacOS/"

mkdir -p "QMKonnect.app/Contents/Resources"
cp Icon.icns "QMKonnect.app/Contents/Resources/"
cp ../Icon.png "QMKonnect.app/Contents/Resources/"

# Generate Info.plist
cat << EOF > "QMKonnect.app/Contents/Info.plist"
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>qmkonnect</string>
    <key>CFBundleIdentifier</key>
    <string>io.mulletware.qmkonnect</string>
    <key>CFBundleName</key>
    <string>QMKonnect</string>
    <key>CFBundleDisplayName</key>
    <string>QMKonnect</string>
    <key>LSBackgroundOnly</key>
    <true/>
    <key>CFBundleIconFile</key>
    <string>Icon.icns</string>
</dict>
</plist>
EOF

# Code sign (handles spaces in app name)
codesign --deep --force --sign - "QMKonnect.app"

echo "✅ App built: packaging/macos/QMKonnect.app"

# Create a DMG file containing the app bundle
DMG_NAME="QMKonnect.dmg"
VOLNAME="QMKonnect Installer"
TEMP_DIR=$(mktemp -d)

# Create a symbolic link to /Applications
ln -s "/Applications" "$TEMP_DIR/Applications"

# Copy the app bundle to the temporary directory
cp -R "QMKonnect.app" "$TEMP_DIR/"

# Create the DMG file with a compressed, read-only format (UDZO)
hdiutil create -volname "$VOLNAME" -srcfolder "$TEMP_DIR" -ov -format UDZO "$DMG_NAME"

# Clean up the temporary directory
rm -rf "$TEMP_DIR"

echo "✅ DMG built: $DMG_NAME"
