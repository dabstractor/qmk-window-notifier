# Windows Service Implementation Prompt

## Context
I have a working QMK Window Notifier Rust application that currently runs as a console application. The application successfully:
- Detects window focus changes on Windows
- Sends notifications to QMK keyboards
- Shows a system tray icon
- Has a working MSI installer

## Problem
When the application runs, it shows a terminal/console window that pops up and then disappears. This creates a poor user experience for what should be a background system service.

## Goal
Convert the application to run as a proper Windows service that:
- Runs silently in the background without any console windows
- Starts automatically on system boot
- Maintains all current functionality (window detection, QMK notifications, system tray)
- Can be managed through Windows Services panel
- Is properly installed/uninstalled via the MSI installer

## Current Architecture
The application is structured with:
- `src/main.rs` - Main entry point and application logic
- `src/platforms/windows.rs` - Windows-specific window monitoring
- `src/tray.rs` - System tray implementation
- `src/core/` - Core notification and configuration logic
- `packaging/windows/` - MSI installer configuration

## Request
Please review the current codebase and implement the Windows service functionality according to the requirements in `.kiro/specs/windows-service/requirements.md`. Focus on:

1. Converting the console application to a Windows service
2. Eliminating all console window popups
3. Maintaining existing functionality
4. Updating the installer to properly register the service
5. Implementing proper logging via Windows Event Log

The service should be production-ready and provide a seamless user experience where the application runs invisibly in the background after installation.