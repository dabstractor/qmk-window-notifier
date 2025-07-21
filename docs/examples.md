---
title: Examples
layout: default
nav_order: 8
---

# QMK Integration Examples

Real-world QMK firmware examples showing the correct way to integrate with QMKonnect.

## QMKonnect Configuration

QMKonnect only requires vendor_id and product_id configuration:

### Linux Configuration File
```toml
# ~/.config/qmk-notifier/config.toml
vendor_id = 0xfeed
product_id = 0x0000
```

### Windows & macOS Configuration
Use the GUI settings accessible through the system tray:
1. Right-click the QMKonnect system tray icon
2. Select "Settings"
3. Enter your vendor_id and product_id (without the "0x" prefix)
4. Click OK

## QMK Firmware Integration

All functionality is implemented in your QMK firmware using the correct framework patterns:

---

## Next Steps

- [Learn about troubleshooting]({{ site.baseurl }}/troubleshooting)
- [Set up QMK integration]({{ site.baseurl }}/qmk-integration)
- [Contribute to the project](https://github.com/dabstractor/qmkonnect)
