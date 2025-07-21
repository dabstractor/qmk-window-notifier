---
title: Home
layout: default
nav_order: 1
---

# QMKonnect
{: .fs-9 }

Cross-platform window activity notifier for QMK keyboards
{: .fs-6 .fw-300 }

[Get Started]({{ site.baseurl }}/installation){: .btn .btn-primary .fs-5 .mb-4 .mb-md-0 .mr-2 }
[View on GitHub](https://github.com/dabstractor/qmkonnect){: .btn .fs-5 .mb-4 .mb-md-0 }

---

## What is QMKonnect?

QMKonnect watches which window is active and tells your QMK keyboard about it. Your keyboard can then switch layers or run commands based on what app you're using.

### Key Features

- **Cross-Platform Support**: Works on Windows, Linux, and macOS
- **Real-time Detection**: Detects window focus changes
- **QMK Integration**: Talks to QMK keyboards via Raw HID
- **Low Resource Usage**: Runs in the background
- **Easy Configuration**: Simple config file

### How It Works

1. **Window Monitoring**: Watches for active window changes
2. **Data Processing**: Gets the app name and window title
3. **QMK Communication**: Sends that info to your QMK keyboard
4. **Layer Switching**: Your keyboard responds by switching layers or running macros



---

## Quick Start

1. **Download** the latest release for your platform
2. **Install** using the provided installer or package
3. **Configure** your keyboard settings
4. **Start** monitoring window changes automatically

[Installation Guide →]({{ site.baseurl }}/installation){: .btn .btn-outline }

---

## Part of the QMK Ecosystem

QMKonnect works alongside other tools in the QMK notification ecosystem:

- **[qmk-notifier](https://github.com/dabstractor/qmk-notifier)**: QMK firmware module for handling notifications
- **[qmk_notifier](https://github.com/dabstractor/qmk_notifier)**: Core library for Raw HID communication
- **QMKonnect**: This application for cross-platform window detection
