#!/bin/bash
# Ensure XDG_RUNTIME_DIR is set (adjust the default if necessary)
export XDG_RUNTIME_DIR=${XDG_RUNTIME_DIR:-/run/user/$(id -u)}

# Debug logging
echo "Starting QMK Window Notifier wrapper script" > /tmp/qmk-window-notifier-wrapper.log
echo "XDG_RUNTIME_DIR=$XDG_RUNTIME_DIR" >> /tmp/qmk-window-notifier-wrapper.log
echo "PATH=$PATH" >> /tmp/qmk-window-notifier-wrapper.log

# Find the first hyprland socket in the runtime directory
socket=$(find "$XDG_RUNTIME_DIR" -maxdepth 1 -type s -name 'hyprland-*.socket' | head -n 1)

if [ -z "$socket" ]; then
  echo "Error: Hyprland socket not found in $XDG_RUNTIME_DIR" | tee -a /tmp/qmk-window-notifier-wrapper.log
  exit 1
fi

# Extract the instance signature from the socket filename
# The expected filename format is hyprland-<signature>.socket
sig=$(basename "$socket" | sed 's/^hyprland-\(.*\)\.socket$/\1/')
export HYPRLAND_INSTANCE_SIGNATURE="$sig"

echo "Found Hyprland socket: $socket" >> /tmp/qmk-window-notifier-wrapper.log
echo "Setting HYPRLAND_INSTANCE_SIGNATURE=$sig" >> /tmp/qmk-window-notifier-wrapper.log

# Launch the application with the correct environment
echo "Launching qmk-window-notifier with args: $@" >> /tmp/qmk-window-notifier-wrapper.log
exec /usr/bin/qmk-window-notifier "$@"
