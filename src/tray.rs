#![cfg(not(all(target_os = "linux", feature = "hyprland")))]
use tao::{
    event::Event,
    event_loop::{ControlFlow, EventLoopBuilder},
};

use tray_icon::{
    menu::{AboutMetadata, Menu, MenuEvent, MenuItem, PredefinedMenuItem},
    TrayIconBuilder,
};

#[cfg(target_os = "macos")]
use std::env;





enum UserEvent {
    MenuEvent(MenuEvent),
}

pub fn setup_tray() {
    let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();

    let proxy = event_loop.create_proxy();
    

    
    MenuEvent::set_event_handler(Some(move |event| {
        let _ = proxy.send_event(UserEvent::MenuEvent(event));
    }));

    let tray_menu = Menu::new();

    let settings_i = MenuItem::new("Settings", true, None);
    let quit_i = MenuItem::new("Quit", true, None);
    let _ = tray_menu.append_items(&[
        &PredefinedMenuItem::about(
            None,
            Some(AboutMetadata {
                name: Some("QMK Window Notifier".to_string()),
                copyright: Some("Copyright Mulletware 2025".to_string()),
                ..Default::default()
            }),
        ),
        &PredefinedMenuItem::separator(),
        &settings_i,
        &PredefinedMenuItem::separator(),
        &quit_i,
    ]);

    let mut tray_icon = None;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(tao::event::StartCause::Init) => {
                let icon = {
                    #[cfg(target_os = "macos")]
                    {
                        load_icon_from_bundle().unwrap_or_else(|_| create_default_icon())
                    }
                    #[cfg(not(target_os = "macos"))]
                    {
                        // Try to find icon relative to executable first
                        let exe_path = std::env::current_exe().unwrap_or_default();
                        let exe_dir = exe_path.parent().unwrap_or_else(|| std::path::Path::new("."));
                        let icon_path = exe_dir.join("Icon.png");
                        
                        if icon_path.exists() {
                            load_icon(&icon_path).unwrap_or_else(|_| create_default_icon())
                        } else {
                            // Fallback to development path, then default icon
                            load_icon(std::path::Path::new("packaging/Icon.png"))
                                .unwrap_or_else(|_| create_default_icon())
                        }
                    }
                };

                tray_icon = Some(
                    TrayIconBuilder::new()
                        .with_menu(Box::new(tray_menu.clone()))
                        .with_tooltip("QMK Window Notifier")
                        .with_icon(icon)
                        .build()
                        .unwrap(),
                );



                // We have to request a redraw here to have the icon actually show up.
                // Tao only exposes a redraw method on the Window so we use core-foundation directly.
                #[cfg(target_os = "macos")]
                unsafe {
                    use objc2_core_foundation::{CFRunLoopGetMain, CFRunLoopWakeUp};

                    let rl = CFRunLoopGetMain().unwrap();
                    CFRunLoopWakeUp(&rl);
                }
            }

            Event::UserEvent(UserEvent::MenuEvent(event)) => {
                if event.id == settings_i.id() {
                    handle_settings_click();
                } else if event.id == quit_i.id() {
                    println!("Exited");
                    tray_icon.take();
                    *control_flow = ControlFlow::Exit;
                }
            }

            _ => {}
        }
    });
}

fn load_icon(path: &std::path::Path) -> Result<tray_icon::Icon, Box<dyn std::error::Error>> {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)?.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    Ok(tray_icon::Icon::from_rgba(icon_rgba, icon_width, icon_height)?)
}

fn create_default_icon() -> tray_icon::Icon {
    // Create a simple 16x16 default icon if no icon file is found
    let rgba = vec![255u8; 16 * 16 * 4]; // White 16x16 icon
    tray_icon::Icon::from_rgba(rgba, 16, 16).expect("Failed to create default icon")
}

fn handle_settings_click() {
    #[cfg(target_os = "windows")]
    {
        use crate::platforms;
        
        // Get or create the config directory
        match platforms::create_config_dir() {
            Ok(config_dir) => {
                let config_path = config_dir.join("config.toml");
                
                // Create default config if it doesn't exist
                if !config_path.exists() {
                    if let Err(e) = crate::core::create_default_config(&config_path) {
                        show_error_message(&format!("Failed to create configuration file: {}", e));
                        return;
                    }
                }
                
                // Show the settings dialog
                if let Err(e) = show_settings_dialog(&config_path) {
                    show_error_message(&format!("Failed to show settings dialog: {}", e));
                }
            }
            Err(e) => {
                show_error_message(&format!("Failed to access configuration directory: {}", e));
            }
        }
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        // For other platforms, show a simple message for now
        println!("Settings functionality not yet implemented for this platform");
    }
}



#[cfg(target_os = "windows")]
fn show_settings_dialog(config_path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    use windows::Win32::Foundation::HWND;
    use windows::Win32::UI::WindowsAndMessaging::{
        CreateWindowExW, RegisterClassW, ShowWindow, GetMessageW, 
        TranslateMessage, DispatchMessageW, LoadCursorW,
        WS_OVERLAPPED, WS_CAPTION, WS_SYSMENU, WS_VISIBLE,
        SW_SHOW, IDC_ARROW, MSG, WNDCLASSW
    };
    use windows::Win32::System::LibraryLoader::GetModuleHandleW;
    use std::sync::{Arc, Mutex};
    use std::ptr;

    // Load current configuration
    let current_config = match crate::core::parse_config(config_path) {
        Ok(config) => config,
        Err(_) => {
            crate::core::Config {
                vendor_id: 0xfeed,
                product_id: 0x0000,
            }
        }
    };

    // Shared state for the dialog
    let dialog_result = Arc::new(Mutex::new(None::<(u16, u16)>));
    let dialog_result_clone = dialog_result.clone();

    unsafe {
        let h_instance = GetModuleHandleW(None)?;
        let class_name = to_wide_string("QMKSettingsDialog");
        let window_title = to_wide_string("QMK Settings");

        // Load application icon
        let app_icon = load_app_icon();

        // Register window class
        let wnd_class = WNDCLASSW {
            lpfnWndProc: Some(settings_dialog_proc),
            hInstance: h_instance.into(),
            lpszClassName: windows::core::PCWSTR(class_name.as_ptr()),
            hCursor: LoadCursorW(None, IDC_ARROW)?,
            hbrBackground: windows::Win32::Graphics::Gdi::HBRUSH((15 + 1) as isize), // COLOR_3DFACE + 1
            hIcon: app_icon,
            ..Default::default()
        };

        RegisterClassW(&wnd_class);

        // Get screen dimensions to center the dialog
        let screen_width = windows::Win32::UI::WindowsAndMessaging::GetSystemMetrics(windows::Win32::UI::WindowsAndMessaging::SM_CXSCREEN);
        let screen_height = windows::Win32::UI::WindowsAndMessaging::GetSystemMetrics(windows::Win32::UI::WindowsAndMessaging::SM_CYSCREEN);
        
        let dialog_width = 400;
        let dialog_height = 200;
        let x = (screen_width - dialog_width) / 2;
        let y = (screen_height - dialog_height) / 2;

        // Create the dialog window (centered on screen)
        let hwnd = CreateWindowExW(
            windows::Win32::UI::WindowsAndMessaging::WINDOW_EX_STYLE(0),
            windows::core::PCWSTR(class_name.as_ptr()),
            windows::core::PCWSTR(window_title.as_ptr()),
            WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_VISIBLE,
            x, y, dialog_width, dialog_height,
            HWND(0), None, h_instance, Some(ptr::null())
        );

        if hwnd.0 == 0 {
            return Err("Failed to create settings dialog window".into());
        }

        // Create controls
        create_dialog_controls(hwnd, h_instance.into(), &current_config)?;

        // Set the window icon directly using a standard Windows icon
        // This will show the blue information icon, which is better than no icon
        let icon = windows::Win32::UI::WindowsAndMessaging::LoadIconW(
            None, 
            windows::Win32::UI::WindowsAndMessaging::IDI_INFORMATION
        ).unwrap_or(windows::Win32::UI::WindowsAndMessaging::HICON(0));
        
        if icon.0 != 0 {
            windows::Win32::UI::WindowsAndMessaging::SendMessageW(
                hwnd,
                windows::Win32::UI::WindowsAndMessaging::WM_SETICON,
                windows::Win32::Foundation::WPARAM(windows::Win32::UI::WindowsAndMessaging::ICON_SMALL as usize),
                windows::Win32::Foundation::LPARAM(icon.0 as isize)
            );
            windows::Win32::UI::WindowsAndMessaging::SendMessageW(
                hwnd,
                windows::Win32::UI::WindowsAndMessaging::WM_SETICON,
                windows::Win32::Foundation::WPARAM(windows::Win32::UI::WindowsAndMessaging::ICON_BIG as usize),
                windows::Win32::Foundation::LPARAM(icon.0 as isize)
            );
        }

        ShowWindow(hwnd, SW_SHOW);

        // Store dialog result pointer in window user data
        windows::Win32::UI::WindowsAndMessaging::SetWindowLongPtrW(
            hwnd, 
            windows::Win32::UI::WindowsAndMessaging::GWLP_USERDATA, 
            Arc::into_raw(dialog_result_clone) as isize
        );

        // Message loop
        let mut msg = MSG::default();
        while GetMessageW(&mut msg, None, 0, 0).as_bool() {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }

        // Get the result
        let result = dialog_result.lock().unwrap().clone();
        
        if let Some((vendor_id, product_id)) = result {
            // Save to file
            let config_content = format!(
                "# QMK Window Notifier Configuration\n\n# Your QMK keyboard's vendor ID (in hex)\nvendor_id = 0x{:04x}\n\n# Your QMK keyboard's product ID (in hex)\nproduct_id = 0x{:04x}\n\n# Add any other configuration options here\n",
                vendor_id, product_id
            );

            std::fs::write(config_path, config_content)?;

            // Configuration saved successfully - no success dialog needed
            // The QMK connection is established fresh for each notification,
            // so no restart is required for the changes to take effect
        }
    }

    Ok(())
}

#[cfg(target_os = "windows")]
fn create_dialog_controls(hwnd: windows::Win32::Foundation::HWND, h_instance: windows::Win32::Foundation::HINSTANCE, config: &crate::core::Config) -> Result<(), Box<dyn std::error::Error>> {
    use windows::Win32::UI::WindowsAndMessaging::{CreateWindowExW, SetDlgItemTextW, WS_CHILD, WS_VISIBLE, WS_TABSTOP};
    use windows::Win32::UI::Controls::{WC_STATICW, WC_EDITW, WC_BUTTONW};
    use std::ptr;

    unsafe {
        // Vendor ID label
        CreateWindowExW(
            windows::Win32::UI::WindowsAndMessaging::WINDOW_EX_STYLE(0),
            WC_STATICW,
            windows::core::w!("Vendor ID (hex):"),
            WS_CHILD | WS_VISIBLE,
            20, 30, 120, 20,
            hwnd, None, h_instance, Some(ptr::null())
        );

        // Vendor ID text box
        CreateWindowExW(
            windows::Win32::UI::WindowsAndMessaging::WS_EX_CLIENTEDGE,
            WC_EDITW,
            windows::core::PCWSTR::null(),
            WS_CHILD | WS_VISIBLE | WS_TABSTOP,
            150, 28, 100, 24,
            hwnd, windows::Win32::UI::WindowsAndMessaging::HMENU(1001), h_instance, Some(ptr::null())
        );

        // Product ID label
        CreateWindowExW(
            windows::Win32::UI::WindowsAndMessaging::WINDOW_EX_STYLE(0),
            WC_STATICW,
            windows::core::w!("Product ID (hex):"),
            WS_CHILD | WS_VISIBLE,
            20, 70, 120, 20,
            hwnd, None, h_instance, Some(ptr::null())
        );

        // Product ID text box
        CreateWindowExW(
            windows::Win32::UI::WindowsAndMessaging::WS_EX_CLIENTEDGE,
            WC_EDITW,
            windows::core::PCWSTR::null(),
            WS_CHILD | WS_VISIBLE | WS_TABSTOP,
            150, 68, 100, 24,
            hwnd, windows::Win32::UI::WindowsAndMessaging::HMENU(1002), h_instance, Some(ptr::null())
        );

        // OK button
        CreateWindowExW(
            windows::Win32::UI::WindowsAndMessaging::WINDOW_EX_STYLE(0),
            WC_BUTTONW,
            windows::core::w!("OK"),
            WS_CHILD | WS_VISIBLE | WS_TABSTOP,
            150, 110, 75, 30,
            hwnd, windows::Win32::UI::WindowsAndMessaging::HMENU(1003), h_instance, Some(ptr::null())
        );

        // Cancel button
        CreateWindowExW(
            windows::Win32::UI::WindowsAndMessaging::WINDOW_EX_STYLE(0),
            WC_BUTTONW,
            windows::core::w!("Cancel"),
            WS_CHILD | WS_VISIBLE | WS_TABSTOP,
            240, 110, 75, 30,
            hwnd, windows::Win32::UI::WindowsAndMessaging::HMENU(1004), h_instance, Some(ptr::null())
        );

        // Set initial values (without "0x" prefix - user only sees hex digits)
        let vendor_text = to_wide_string(&format!("{:04x}", config.vendor_id));
        let product_text = to_wide_string(&format!("{:04x}", config.product_id));
        
        let _ = SetDlgItemTextW(hwnd, 1001, windows::core::PCWSTR(vendor_text.as_ptr()));
        let _ = SetDlgItemTextW(hwnd, 1002, windows::core::PCWSTR(product_text.as_ptr()));
    }

    Ok(())
}

#[cfg(target_os = "windows")]
unsafe extern "system" fn settings_dialog_proc(
    hwnd: windows::Win32::Foundation::HWND,
    msg: u32,
    wparam: windows::Win32::Foundation::WPARAM,
    lparam: windows::Win32::Foundation::LPARAM,
) -> windows::Win32::Foundation::LRESULT {
    use windows::Win32::UI::WindowsAndMessaging::{
        DefWindowProcW, PostQuitMessage, DestroyWindow, GetDlgItemTextW, MessageBoxW,
        WM_COMMAND, WM_CLOSE, WM_DESTROY, MB_OK, MB_ICONERROR, GWLP_USERDATA, GetWindowLongPtrW
    };
    use std::sync::{Arc, Mutex};

    match msg {
        WM_COMMAND => {
            let control_id = (wparam.0 & 0xFFFF) as u32;
            match control_id {
                1003 => { // OK button
                    // Get text from controls
                    let mut vendor_buffer = [0u16; 256];
                    let mut product_buffer = [0u16; 256];
                    
                    GetDlgItemTextW(hwnd, 1001, &mut vendor_buffer);
                    GetDlgItemTextW(hwnd, 1002, &mut product_buffer);

                    // Convert to strings
                    let vendor_str = String::from_utf16_lossy(&vendor_buffer).trim_end_matches('\0').to_string();
                    let product_str = String::from_utf16_lossy(&product_buffer).trim_end_matches('\0').to_string();

                    // Parse hex values
                    match (parse_hex_value(&vendor_str), parse_hex_value(&product_str)) {
                        (Ok(vendor_id), Ok(product_id)) => {
                            // Store result in shared state
                            let user_data = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *const Mutex<Option<(u16, u16)>>;
                            if !user_data.is_null() {
                                let result_arc = Arc::from_raw(user_data);
                                if let Ok(mut result) = result_arc.lock() {
                                    *result = Some((vendor_id, product_id));
                                }
                                // Don't drop the Arc, we need it to persist
                                std::mem::forget(result_arc);
                            }
                            let _ = DestroyWindow(hwnd);
                        }
                        (Err(e), _) | (_, Err(e)) => {
                            let error_msg = to_wide_string(&format!("Invalid input: {}", e));
                            let _ = MessageBoxW(hwnd, windows::core::PCWSTR(error_msg.as_ptr()), 
                                      windows::core::w!("Error"), MB_OK | MB_ICONERROR);
                        }
                    }
                }
                1004 => { // Cancel button
                    let _ = DestroyWindow(hwnd);
                }
                _ => {}
            }
        }
        WM_CLOSE => {
            let _ = DestroyWindow(hwnd);
        }
        WM_DESTROY => {
            PostQuitMessage(0);
        }
        _ => return DefWindowProcW(hwnd, msg, wparam, lparam),
    }
    windows::Win32::Foundation::LRESULT(0)
}



#[cfg(target_os = "windows")]
fn parse_hex_value(input: &str) -> Result<u16, Box<dyn std::error::Error>> {
    let trimmed = input.trim().to_lowercase(); // Convert to lowercase for case-insensitive parsing
    let hex_str = if trimmed.starts_with("0x") {
        &trimmed[2..]
    } else {
        &trimmed
    };
    
    u16::from_str_radix(hex_str, 16).map_err(|e| format!("Invalid hex value '{}': {}", input, e).into())
}



#[cfg(target_os = "windows")]
fn to_wide_string(s: &str) -> Vec<u16> {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    OsStr::new(s).encode_wide().chain(std::iter::once(0)).collect()
}

#[cfg(target_os = "windows")]
fn load_app_icon() -> windows::Win32::UI::WindowsAndMessaging::HICON {
    use windows::Win32::UI::WindowsAndMessaging::{LoadIconW, IDI_INFORMATION, LoadImageW, IMAGE_ICON, LR_LOADFROMFILE, LR_DEFAULTSIZE};
    use std::path::Path;
    
    unsafe {
        // Try to find the ICO file (Windows native format)
        let exe_path = std::env::current_exe().unwrap_or_default();
        let exe_dir = exe_path.parent().unwrap_or_else(|| Path::new("."));
        
        // Try these paths in order
        let icon_paths = [
            exe_dir.join("Icon.ico"),
            Path::new("packaging/Icon.ico").to_path_buf(),
        ];
        
        for icon_path in &icon_paths {
            if icon_path.exists() {
                let icon_path_wide = to_wide_string(&icon_path.to_string_lossy());
                let hicon = LoadImageW(
                    None,
                    windows::core::PCWSTR(icon_path_wide.as_ptr()),
                    IMAGE_ICON,
                    0, 0, // Use default size
                    LR_DEFAULTSIZE | LR_LOADFROMFILE
                );
                
                if let Ok(icon) = hicon {
                    if icon.0 != 0 {
                        return windows::Win32::UI::WindowsAndMessaging::HICON(icon.0);
                    }
                }
            }
        }
        
        // Fallback to standard Windows information icon
        LoadIconW(None, IDI_INFORMATION).unwrap_or(windows::Win32::UI::WindowsAndMessaging::HICON(0))
    }
}

#[cfg(target_os = "windows")]
fn show_error_message(message: &str) {
    use windows::Win32::UI::WindowsAndMessaging::{MessageBoxW, MB_OK, MB_ICONERROR};
    use windows::Win32::Foundation::HWND;
    
    unsafe {
        let error_msg = to_wide_string(message);
        let title = to_wide_string("QMK Window Notifier - Error");
        MessageBoxW(
            HWND(0), 
            windows::core::PCWSTR(error_msg.as_ptr()), 
            windows::core::PCWSTR(title.as_ptr()), 
            MB_OK | MB_ICONERROR
        );
    }
    
    // Also log to console/event log
    eprintln!("Settings error: {}", message);
}




#[cfg(target_os = "macos")]
fn load_icon_from_bundle() -> Result<tray_icon::Icon, Box<dyn std::error::Error>> {
    let bundle = core_foundation::bundle::CFBundle::main_bundle();
    let bundle_path = bundle.executable_url().unwrap().to_path().unwrap();
    let resources_path = bundle_path.parent().unwrap().join("../Resources");
    let icon_path = resources_path.join("Icon.png");

    load_icon(&icon_path)
}